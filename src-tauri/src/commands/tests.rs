#![cfg(test)]

//! The session gate, exercised through the real command functions.
//!
//! Every other module in this suite tests a unit in isolation. This one exists for
//! the thing that cannot be checked that way: that each command actually consults
//! the session. A command missing its `require_unlocked` call still passes every
//! unit test in `db.rs`, and the frontend route guard is not a substitute, because
//! the Tauri IPC is reachable from any script that ends up in the webview.
//!
//! The list below is deliberately explicit rather than generated. It should match
//! the `invoke_handler` in `lib.rs` one for one, and a reader has to be able to
//! check that by eye.
//!
//! The last section is the one exception: it is not about the session gate but
//! about the master password policy, which for the same reason — the IPC is
//! reachable from any script that ends up in the webview — has to hold even when
//! the frontend never gets to check it.

use super::*;
use crate::crypto;
use crate::db::{DbConn, Session, SCHEMA};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::test::{mock_builder, mock_context, noop_assets, MockRuntime};
use tauri::{App, Manager};
use zeroize::Zeroizing;

/// The error `Session` returns for every locked call. Asserted as an exact string
/// so the frontend can key off it, and so a command that fails for some *other*
/// reason cannot be mistaken for a locked vault.
const LOCKED: &str = "No active session. Unlock first.";

const DEK: [u8; 32] = [7u8; 32];

/// An app backed by an empty in-memory vault and a locked session, which is the
/// state in which no vault command is allowed to do anything.
fn locked() -> App<MockRuntime> {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(SCHEMA).unwrap();

    mock_builder()
        .manage(DbConn(Mutex::new(conn)))
        .manage(Session::new())
        .build(mock_context(noop_assets()))
        .unwrap()
}

/// The same, unlocked. A gate test that passes because the command errored for an
/// unrelated reason is worse than no test, so the success paths are covered too.
fn unlocked() -> App<MockRuntime> {
    let app = locked();
    app.state::<Session>().unlock(Zeroizing::new(DEK));
    app
}

fn assert_locked<T>(result: Result<T, String>, command: &str) {
    match result {
        Err(e) => assert_eq!(e, LOCKED, "{command} should report a locked vault"),
        Ok(_) => panic!("{command} succeeded on a locked session"),
    }
}

// --- categories -------------------------------------------------------------

#[test]
fn add_category_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        categories::add_category(
            app.state::<DbConn>(),
            app.state::<Session>(),
            "work".into(),
            None,
            None,
        ),
        "add_category",
    );
}

#[test]
fn get_categories_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        categories::get_categories(app.state::<DbConn>(), app.state::<Session>()),
        "get_categories",
    );
}

#[test]
fn update_category_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        categories::update_category(
            app.state::<DbConn>(),
            app.state::<Session>(),
            "c1".into(),
            "work".into(),
            None,
            None,
        ),
        "update_category",
    );
}

#[test]
fn delete_category_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        categories::delete_category(app.state::<DbConn>(), app.state::<Session>(), "c1".into()),
        "delete_category",
    );
}

// --- notes ------------------------------------------------------------------

#[tokio::test]
async fn add_note_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        notes::add_note(
            app.state::<DbConn>(),
            app.state::<Session>(),
            "secret".into(),
            "body".into(),
            None,
            None,
        )
        .await,
        "add_note",
    );
}

#[test]
fn get_notes_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        notes::get_notes(app.state::<DbConn>(), app.state::<Session>(), None),
        "get_notes",
    );
}

#[tokio::test]
async fn update_note_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        notes::update_note(
            app.state::<DbConn>(),
            app.state::<Session>(),
            "n1".into(),
            "new title".into(),
            Some("new body".into()),
            None,
            None,
        )
        .await,
        "update_note",
    );
}

/// The content-less branch renames and recolours a note without touching the DEK,
/// so it is the branch most likely to skip the session check by accident.
#[tokio::test]
async fn update_note_without_content_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        notes::update_note(
            app.state::<DbConn>(),
            app.state::<Session>(),
            "n1".into(),
            "new title".into(),
            None,
            Some("#fff740".into()),
            None,
        )
        .await,
        "update_note (no content)",
    );
}

#[test]
fn delete_note_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        notes::delete_note(app.state::<DbConn>(), app.state::<Session>(), "n1".into()),
        "delete_note",
    );
}

#[tokio::test]
async fn decrypt_note_by_id_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        notes::decrypt_note_by_id(app.state::<DbConn>(), app.state::<Session>(), "n1".into()).await,
        "decrypt_note_by_id",
    );
}

// --- passwords --------------------------------------------------------------

#[tokio::test]
async fn add_password_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        passwords::add_password(
            app.state::<DbConn>(),
            app.state::<Session>(),
            "user".into(),
            "hunter2".into(),
            "https://example.test".into(),
            None,
        )
        .await,
        "add_password",
    );
}

#[test]
fn get_passwords_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        passwords::get_passwords(app.state::<DbConn>(), app.state::<Session>(), None),
        "get_passwords",
    );
}

#[tokio::test]
async fn update_password_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        passwords::update_password(
            app.state::<DbConn>(),
            app.state::<Session>(),
            "p1".into(),
            "user".into(),
            Some("hunter3".into()),
            "https://example.test".into(),
            None,
        )
        .await,
        "update_password",
    );
}

/// Same reasoning as the content-less note update: a password-only rename never
/// reaches the DEK.
#[tokio::test]
async fn update_password_without_a_new_password_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        passwords::update_password(
            app.state::<DbConn>(),
            app.state::<Session>(),
            "p1".into(),
            "user".into(),
            None,
            "https://example.test".into(),
            None,
        )
        .await,
        "update_password (no password)",
    );
}

#[test]
fn delete_password_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        passwords::delete_password(app.state::<DbConn>(), app.state::<Session>(), "p1".into()),
        "delete_password",
    );
}

#[tokio::test]
async fn decrypt_password_by_id_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        passwords::decrypt_password_by_id(
            app.state::<DbConn>(),
            app.state::<Session>(),
            "p1".into(),
        )
        .await,
        "decrypt_password_by_id",
    );
}

// --- clipboard --------------------------------------------------------------

/// Only the locked path is tested. The unlocked one writes to the real system
/// clipboard, which a test has no business doing. Note that the plaintext in
/// `text` is the whole reason this command needs a gate at all: it is the one
/// command that hands decrypted material to another process.
#[test]
fn copy_to_clipboard_refuses_a_locked_session() {
    let app = locked();
    assert_locked(
        clipboard::copy_to_clipboard(app.state::<Session>(), "hunter2".into()),
        "copy_to_clipboard",
    );
}

// --- the gate is a lock, not a one-off check --------------------------------

/// A vault that was unlocked and then locked must go back to refusing, otherwise
/// the single most important transition in the app is not covered by any test.
#[test]
fn relocking_restores_every_refusal() {
    let app = unlocked();
    app.state::<Session>().lock();

    assert_locked(
        categories::get_categories(app.state::<DbConn>(), app.state::<Session>()),
        "get_categories after relock",
    );
    assert_locked(
        notes::get_notes(app.state::<DbConn>(), app.state::<Session>(), None),
        "get_notes after relock",
    );
    assert_locked(
        passwords::get_passwords(app.state::<DbConn>(), app.state::<Session>(), None),
        "get_passwords after relock",
    );
    assert_locked(
        clipboard::copy_to_clipboard(app.state::<Session>(), "x".into()),
        "copy_to_clipboard after relock",
    );
}

// --- and the gate is not just refusing --------------------------------------

/// A gate that always refused would pass every test above. This confirms the
/// commands still do their job once the session is live, and that the check is
/// not sitting in front of a stub.
#[tokio::test]
async fn the_vault_works_again_once_unlocked() {
    let app = unlocked();
    let db = app.state::<DbConn>();
    let session = app.state::<Session>();

    let category = categories::add_category(db.clone(), session.clone(), "work".into(), None, None)
        .expect("unlocked add_category should work");

    let note = notes::add_note(
        db.clone(),
        session.clone(),
        "secret".into(),
        "body".into(),
        None,
        Some(category.id.clone()),
    )
    .await
    .expect("unlocked add_note should work");

    assert_eq!(
        notes::decrypt_note_by_id(db.clone(), session.clone(), note.id.clone())
            .await
            .unwrap(),
        "body"
    );

    let password = passwords::add_password(
        db.clone(),
        session.clone(),
        "user".into(),
        "hunter2".into(),
        "https://example.test".into(),
        Some(category.id.clone()),
    )
    .await
    .expect("unlocked add_password should work");

    assert_eq!(
        passwords::decrypt_password_by_id(db, session, password.id)
            .await
            .unwrap(),
        "hunter2"
    );

    // A metadata-only edit, the branch the gate tests above cover while locked.
    notes::update_note(
        app.state::<DbConn>(),
        app.state::<Session>(),
        note.id,
        "renamed".into(),
        None,
        Some("#3ee0cf".into()),
        None,
    )
    .await
    .expect("unlocked metadata-only update should work");
}

// --- the master password policy ----------------------------------------------

/// The refusal both master commands have to give, and the one they must give for
/// every short password rather than a per-length message.
///
/// Spelled out instead of formatted from the constant: this is the string the
/// setup page puts in front of the user, so a drifting number should fail here
/// instead of being reproduced faithfully. The constant is asserted alongside it,
/// and `PASSWORD_MIN_LENGTH` in `src/lib/consts.ts` is the third copy of the same
/// number, which is why all of them are pinned rather than derived.
fn too_short() -> String {
    assert_eq!(
        12,
        master::MASTER_PASSWORD_MIN,
        "PASSWORD_MIN_LENGTH in src/lib/consts.ts mirrors this number"
    );
    "Master password must be at least 12 characters".to_string()
}

/// Passwords the policy has to refuse, shared by the setup and rotation tests so
/// the two paths cannot be shown to agree by coincidence.
const TOO_SHORT: [&str; 4] = ["", "short", "1234567", "eleven char"];

/// A literal in `TOO_SHORT` that is not actually too short would silently turn its
/// test into a positive case — the loop would find the call *succeeding* and report
/// a confusing mismatch instead of a miscounted string. Counting characters by eye
/// is exactly the mistake this guards, and it is worth having: it is how the entry
/// above was wrong once already.
#[test]
fn every_entry_of_the_too_short_list_is_really_too_short() {
    for password in TOO_SHORT {
        assert!(
            password.chars().count() < master::MASTER_PASSWORD_MIN,
            "{password:?} has {} characters, so it is not below the minimum",
            password.chars().count()
        );
    }

    // And the list has to reach right up to the boundary, or it is not testing the
    // length rule at all — just obviously-short passwords.
    let closest = TOO_SHORT.iter().map(|p| p.chars().count()).max().unwrap();
    assert_eq!(closest, master::MASTER_PASSWORD_MIN - 1);
}

/// The empty passphrase is the case that matters: without this check the vault is
/// wrapped under it and every row inside is sealed with a DEK anybody can unwrap.
/// The setup page refuses it, but the setup page is not the boundary.
#[tokio::test]
async fn setup_refuses_a_master_password_the_frontend_would_refuse() {
    let app = locked();
    let db = app.state::<DbConn>();
    let session = app.state::<Session>();

    for password in TOO_SHORT {
        assert_eq!(
            master::setup_master_password(db.clone(), session.clone(), password.into())
                .await
                .unwrap_err(),
            too_short(),
            "{password:?} must not be able to create a vault"
        );
    }

    assert!(
        session.dek().is_err(),
        "a refused setup must not unlock the session"
    );
    let configured: i64 =
        db.0.lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM master_password", [], |row| row.get(0))
            .unwrap();
    assert_eq!(configured, 0, "a refused setup must not write a master row");
}

/// Same rule on the rotation path. The vault already exists here, so this is the
/// command that would otherwise be the way to *downgrade* an existing master
/// password to something short.
#[tokio::test]
async fn change_refuses_a_new_master_password_the_frontend_would_refuse() {
    let app = unlocked();
    let db = app.state::<DbConn>();

    for password in TOO_SHORT {
        assert_eq!(
            master::change_master_password(
                db.clone(),
                "whatever the old one was".into(),
                password.into(),
            )
            .await
            .unwrap_err(),
            too_short(),
            "a rotation onto {password:?} must be refused"
        );
    }
}

#[tokio::test]
async fn setup_still_writes_the_row_and_unlocks_for_a_long_enough_password() {
    let app = locked();
    let db = app.state::<DbConn>();
    let session = app.state::<Session>();

    // The other half of the pair above: proving the check rejects only short
    // passwords needs a positive case, and this one also covers the side effects
    // the refusal test asserts are absent.
    //
    // Expensive by design, and the only test that pays for production Argon2
    // through this path: a stubbed KDF here would not catch a policy that is
    // enforced after the row is written.
    assert!(master::setup_master_password(
        db.clone(),
        session.clone(),
        "a".repeat(master::MASTER_PASSWORD_MIN),
    )
    .await
    .is_ok());

    assert!(session.dek().is_ok(), "setup leaves the vault unlocked");

    let stored: i64 =
        db.0.lock()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM master_password", [], |row| row.get(0))
            .unwrap();
    assert_eq!(stored, 1, "setup persists exactly one master row");

    // And the second call is refused as already configured, which is the state the
    // `/setup` dead end in the frontend turns into.
    assert_eq!(
        master::setup_master_password(db, session, "a".repeat(master::MASTER_PASSWORD_MIN),)
            .await
            .unwrap_err(),
        "Master password already configured"
    );
}

/// The check has to come before the work rather than after it, or the caller pays
/// for a 64MB Argon2 derivation on a request that is going to be refused — and
/// the error the user sees would be about the old password rather than about the
/// one they just typed.
#[tokio::test]
async fn the_policy_is_checked_before_the_old_password_or_the_key_is_touched() {
    let app = locked();
    let db = app.state::<DbConn>();

    // The vault has no master row at all, so any error here came from the policy
    // and not from the lookup that follows it. The old password is long enough to
    // pass on its own account, which is what makes the new one the only thing left
    // to refuse.
    assert_eq!(
        master::change_master_password(db, "a long enough old one".into(), "short".into())
            .await
            .unwrap_err(),
        too_short(),
        "the length must be settled before the database is read"
    );
}

/// The policy guards passwords being *set*, not passwords being *used*, and that
/// asymmetry is the only thing that makes raising the minimum safe: a vault created
/// under an older, shorter rule has to keep opening, or bumping the number bricks
/// every install it was meant to protect.
///
/// The master row is written directly rather than through `setup_master_password`,
/// which is the only way to obtain a vault the current policy would refuse. Its
/// blob lengths are the ones the schema accepts, so this is a legitimate row and
/// not a corrupt one.
#[tokio::test]
async fn a_vault_whose_password_is_now_too_short_still_unlocks() {
    let app = locked();
    let db = app.state::<DbConn>();
    let session = app.state::<Session>();

    let short = "eight chars";
    assert!(
        short.chars().count() < master::MASTER_PASSWORD_MIN,
        "this vault has to be one the current policy would refuse"
    );

    // Exactly what a build with the old minimum would have written.
    let salt = crypto::generate_salt();
    let kek = Zeroizing::new(crypto::derive_kek(short.as_bytes(), &salt).unwrap());
    let (wrapped_dek, dek_nonce) = crypto::wrap_dek(&crypto::generate_dek(), &kek).unwrap();
    db.0.lock()
        .unwrap()
        .execute(
            "INSERT INTO master_password (id, salt, wrapped_dek, dek_nonce) \
             VALUES (1, ?1, ?2, ?3)",
            rusqlite::params![salt, wrapped_dek, dek_nonce],
        )
        .unwrap();

    master::verify_master_password(db, session.clone(), short.into())
        .await
        .expect("a vault built under an older minimum must still open");

    assert!(session.dek().is_ok());
}

/// And the way out of that state is a rotation, which is why `change_master_password`
/// validates only the new password: an owner stuck on a short passphrase has to be
/// able to move off it without being able to create a new vault, since one already
/// exists.
#[tokio::test]
async fn an_owner_of_a_short_password_can_rotate_onto_a_longer_one() {
    let app = locked();
    let db = app.state::<DbConn>();
    let session = app.state::<Session>();

    let short = "eight chars";
    let longer = "a".repeat(master::MASTER_PASSWORD_MIN);

    let salt = crypto::generate_salt();
    let kek = Zeroizing::new(crypto::derive_kek(short.as_bytes(), &salt).unwrap());
    let (wrapped_dek, dek_nonce) = crypto::wrap_dek(&crypto::generate_dek(), &kek).unwrap();
    db.0.lock()
        .unwrap()
        .execute(
            "INSERT INTO master_password (id, salt, wrapped_dek, dek_nonce) \
             VALUES (1, ?1, ?2, ?3)",
            rusqlite::params![salt, wrapped_dek, dek_nonce],
        )
        .unwrap();

    master::change_master_password(db.clone(), short.into(), longer.clone())
        .await
        .expect("a short password is not a reason to refuse the upgrade");

    // The old one no longer opens it, and the new one does.
    assert!(
        master::verify_master_password(db.clone(), session.clone(), short.into())
            .await
            .is_err()
    );
    assert!(
        master::verify_master_password(db, session, longer)
            .await
            .is_ok(),
        "the rotation has to actually take effect"
    );
}
