//! Generic HTTP helpers and connectivity state.

pub mod client;
pub mod connectivity;
pub mod error;

pub use client::{HttpClient, HttpClientConfig};
pub use connectivity::{Connectivity, ConnectivityReading, Verdict, FRESHNESS, OFFLINE_STREAK, ONLINE_STREAK};
pub use error::HttpError;
