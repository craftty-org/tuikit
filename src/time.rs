use std::time::{SystemTime, UNIX_EPOCH};

/// Current Unix timestamp in seconds. Returns `0` if the system clock is before the epoch.
pub fn unix_now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// Current Unix timestamp in milliseconds. Returns `0` if the system clock is before the epoch.
pub fn unix_now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
