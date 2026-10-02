use crate::crypto;
use crate::db::{DbConn, MasterPasswordInfo, Session};
use tauri::State;
use zeroize::Zeroizing;

/// The shortest master password the vault will accept.
///
/// This lives here rather than only in the frontend because the IPC is reachable
/// from any script that ends up in the webview, so a rule that only exists in
/// `src/routes/setup/+page.svelte` is a suggestion: `setup_master_password` would
/// happily wrap a vault under `""`. The setup page mirrors the number from
/// `PASSWORD_MIN_LENGTH` in `src/lib/consts.ts` for its own message, and nothing
/// here trusts that copy — nor does anything there trust this one, which is why the
/// tests below pin both sides to the same number.
///
/// Raising it only constrains passwords being *set*. [`verify_master_password`]
/// deliberately does not check it, so a vault created under a shorter rule keeps
/// unlocking and can be rotated onto a longer one; without that asymmetry, every
/// bump would brick the installs it was meant to protect.
pub const MASTER_PASSWORD_MIN: usize = 12;

/// Rejects a master password the vault must not be built on.
///
/// Counted in characters rather than bytes, the same way
/// `notes::check_content` counts a body, so a passphrase of accented characters is
/// not penalised by its encoding. The frontend's own check compares UTF-16 code
/// units, so for an emoji-heavy passphrase this is the stricter of the two — which
/// is the right way round, because this is the side that decides.
fn check_master_password(password: &str) -> Result<(), String> {
    let length = password.chars().count();

    if length < MASTER_PASSWORD_MIN {
        return Err(format!(
            "Master password must be at least {MASTER_PASSWORD_MIN} characters"
        ));
    }

    Ok(())
}

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

/// What the setup worker hands back: the material to persist and the DEK to
/// unlock the session with.
///
/// A struct rather than a tuple because `wrapped_dek` and `dek_nonce` are both
/// `Vec<u8>`, so a tuple would compile happily with the two swapped and the
/// mistake would only surface when the vault failed to unlock.
struct NewMaster {
    salt: [u8; 16],
    dek: Zeroizing<[u8; 32]>,
    wrapped_dek: Vec<u8>,
    dek_nonce: Vec<u8>,
}

/// The rewrapped DEK for `change_master_password`, shaped like the three
/// `master_password` columns it replaces.
struct RewrappedMaster {
    salt: Vec<u8>,
    wrapped_dek: Vec<u8>,
    dek_nonce: Vec<u8>,
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

    // Before `password.into_bytes()` and before the derivation, so a rejected
    // passphrase costs nothing: the KDF below spends 64MB and three iterations to
    // produce a key nobody is going to use.
    check_master_password(&password)?;

    let password_bytes = password.into_bytes();

    let NewMaster {
        salt,
        dek,
        wrapped_dek,
        dek_nonce,
    } = tauri::async_runtime::spawn_blocking(move || -> Result<NewMaster, String> {
        let salt = crypto::generate_salt();
        let kek = Zeroizing::new(crypto::derive_kek(&password_bytes, &salt)?);
        let dek = Zeroizing::new(crypto::generate_dek());
        let (wrapped_dek, dek_nonce) = crypto::wrap_dek(&dek, &kek)?;
        Ok(NewMaster {
            salt,
            dek,
            wrapped_dek,
            dek_nonce,
        })
    })
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

    session.unlock(dek);
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

    let dek =
        tauri::async_runtime::spawn_blocking(move || -> Result<Zeroizing<[u8; 32]>, String> {
            let kek = Zeroizing::new(crypto::derive_kek(&password_bytes, &info.salt)?);
            Ok(Zeroizing::new(crypto::unwrap_dek(
                &info.wrapped_dek,
                &info.dek_nonce,
                &kek,
            )?))
        })
        .await
        .map_err(|e| e.to_string())??;

    session.unlock(dek);
    Ok(true)
}

#[tauri::command]
pub fn lock_session(session: State<'_, Session>) -> Result<(), String> {
    session.lock();
    Ok(())
}

#[tauri::command]
pub async fn change_master_password(
    db: State<'_, DbConn>,
    old_password: String,
    new_password: String,
) -> Result<(), String> {
    // The same rule `setup_master_password` enforces, checked before the database
    // is touched: rotating onto a passphrase the vault would have refused at setup
    // is not a weaker position than starting with it.
    check_master_password(&new_password)?;

    let info = {
        let conn = db.0.lock().unwrap();
        get_master_info(&conn)?
    };

    let old_bytes = old_password.into_bytes();
    let new_bytes = new_password.into_bytes();

    let RewrappedMaster {
        salt: new_salt,
        wrapped_dek: new_wrapped,
        dek_nonce: new_nonce,
    } = tauri::async_runtime::spawn_blocking(move || -> Result<RewrappedMaster, String> {
        let old_kek = Zeroizing::new(crypto::derive_kek(&old_bytes, &info.salt)?);
        let dek = Zeroizing::new(crypto::unwrap_dek(
            &info.wrapped_dek,
            &info.dek_nonce,
            &old_kek,
        )?);

        let new_salt = crypto::generate_salt();
        let new_kek = Zeroizing::new(crypto::derive_kek(&new_bytes, &new_salt)?);
        let (new_wrapped, new_nonce) = crypto::wrap_dek(&dek, &new_kek)?;
        Ok(RewrappedMaster {
            salt: new_salt.to_vec(),
            wrapped_dek: new_wrapped,
            dek_nonce: new_nonce,
        })
    })
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

#[cfg(test)]
mod tests {
    use super::{check_master_password, MASTER_PASSWORD_MIN};

    /// Spelled out rather than built from the constant: this is the string the
    /// setup page shows the user, so a drifting number has to be visible here
    /// rather than reproduced faithfully by a test.
    const REFUSED: &str = "Master password must be at least 12 characters";

    fn err(password: &str) -> String {
        check_master_password(password).unwrap_err()
    }

    /// The whole point of moving the rule into the backend: without it, this vault
    /// would be wrapped under an empty passphrase and every row inside it would be
    /// encrypted with a DEK that anyone can unwrap.
    #[test]
    fn an_empty_password_is_refused() {
        assert_eq!(err(""), REFUSED);
    }

    #[test]
    fn every_length_below_the_minimum_is_refused() {
        for len in 0..MASTER_PASSWORD_MIN {
            assert_eq!(
                err(&"a".repeat(len)),
                REFUSED,
                "a {len}-character password must be refused"
            );
        }
    }

    /// The boundary is the part worth pinning: off-by-one here either locks out a
    /// legitimate passphrase or lets one through that the setup page refused.
    #[test]
    fn the_minimum_length_is_accepted() {
        assert!(check_master_password(&"a".repeat(MASTER_PASSWORD_MIN)).is_ok());
    }

    /// Pinning the number is the whole reason this file cannot quietly drift away
    /// from `PASSWORD_MIN_LENGTH` in `src/lib/consts.ts`. The two sides enforce the
    /// same rule and neither reads the other, so the tests are the only place they
    /// meet.
    #[test]
    fn the_minimum_is_twelve() {
        assert_eq!(MASTER_PASSWORD_MIN, 12);
    }

    #[test]
    fn the_rule_is_a_floor_and_not_a_ceiling() {
        assert!(check_master_password(&"a".repeat(200)).is_ok());
        assert!(
            check_master_password("correct horse battery staple").is_ok(),
            "a passphrase is the case the rule must not get in the way of"
        );
    }

    /// Counted in characters, not bytes, so a passphrase is not measured by its
    /// encoding. Twelve accented characters is twelve characters even though it is
    /// more than twelve bytes.
    #[test]
    fn length_is_counted_in_characters_rather_than_bytes() {
        assert!(
            check_master_password(&"ñ".repeat(MASTER_PASSWORD_MIN)).is_ok(),
            "12 chars, 14 bytes"
        );
        assert!(
            check_master_password(&"ñ".repeat(MASTER_PASSWORD_MIN - 1)).is_err(),
            "11 chars, 13 bytes"
        );
    }
}
