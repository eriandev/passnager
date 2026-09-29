use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;
use zeroize::Zeroizing;

pub struct DbConn(pub Mutex<Connection>);

pub struct Session(pub Mutex<Option<Zeroizing<[u8; 32]>>>);

impl Session {
    pub fn dek(&self) -> Result<Zeroizing<[u8; 32]>, String> {
        self.0
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| "No active session. Unlock first.".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct NoteEntry {
    pub id: String,
    pub title: String,
    pub color: Option<String>,
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
            color TEXT,
            -- The colour is interpolated into a CSS property, so only a hex triple.
            CHECK (color IS NULL OR color GLOB '#[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]')
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
            encrypted_content BLOB NOT NULL,
            nonce BLOB NOT NULL,
            color TEXT,
            category_id TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL,
            -- The body is capped at 256 characters. The column holds ciphertext, so
            -- the constraint can only bound the worst case in bytes: 256 * 4 UTF-8
            -- bytes plus the 16-byte AES-GCM tag, and at least 1 byte plus the tag.
            CHECK (length(encrypted_content) BETWEEN 17 AND 1040),
            -- The colour is interpolated into a CSS property, so only a hex triple.
            CHECK (color IS NULL OR color GLOB '#[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]')
        );

        INSERT OR IGNORE INTO settings (key, value) VALUES ('theme', 'dark');
        ",
    )?;

    app.manage(DbConn(Mutex::new(conn)));
    app.manage(Session(Mutex::new(None)));
    Ok(())
}

pub fn unix_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or_default()
        .to_string()
}
