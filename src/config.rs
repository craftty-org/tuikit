//! Generic TOML configuration helpers.

use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

/// Load a TOML config file into `T`, returning `T::default()` if the file is missing or invalid.
pub fn load_toml_or_default<T>(path: impl AsRef<Path>) -> T
where
    T: DeserializeOwned + Default,
{
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| toml::from_str::<T>(&s).ok())
        .unwrap_or_default()
}

/// Load a TOML config file, returning `Ok(None)` if it does not exist.
pub fn load_toml_optional<T>(path: impl AsRef<Path>) -> Result<Option<T>, ConfigIoError>
where
    T: DeserializeOwned,
{
    let path = path.as_ref();
    match std::fs::read_to_string(path) {
        Ok(s) => toml::from_str::<T>(&s)
            .map(Some)
            .map_err(|e| ConfigIoError::Parse(e.to_string())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(ConfigIoError::Io(e)),
    }
}

/// Save a value as pretty TOML, creating parent directories as needed.
pub fn save_toml_pretty<T>(path: impl AsRef<Path>, value: &T) -> Result<(), ConfigIoError>
where
    T: Serialize,
{
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(ConfigIoError::Io)?;
    }
    let body = toml::to_string_pretty(value).map_err(|e| ConfigIoError::Parse(e.to_string()))?;
    std::fs::write(path, body).map_err(ConfigIoError::Io)
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigIoError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("toml error: {0}")]
    Parse(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
    struct Demo {
        name: String,
    }

    #[test]
    fn roundtrip_toml() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("demo.toml");
        let v = Demo {
            name: "x".into(),
        };
        save_toml_pretty(&path, &v).unwrap();
        let loaded: Option<Demo> = load_toml_optional(&path).unwrap();
        assert_eq!(loaded, Some(v));
    }
}
