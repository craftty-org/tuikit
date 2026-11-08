//! Application path helpers (XDG-oriented, with practical fallbacks).

use std::path::{Path, PathBuf};

/// Resolve the user config directory for `app_name`.
pub fn config_dir(app_name: &str) -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(app_name)
}

/// Resolve the user cache directory for `app_name`.
pub fn cache_dir(app_name: &str) -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(app_name)
}

/// Resolve a runtime/state directory for transient files (sockets, pid files).
///
/// Resolution order:
/// 1. `$XDG_RUNTIME_DIR/<app_name>`
/// 2. `/tmp/<app_name>-$USER`
/// 3. `$TMPDIR/<app_name>`
/// 4. `$PREFIX/tmp/<app_name>` (Termux-friendly)
/// 5. `$HOME/.<app_name>/run`
/// 6. `std::env::temp_dir()/ <app_name>`
pub fn runtime_dir(app_name: &str) -> PathBuf {
    if let Ok(runtime) = std::env::var("XDG_RUNTIME_DIR") {
        let p = PathBuf::from(runtime).join(app_name);
        if ensure_parent_exists(&p) {
            let _ = std::fs::create_dir_all(&p);
            return p;
        }
    }
    if let Ok(user) = std::env::var("USER") {
        let p = PathBuf::from("/tmp").join(format!("{app_name}-{user}"));
        if ensure_parent_exists(&p) {
            let _ = std::fs::create_dir_all(&p);
            return p;
        }
    }
    if let Ok(tmpdir) = std::env::var("TMPDIR") {
        let p = PathBuf::from(tmpdir).join(app_name);
        if ensure_parent_exists(&p) {
            let _ = std::fs::create_dir_all(&p);
            return p;
        }
    }
    if is_termux() {
        if let Ok(prefix) = std::env::var("PREFIX") {
            let p = PathBuf::from(prefix).join("tmp").join(app_name);
            if ensure_parent_exists(&p) {
                let _ = std::fs::create_dir_all(&p);
                return p;
            }
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        let p = PathBuf::from(home).join(format!(".{app_name}")).join("run");
        let _ = std::fs::create_dir_all(&p);
        return p;
    }
    let p = std::env::temp_dir().join(app_name);
    let _ = std::fs::create_dir_all(&p);
    p
}

/// Resolve a file path under the runtime directory.
pub fn runtime_file(app_name: &str, file_name: impl AsRef<Path>) -> PathBuf {
    runtime_dir(app_name).join(file_name)
}

/// Detect whether we appear to be running inside Termux.
pub fn is_termux() -> bool {
    std::env::var("PREFIX").is_ok() || std::env::var("TERMUX_VERSION").is_ok()
}

fn ensure_parent_exists(path: &Path) -> bool {
    path.parent().is_some_and(|d| d.exists())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_app_name() {
        let p = config_dir("myapp");
        assert!(p.ends_with("myapp"));
    }
}
