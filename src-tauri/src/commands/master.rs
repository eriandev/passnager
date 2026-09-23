use crate::crypto;
use crate::db::{DbConn, MasterPasswordInfo};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use tauri::State;

fn get_master_info(conn: &rusqlite::Connection) -> Result<MasterPasswordInfo, String> {
    conn.query_row(
        "SELECT salt, wrapped_dek, dek_nonce FROM master_password WHERE id = 1",
        [],
        |row| {
            Ok(MasterPasswordInfo {
                salt: row.get(0)?,
                wrapped_dek: row.get(1)?,
                dek_nonce: row.get(2)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn is_master_configured(state: State<'_, DbConn>) -> Result<bool, String> {
    let conn = state.0.lock().unwrap();
    let result: Result<i32, _> =
        conn.query_row("SELECT COUNT(*) FROM master_password", [], |row| row.get(0));
    match result {
        Ok(count) => Ok(count > 0),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
pub fn setup_master_password(
    state: State<'_, DbConn>,
    password: String,
) -> Result<String, String> {
    let conn = state.0.lock().unwrap();

    let count: i32 = conn
        .query_row("SELECT COUNT(*) FROM master_password", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    if count > 0 {
        return Err("Master password already configured".to_string());
    }

    let salt = crypto::generate_salt();
    let kek = crypto::derive_kek(password.as_bytes(), &salt)?;
    let dek = crypto::generate_dek();
    let (wrapped_dek, dek_nonce) = crypto::wrap_dek(&dek, &kek)?;

    conn.execute(
        "INSERT INTO master_password (id, salt, wrapped_dek, dek_nonce) VALUES (1, ?1, ?2, ?3)",
        rusqlite::params![salt, wrapped_dek, dek_nonce],
    )
    .map_err(|e| e.to_string())?;

    Ok(BASE64.encode(dek))
}

#[tauri::command]
pub fn verify_master_password(
    state: State<'_, DbConn>,
    password: String,
) -> Result<String, String> {
    let conn = state.0.lock().unwrap();
    let info = get_master_info(&conn)?;

    let kek = crypto::derive_kek(password.as_bytes(), &info.salt)?;
    let dek = crypto::unwrap_dek(&info.wrapped_dek, &info.dek_nonce, &kek)?;

    Ok(BASE64.encode(dek))
}

#[tauri::command]
pub fn change_master_password(
    state: State<'_, DbConn>,
    old_password: String,
    new_password: String,
) -> Result<(), String> {
    let conn = state.0.lock().unwrap();
    let info = get_master_info(&conn)?;

    let old_kek = crypto::derive_kek(old_password.as_bytes(), &info.salt)?;
    let dek = crypto::unwrap_dek(&info.wrapped_dek, &info.dek_nonce, &old_kek)?;

    let new_salt = crypto::generate_salt();
    let new_kek = crypto::derive_kek(new_password.as_bytes(), &new_salt)?;
    let (new_wrapped, new_nonce) = crypto::wrap_dek(&dek, &new_kek)?;

    conn.execute(
        "UPDATE master_password SET salt = ?1, wrapped_dek = ?2, dek_nonce = ?3 WHERE id = 1",
        rusqlite::params![new_salt, new_wrapped, new_nonce],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
