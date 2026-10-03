use crate::color;
use crate::crypto;
use crate::db::{
    check_max_chars, unix_timestamp, DbConn, NoteEntry, Session, NOTE_CONTENT_MAX_CHARS,
    NOTE_TITLE_MAX_CHARS,
};
use tauri::State;
use uuid::Uuid;

fn check_content(content: &str) -> Result<(), String> {
    // `notes.content` is the one column here with a lower bound as well, and it
    // keeps it: unlike the metadata columns, an empty body has never been a valid
    // row and there is no legacy value to protect.
    if content.is_empty() {
        return Err("Content is required".to_string());
    }

    check_max_chars("Content", content, NOTE_CONTENT_MAX_CHARS)
}

/// The one plaintext column `add_note` and `update_note` write that `check_content`
/// does not already cover.
fn check_title(title: &str) -> Result<(), String> {
    check_max_chars("Title", title, NOTE_TITLE_MAX_CHARS)
}

#[tauri::command]
pub async fn add_note(
    db: State<'_, DbConn>,
    session: State<'_, Session>,
    title: String,
    content: String,
    color: Option<String>,
    category_id: Option<String>,
) -> Result<NoteEntry, String> {
    let conn = db.0.lock().unwrap();
    let dek = session.dek()?;
    check_content(&content)?;
    check_title(&title)?;
    let color = color::normalize(color)?;

    let (encrypted, nonce) = crypto::encrypt_password(content.as_bytes(), &dek)?;

    let id = Uuid::new_v4().to_string();
    let now = unix_timestamp();

    conn.execute(
        "INSERT INTO notes (id, title, encrypted_content, nonce, color, category_id, created_at, updated_at) \
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![id, title, encrypted, nonce, color, category_id, now, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(NoteEntry {
        id,
        title,
        color,
        category_id,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub fn get_notes(
    state: State<'_, DbConn>,
    session: State<'_, Session>,
    category_id: Option<String>,
) -> Result<Vec<NoteEntry>, String> {
    session.require_unlocked()?;
    let conn = state.0.lock().unwrap();

    let mut stmt = if category_id.is_some() {
        conn.prepare("SELECT id, title, color, category_id, created_at, updated_at FROM notes WHERE category_id = ?1 ORDER BY created_at DESC")
            .map_err(|e| e.to_string())?
    } else {
        conn.prepare("SELECT id, title, color, category_id, created_at, updated_at FROM notes ORDER BY created_at DESC")
            .map_err(|e| e.to_string())?
    };

    let map_row = |row: &rusqlite::Row| {
        Ok(NoteEntry {
            id: row.get(0)?,
            title: row.get(1)?,
            color: row.get(2)?,
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
pub async fn update_note(
    db: State<'_, DbConn>,
    session: State<'_, Session>,
    id: String,
    title: String,
    content: Option<String>,
    color: Option<String>,
    category_id: Option<String>,
) -> Result<(), String> {
    session.require_unlocked()?;
    check_title(&title)?;

    let conn = db.0.lock().unwrap();
    let color = color::normalize(color)?;
    let now = unix_timestamp();

    match content {
        Some(content) => {
            let dek = session.dek()?;
            check_content(&content)?;
            let (encrypted, nonce) = crypto::encrypt_password(content.as_bytes(), &dek)?;

            conn.execute(
                "UPDATE notes SET title = ?1, encrypted_content = ?2, nonce = ?3, color = ?4, \
                category_id = ?5, updated_at = ?6 WHERE id = ?7",
                rusqlite::params![title, encrypted, nonce, color, category_id, now, id],
            )
            .map_err(|e| e.to_string())?;
        }
        None => {
            conn.execute(
                "UPDATE notes SET title = ?1, color = ?2, category_id = ?3, updated_at = ?4 \
                WHERE id = ?5",
                rusqlite::params![title, color, category_id, now, id],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn delete_note(
    state: State<'_, DbConn>,
    session: State<'_, Session>,
    id: String,
) -> Result<(), String> {
    session.require_unlocked()?;
    let conn = state.0.lock().unwrap();
    conn.execute("DELETE FROM notes WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn decrypt_note_by_id(
    db: State<'_, DbConn>,
    session: State<'_, Session>,
    id: String,
) -> Result<String, String> {
    let conn = db.0.lock().unwrap();
    let dek = session.dek()?;

    let (encrypted, nonce): (Vec<u8>, Vec<u8>) = conn
        .query_row(
            "SELECT encrypted_content, nonce FROM notes WHERE id = ?1",
            rusqlite::params![id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| e.to_string())?;

    let plaintext = crypto::decrypt_password(&encrypted, &nonce, &dek)?;

    String::from_utf8(plaintext).map_err(|e| e.to_string())
}
