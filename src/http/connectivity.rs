//! Connectivity verdict and hysteresis, independent of any specific endpoint.

use serde::{Deserialize, Serialize};

/// Whether the network is reachable, and how confident we are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Connectivity {
    /// Never probed.
    #[default]
    Unknown,
    /// A probe succeeded within the freshness window.
    Online,
    /// Consecutive probes have failed.
    Offline,
    /// The last probe answered, but not recently enough to remain trusted.
    Stale,
}

/// How long a reading remains fresh.
pub const FRESHNESS: std::time::Duration = std::time::Duration::from_secs(60);
/// Successes required before flipping to online.
pub const ONLINE_STREAK: u32 = 2;
/// Failures required before flipping to offline.
pub const OFFLINE_STREAK: u32 = 2;

/// A connectivity verdict and when it was observed.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct ConnectivityReading {
    pub state: Connectivity,
    /// Unix seconds when the probe ran. Zero means never.
    pub checked_at: i64,
}

impl ConnectivityReading {
    /// Return the effective state at `now_secs`, demoting stale readings.
    pub fn effective_state(&self, now_secs: i64) -> Connectivity {
        if self.checked_at == 0 {
            return Connectivity::Unknown;
        }
        if now_secs - self.checked_at > FRESHNESS.as_secs() as i64 {
            return Connectivity::Stale;
        }
        self.state
    }

    /// Compatibility alias for [`Self::effective_state`].
    pub fn now(&self, now_secs: i64) -> Connectivity {
        self.effective_state(now_secs)
    }
}

/// A verdict with hysteresis counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verdict {
    pub state: Connectivity,
    pub on_streak: u32,
    pub off_streak: u32,
}

impl Verdict {
    pub fn settled(state: Connectivity) -> Self {
        Self {
            state,
            on_streak: 0,
            off_streak: 0,
        }
    }

    /// Fold in one probe result. Returns `Some(new_state)` only when the visible state changes.
    pub fn observe(&mut self, reachable: bool) -> Option<Connectivity> {
        if reachable {
            self.on_streak += 1;
            self.off_streak = 0;
        } else {
            self.off_streak += 1;
            self.on_streak = 0;
        }
        let next = if self.on_streak >= ONLINE_STREAK {
            Connectivity::Online
        } else if self.off_streak >= OFFLINE_STREAK {
            Connectivity::Offline
        } else {
            return None;
        };
        if next == self.state {
            return None;
        }
        self.state = next;
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_streaks() {
        let mut v = Verdict::settled(Connectivity::Unknown);
        assert_eq!(v.observe(true), None);
        assert_eq!(v.observe(true), Some(Connectivity::Online));
        assert_eq!(v.observe(false), None);
        assert_eq!(v.observe(false), Some(Connectivity::Offline));
    }

    #[test]
    fn ages_to_stale() {
        let now = 1_000_000i64;
        let r = ConnectivityReading {
            state: Connectivity::Online,
            checked_at: now,
        };
        assert_eq!(r.effective_state(now), Connectivity::Online);
        assert_eq!(
            r.effective_state(now + FRESHNESS.as_secs() as i64 + 1),
            Connectivity::Stale
        );
    }
}
