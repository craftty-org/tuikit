//! Generic HTTP client helpers built on `reqwest`.

use std::time::Duration;

use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, USER_AGENT};
use serde::de::DeserializeOwned;

use super::error::HttpError;

/// Configuration for [`HttpClient`].
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub user_agent: String,
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub retries: u32,
    pub redirect_limit: usize,
    pub default_accept: Option<String>,
    pub extra_headers: Vec<(String, String)>,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            user_agent: "tuikit-http/0.1".to_string(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            retries: 2,
            redirect_limit: 8,
            default_accept: Some("application/json, */*;q=0.5".to_string()),
            extra_headers: Vec::new(),
        }
    }
}

/// A reusable HTTP client with retry and size-limited helpers.
#[derive(Clone)]
pub struct HttpClient {
    client: reqwest::Client,
    retries: u32,
}

impl HttpClient {
    pub fn new(config: HttpClientConfig) -> Result<Self, HttpError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(config.user_agent.trim())
                .map_err(|_| HttpError::Network("invalid user agent".into()))?,
        );
        if let Some(accept) = config.default_accept.as_deref() {
            headers.insert(
                ACCEPT,
                HeaderValue::from_str(accept).map_err(|_| HttpError::Network("invalid accept header".into()))?,
            );
        }
        for (k, v) in config.extra_headers {
            let name = HeaderName::from_bytes(k.as_bytes())
                .map_err(|_| HttpError::Network(format!("invalid header name: {k}")))?;
            let value = HeaderValue::from_str(&v)
                .map_err(|_| HttpError::Network(format!("invalid header value for {k}")))?;
            headers.insert(name, value);
        }
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .connect_timeout(config.connect_timeout)
            .redirect(reqwest::redirect::Policy::limited(config.redirect_limit))
            .build()
            .map_err(HttpError::from_reqwest)?;
        Ok(Self {
            client,
            retries: config.retries.min(8),
        })
    }

    pub fn inner(&self) -> &reqwest::Client {
        &self.client
    }

    /// Perform a GET with basic retry handling for transient failures and 429/404 mapping.
    pub async fn get(&self, url: &str) -> Result<reqwest::Response, HttpError> {
        let mut attempt = 0u32;
        loop {
            let result = self.client.get(url).send().await;
            match result {
                Ok(response) => {
                    let status = response.status();
                    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        return Err(HttpError::RateLimited);
                    }
                    if status == reqwest::StatusCode::NOT_FOUND {
                        return Err(HttpError::NotFound);
                    }
                    if status.is_server_error() && attempt < self.retries {
                        attempt += 1;
                        tokio::time::sleep(backoff(attempt)).await;
                        continue;
                    }
                    if !status.is_success() {
                        return Err(HttpError::Status {
                            status: status.as_u16(),
                        });
                    }
                    return Ok(response);
                }
                Err(error) => {
                    let transient = error.is_timeout() || error.is_connect();
                    if transient && attempt < self.retries {
                        attempt += 1;
                        tokio::time::sleep(backoff(attempt)).await;
                        continue;
                    }
                    return Err(HttpError::from_reqwest(error));
                }
            }
        }
    }

    /// Fetch and deserialize JSON with a maximum byte limit.
    pub async fn get_json<T: DeserializeOwned>(&self, url: &str, max_bytes: usize) -> Result<T, HttpError> {
        let response = self.get(url).await?;
        if let Some(length) = response.content_length() {
            if length as usize > max_bytes {
                return Err(HttpError::TooLarge);
            }
        }
        let bytes = response.bytes().await.map_err(HttpError::from_reqwest)?;
        if bytes.len() > max_bytes {
            return Err(HttpError::TooLarge);
        }
        serde_json::from_slice(&bytes).map_err(|e| HttpError::InvalidBody(e.to_string()))
    }

    /// Download a response body as bytes with a maximum byte limit.
    pub async fn download_bytes(&self, url: &str, max_bytes: u64) -> Result<Vec<u8>, HttpError> {
        let response = self.get(url).await?;
        if let Some(length) = response.content_length() {
            if length > max_bytes {
                return Err(HttpError::TooLarge);
            }
        }
        let mut stream = response.bytes_stream();
        let mut data = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(HttpError::from_reqwest)?;
            if data.len() + chunk.len() > max_bytes as usize {
                return Err(HttpError::TooLarge);
            }
            data.extend_from_slice(&chunk);
        }
        if data.is_empty() {
            return Err(HttpError::InvalidBody("empty response".into()));
        }
        Ok(data)
    }
}

fn backoff(attempt: u32) -> Duration {
    Duration::from_millis(200 * u64::from(attempt.max(1)))
}
