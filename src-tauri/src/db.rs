use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;

pub struct DbConn(pub Mutex<Connection>);

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: String,
    pub username: String,
    pub url: String,
    pub category_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterPasswordInfo {
    pub salt: Vec<u8>,
    pub wrapped_dek: Vec<u8>,
    pub dek_nonce: Vec<u8>,
}

pub fn db_path(app: &AppHandle) -> std::path::PathBuf {
    let dir = app
        .path()
        .app_config_dir()
        .expect("failed to get app config dir");
    fs::create_dir_all(&dir).ok();
    dir.join("passnager.db")
}

pub fn init_db(app: &AppHandle) -> SqlResult<()> {
    let path = db_path(app);
    let conn = Connection::open(&path)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS master_password (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            salt BLOB NOT NULL,
            wrapped_dek BLOB NOT NULL,
            dek_nonce BLOB NOT NULL
        );

        CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            icon TEXT,
            color TEXT
        );

        CREATE TABLE IF NOT EXISTS passwords (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            encrypted_password BLOB NOT NULL,
            nonce BLOB NOT NULL,
            url TEXT NOT NULL,
            category_id TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL
        );

        CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            category_id TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL
        );

        INSERT OR IGNORE INTO settings (key, value) VALUES ('theme', 'dark');
        ",
    )?;

    app.manage(DbConn(Mutex::new(conn)));
    Ok(())
}
