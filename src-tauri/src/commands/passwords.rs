use crate::crypto;
use crate::db::{unix_timestamp, DbConn, PasswordEntry, Session};
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn add_password(
    db: State<'_, DbConn>,
    session: State<'_, Session>,
    username: String,
    password: String,
    url: String,
    category_id: Option<String>,
) -> Result<PasswordEntry, String> {
    let conn = db.0.lock().unwrap();
    let dek = session.dek()?;

    let (encrypted, nonce) = crypto::encrypt_password(password.as_bytes(), &dek)?;

    let id = Uuid::new_v4().to_string();
    let now = unix_timestamp();

    conn.execute(
        "INSERT INTO passwords (id, username, encrypted_password, nonce, url, category_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![id, username, encrypted, nonce, url, category_id, now, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(PasswordEntry {
        id,
        username,
        url,
        category_id,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub fn get_passwords(
    state: State<'_, DbConn>,
    session: State<'_, Session>,
    category_id: Option<String>,
) -> Result<Vec<PasswordEntry>, String> {
    session.require_unlocked()?;
    let conn = state.0.lock().unwrap();

    let mut stmt = if category_id.is_some() {
        conn.prepare("SELECT id, username, url, category_id, created_at, updated_at FROM passwords WHERE category_id = ?1 ORDER BY created_at DESC")
            .map_err(|e| e.to_string())?
    } else {
        conn.prepare("SELECT id, username, url, category_id, created_at, updated_at FROM passwords ORDER BY created_at DESC")
            .map_err(|e| e.to_string())?
    };

    let map_row = |row: &rusqlite::Row| {
        Ok(PasswordEntry {
            id: row.get(0)?,
            username: row.get(1)?,
            url: row.get(2)?,
            category_id: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    };

    let rows = if let Some(ref cat_id) = category_id {
        stmt.query_map(rusqlite::params![cat_id], map_row)
    } else {
        stmt.query_map([], map_row)
    }
    .map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

#[tauri::command]
pub async fn update_password(
    db: State<'_, DbConn>,
    session: State<'_, Session>,
    id: String,
    username: String,
    password: Option<String>,
    url: String,
    category_id: Option<String>,
) -> Result<(), String> {
    let conn = db.0.lock().unwrap();
    let now = unix_timestamp();

    if let Some(ref pwd) = password {
        let dek = session.dek()?;
        let (encrypted, nonce) = crypto::encrypt_password(pwd.as_bytes(), &dek)?;
        conn.execute(
            "UPDATE passwords SET username = ?1, encrypted_password = ?2, nonce = ?3, url = ?4, category_id = ?5, updated_at = ?6 WHERE id = ?7",
            rusqlite::params![username, encrypted, nonce, url, category_id, now, id],
        )
        .map_err(|e| e.to_string())?;
    } else {
        conn.execute(
            "UPDATE passwords SET username = ?1, url = ?2, category_id = ?3, updated_at = ?4 WHERE id = ?5",
            rusqlite::params![username, url, category_id, now, id],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn delete_password(
    state: State<'_, DbConn>,
    session: State<'_, Session>,
    id: String,
) -> Result<(), String> {
    session.require_unlocked()?;
    let conn = state.0.lock().unwrap();
    conn.execute("DELETE FROM passwords WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn decrypt_password_by_id(
    db: State<'_, DbConn>,
    session: State<'_, Session>,
    id: String,
) -> Result<String, String> {
    let conn = db.0.lock().unwrap();
    let dek = session.dek()?;

    let (encrypted, nonce): (Vec<u8>, Vec<u8>) = conn
        .query_row(
            "SELECT encrypted_password, nonce FROM passwords WHERE id = ?1",
            rusqlite::params![id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| e.to_string())?;

    let plaintext = crypto::decrypt_password(&encrypted, &nonce, &dek)?;

    String::from_utf8(plaintext).map_err(|e| e.to_string())
}
