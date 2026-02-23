pub mod episode;
pub mod podcast;

use std::path::Path;
use std::sync::{LazyLock, Mutex};

use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

use crate::error::AppError;

/// Tauri State として管理する DB 接続ラッパー
pub struct DbState(pub Mutex<Connection>);

static MIGRATIONS: LazyLock<Migrations<'static>> = LazyLock::new(|| {
    Migrations::new(vec![
        M::up(include_str!("../../migrations/001_initial.sql")),
        M::up(include_str!("../../migrations/002_drop_duration.sql")),
    ])
});

/// DB を初期化し、マイグレーションを実行して接続を返す
pub fn init_db(app_data_dir: &Path) -> Result<Connection, AppError> {
    let db_path = app_data_dir.join("podcast-downloader.db");
    let mut conn = Connection::open(db_path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    MIGRATIONS
        .to_latest(&mut conn)
        .map_err(|e| AppError::Other(e.to_string()))?;
    Ok(conn)
}

#[cfg(test)]
pub fn init_test_db() -> Result<Connection, AppError> {
    let mut conn = Connection::open_in_memory()?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    MIGRATIONS
        .to_latest(&mut conn)
        .map_err(|e| AppError::Other(e.to_string()))?;
    Ok(conn)
}
