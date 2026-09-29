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

use super::*;
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
