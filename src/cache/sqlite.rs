//! Small SQLite helper utilities.

use rusqlite::Connection;
use std::path::Path;
use std::sync::Mutex;

/// Open a SQLite connection and apply common TUI/cache pragmas (WAL, normal sync, foreign keys).
pub fn open_with_pragmas(path: impl AsRef<Path>) -> Result<Connection, rusqlite::Error> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA synchronous=NORMAL;
         PRAGMA foreign_keys=ON;
         PRAGMA temp_store=MEMORY;",
    )?;
    Ok(conn)
}

/// A mutex-wrapped connection for simple shared access in blocking contexts.
pub struct SharedConnection {
    inner: Mutex<Connection>,
}

impl SharedConnection {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            inner: Mutex::new(open_with_pragmas(path)?),
        })
    }

    pub fn with_lock<T>(&self, f: impl FnOnce(&Connection) -> T) -> T {
        let conn = self.inner.lock().unwrap_or_else(|p| p.into_inner());
        f(&conn)
    }
}
