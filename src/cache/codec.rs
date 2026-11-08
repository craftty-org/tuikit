//! Compressed serde blob helpers.

use serde::{de::DeserializeOwned, Serialize};
use std::io::Cursor;

#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("compression error: {0}")]
    Compress(std::io::Error),
    #[error("decompression error: {0}")]
    Decompress(String),
}

/// Serialize a value as JSON and compress with zstd.
pub fn compress_json<T: Serialize>(value: &T, level: i32) -> Result<Vec<u8>, CodecError> {
    let json = serde_json::to_vec(value)?;
    zstd::stream::encode_all(Cursor::new(json), level).map_err(CodecError::Compress)
}

/// Decompress zstd bytes and deserialize JSON into `T`.
pub fn decompress_json<T: DeserializeOwned>(blob: &[u8], max_bytes: usize) -> Result<T, CodecError> {
    let raw = zstd::bulk::decompress(blob, max_bytes)
        .map_err(|e| CodecError::Decompress(e.to_string()))?;
    Ok(serde_json::from_slice(&raw)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct V { s: String }

    #[test]
    fn roundtrip() {
        let v = V { s: "hello".repeat(50) };
        let b = compress_json(&v, 3).unwrap();
        let v2: V = decompress_json(&b, 1024 * 1024).unwrap();
        assert_eq!(v, v2);
    }
}
