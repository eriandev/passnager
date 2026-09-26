use crate::crypto;
use crate::db::{DbConn, MasterPasswordInfo, Session};
use tauri::State;
use zeroize::Zeroizing;

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

fn set_session_dek(session: &State<'_, Session>, dek: [u8; 32]) {
    let mut guard = session.0.lock().unwrap();
    *guard = Some(Zeroizing::new(dek));
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
pub async fn setup_master_password(
    db: State<'_, DbConn>,
    session: State<'_, Session>,
    password: String,
) -> Result<bool, String> {
    let count: i32 = {
        let conn = db.0.lock().unwrap();
        conn.query_row("SELECT COUNT(*) FROM master_password", [], |row| row.get(0))
            .map_err(|e| e.to_string())?
    };

    if count > 0 {
        return Err("Master password already configured".to_string());
    }

    let password_bytes = password.into_bytes();

    let (salt, dek, wrapped_dek, dek_nonce) = tauri::async_runtime::spawn_blocking(
        move || -> Result<([u8; 16], [u8; 32], Vec<u8>, Vec<u8>), String> {
            let salt = crypto::generate_salt();
            let kek = Zeroizing::new(crypto::derive_kek(&password_bytes, &salt)?);
            let dek = crypto::generate_dek();
            let (wrapped_dek, dek_nonce) = crypto::wrap_dek(&dek, &kek)?;
            Ok((salt, dek, wrapped_dek, dek_nonce))
        },
    )
    .await
    .map_err(|e| e.to_string())??;

    {
        let conn = db.0.lock().unwrap();
        conn.execute(
            "INSERT INTO master_password (id, salt, wrapped_dek, dek_nonce) VALUES (1, ?1, ?2, ?3)",
            rusqlite::params![salt, wrapped_dek, dek_nonce],
        )
        .map_err(|e| e.to_string())?;
    }

    set_session_dek(&session, dek);
    Ok(true)
}

#[tauri::command]
pub async fn verify_master_password(
    db: State<'_, DbConn>,
    session: State<'_, Session>,
    password: String,
) -> Result<bool, String> {
    let info = {
        let conn = db.0.lock().unwrap();
        get_master_info(&conn)?
    };

    let password_bytes = password.into_bytes();

    let dek = tauri::async_runtime::spawn_blocking(move || -> Result<[u8; 32], String> {
        let kek = Zeroizing::new(crypto::derive_kek(&password_bytes, &info.salt)?);
        crypto::unwrap_dek(&info.wrapped_dek, &info.dek_nonce, &kek)
    })
    .await
    .map_err(|e| e.to_string())??;

    set_session_dek(&session, dek);
    Ok(true)
}

#[tauri::command]
pub fn lock_session(state: State<'_, Session>) -> Result<(), String> {
    let mut guard = state.0.lock().unwrap();
    *guard = None;
    Ok(())
}

#[tauri::command]
pub async fn change_master_password(
    db: State<'_, DbConn>,
    old_password: String,
    new_password: String,
) -> Result<(), String> {
    let info = {
        let conn = db.0.lock().unwrap();
        get_master_info(&conn)?
    };

    let old_bytes = old_password.into_bytes();
    let new_bytes = new_password.into_bytes();

    let (new_salt, new_wrapped, new_nonce) = tauri::async_runtime::spawn_blocking(
        move || -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), String> {
            let old_kek = Zeroizing::new(crypto::derive_kek(&old_bytes, &info.salt)?);
            let dek = Zeroizing::new(crypto::unwrap_dek(
                &info.wrapped_dek,
                &info.dek_nonce,
                &old_kek,
            )?);

            let new_salt = crypto::generate_salt();
            let new_kek = Zeroizing::new(crypto::derive_kek(&new_bytes, &new_salt)?);
            let (new_wrapped, new_nonce) = crypto::wrap_dek(&dek, &new_kek)?;
            Ok((new_salt.to_vec(), new_wrapped, new_nonce))
        },
    )
    .await
    .map_err(|e| e.to_string())??;

    {
        let conn = db.0.lock().unwrap();
        conn.execute(
            "UPDATE master_password SET salt = ?1, wrapped_dek = ?2, dek_nonce = ?3 WHERE id = 1",
            rusqlite::params![new_salt, new_wrapped, new_nonce],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}
