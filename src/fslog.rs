//! Simple append-only file logging helpers.

use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// A minimal thread-safe file logger.
#[derive(Debug)]
pub struct FileLog {
    path: PathBuf,
    file: Mutex<std::fs::File>,
}

impl FileLog {
    /// Open (or create) an append-only log file, creating parent directories as needed.
    pub fn open(path: impl Into<PathBuf>) -> std::io::Result<Self> {
        let path = path.into();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        Ok(Self {
            path,
            file: Mutex::new(file),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Append a timestamped line.
    pub fn log(&self, msg: &str) {
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        let line = format!("[{ts}] {msg}\n");
        if let Ok(mut f) = self.file.lock() {
            let _ = f.write_all(line.as_bytes());
            let _ = f.flush();
        }
    }
}

/// Redirect stderr to `path` (Unix only). Returns the duplicated original stderr fd on success.
#[cfg(unix)]
pub fn redirect_stderr_to_file(path: impl AsRef<Path>) -> std::io::Result<std::os::unix::io::RawFd> {
    use std::os::unix::io::AsRawFd;
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new().create(true).append(true).open(path)?;
    let original_fd = unsafe { libc::dup(libc::STDERR_FILENO) };
    if original_fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    let raw = file.as_raw_fd();
    let rc = unsafe { libc::dup2(raw, libc::STDERR_FILENO) };
    if rc < 0 {
        unsafe {
            libc::close(original_fd);
        }
        return Err(std::io::Error::last_os_error());
    }
    Ok(original_fd)
}

#[cfg(not(unix))]
pub fn redirect_stderr_to_file(_path: impl AsRef<Path>) -> std::io::Result<i32> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "stderr redirection helper is only implemented for Unix",
    ))
}
