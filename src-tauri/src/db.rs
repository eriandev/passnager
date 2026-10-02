use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;
use zeroize::Zeroizing;

pub struct DbConn(pub Mutex<Connection>);

/// The unlocked-vault state: the in-memory DEK that every encrypted row is
/// sealed with.
///
/// The mutex is private so the key can only be read, stored or dropped through
/// the three methods below. That is the whole point of this type: no command
/// should be reaching into the key material, and hiding the field makes that a
/// compile error rather than a convention someone can quietly break.
#[derive(Default)]
pub struct Session(Mutex<Option<Zeroizing<[u8; 32]>>>);

impl Session {
    /// Shared so every rejection reads the same to the frontend.
    const LOCKED: &'static str = "No active session. Unlock first.";

    pub fn new() -> Self {
        Self::default()
    }

    /// The key for the current session, or an error if the vault is locked.
    ///
    /// Takes `&self` and hands back an owned copy, so the DEK a command encrypts
    /// with cannot outlive the borrow of the session and the lock is not held
    /// across the encryption itself.
    pub fn dek(&self) -> Result<Zeroizing<[u8; 32]>, String> {
        self.0
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| Self::LOCKED.to_string())
    }

    /// Errors unless the vault is unlocked, without handing out the key.
    ///
    /// For the commands that read metadata but never touch ciphertext. They are
    /// still reading the vault's contents, so they belong behind the same gate as
    /// the ones that decrypt, but they have no reason to clone a DEK just to find
    /// out whether there is one.
    pub fn require_unlocked(&self) -> Result<(), String> {
        self.0
            .lock()
            .unwrap()
            .as_ref()
            .map(|_| ())
            .ok_or_else(|| Self::LOCKED.to_string())
    }

    /// Stores `dek` as the session key, replacing any previous one.
    ///
    /// Takes the key already wrapped, so the only place raw DEK bytes exist is
    /// the worker that unwraps them, and they are zeroed the moment this drops.
    pub fn unlock(&self, dek: Zeroizing<[u8; 32]>) {
        *self.0.lock().unwrap() = Some(dek);
    }

    /// Drops the session key. This is what locking the vault means.
    pub fn lock(&self) {
        *self.0.lock().unwrap() = None;
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
pub struct MasterPasswordInfo {
    pub salt: Vec<u8>,
    pub wrapped_dek: Vec<u8>,
    pub dek_nonce: Vec<u8>,
}

/// The directory holding the vault and the log file.
///
/// Panics when the OS will not name a config dir: with nowhere to put a
/// vault there is no app to run, and a panic in `setup` is a louder and more
/// honest outcome than a vault that silently appears somewhere else.
pub fn config_dir(app: &AppHandle) -> std::path::PathBuf {
    let dir = app
        .path()
        .app_config_dir()
        .expect("failed to get app config dir");
    fs::create_dir_all(&dir).ok();
    dir
}

pub fn db_path(app: &AppHandle) -> std::path::PathBuf {
    config_dir(app).join("passnager.db")
}

/// The whole schema, kept out of `init_db` so the constraints can be tested
/// against an in-memory database instead of only against a real vault on disk.
///
/// The three numbers the blob columns are pinned to are not preferences, they are
/// facts about the crypto module: a 16 byte `crypto::generate_salt`, a 12 byte
/// `crypto::generate_nonce`, and a 32 byte DEK plus the 16 byte AES-GCM tag.
/// `crypto::tests` asserts the same facts from the other side, so neither drifts
/// alone. The rows themselves are rejected by `crypto::stored_nonce` before any of
/// this matters — these constraints exist so a malformed row cannot be written at
/// all, and so the shape is documented next to the data.
///
/// `CREATE TABLE IF NOT EXISTS` leaves an already-created table untouched, so a
/// vault written by an earlier build keeps the schema it has. The Rust-side check
/// is what protects those; these only apply to tables created from here on.
pub(crate) const SCHEMA: &str = "
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS master_password (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            salt BLOB NOT NULL CHECK (length(salt) = 16),
            -- The DEK is 32 bytes and AES-GCM appends a 16 byte tag.
            wrapped_dek BLOB NOT NULL CHECK (length(wrapped_dek) = 48),
            dek_nonce BLOB NOT NULL CHECK (length(dek_nonce) = 12)
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
            nonce BLOB NOT NULL CHECK (length(nonce) = 12),
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
            nonce BLOB NOT NULL CHECK (length(nonce) = 12),
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
        ";

pub fn init_db(app: &AppHandle) -> SqlResult<()> {
    let path = db_path(app);
    let conn = Connection::open(&path)?;

    conn.execute_batch(SCHEMA)?;

    app.manage(DbConn(Mutex::new(conn)));
    app.manage(Session::new());
    Ok(())
}

pub fn unix_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or_default()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto;

    fn in_memory() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(SCHEMA).unwrap();
        conn
    }

    /// The nonce is written as `zeroblob(12)` rather than passed in: it is the same
    /// on every row, and the point of most of these tests is a different column
    /// entirely. Before the `CHECK` landed this fixture used a one byte `x'00'`,
    /// which the constraint now refuses — a reminder that the fixtures were
    /// writing a shape the app never writes.
    fn insert_note(conn: &Connection, id: &str, body: &[u8], color: Option<&str>) -> SqlResult<()> {
        conn.execute(
            "INSERT INTO notes (id, title, encrypted_content, nonce, color, created_at, updated_at) \
             VALUES (?1, 't', ?2, zeroblob(12), ?3, '1', '1')",
            rusqlite::params![id, body, color],
        )
        .map(|_| ())
    }

    fn insert_category(conn: &Connection, id: &str, color: Option<&str>) -> SqlResult<()> {
        conn.execute(
            "INSERT INTO categories (id, name, color) VALUES (?1, 'n', ?2)",
            rusqlite::params![id, color],
        )
        .map(|_| ())
    }

    /// The `notes.encrypted_content` bound is derived from a fact about AES-GCM,
    /// not from a preference: RustCrypto returns `ciphertext || tag` and the tag
    /// is 16 bytes. If a dependency bump ever changed that, the constraint would
    /// silently start rejecting legitimate bodies, and `crypto::tests` asserts the
    /// same fact from the other side. Both are here so neither drifts alone.
    const TAG_LEN: usize = 16;

    fn insert_master(
        conn: &Connection,
        salt: &[u8],
        wrapped_dek: &[u8],
        dek_nonce: &[u8],
    ) -> SqlResult<usize> {
        conn.execute(
            "INSERT INTO master_password (id, salt, wrapped_dek, dek_nonce) \
             VALUES (1, ?1, ?2, ?3)",
            rusqlite::params![salt, wrapped_dek, dek_nonce],
        )
    }

    fn insert_password(conn: &Connection, id: &str, nonce: &[u8]) -> SqlResult<usize> {
        conn.execute(
            "INSERT INTO passwords (id, username, encrypted_password, nonce, url, created_at, updated_at) \
             VALUES (?1, 'u', x'00', ?2, 'https://example.test', '1', '1')",
            rusqlite::params![id, nonce],
        )
    }

    /// Every blob the crypto module reads is written by that module and nowhere
    /// else, so the schema pins the three lengths against the functions that
    /// produce them. A dependency bump that changed the salt, the nonce or the tag
    /// size would fail here instead of silently rejecting every row the app writes.
    #[test]
    fn the_pinned_blob_lengths_are_the_ones_the_crypto_module_produces() {
        assert_eq!(16, crypto::generate_salt().len());
        assert_eq!(12, crypto::generate_nonce().len());

        // `wrap_dek` only needs the key to be 32 bytes, so an all-zero one keeps
        // this test off the Argon2 path it is not about.
        let (wrapped, _) = crypto::wrap_dek(&crypto::generate_dek(), &[0u8; 32]).unwrap();
        assert_eq!(48, wrapped.len(), "32 byte DEK plus the 16 byte tag");
    }

    #[test]
    fn accepts_the_master_row_the_setup_command_writes() {
        let conn = in_memory();
        let salt = vec![0u8; 16];
        let wrapped = vec![0u8; 32 + TAG_LEN];
        let nonce = vec![0u8; 12];
        insert_master(&conn, &salt, &wrapped, &nonce).expect("the shapes setup writes must fit");
    }

    /// These three columns are handed to the crypto module untouched, and
    /// `Nonce::from_slice` asserts the length instead of checking it. A row that
    /// reached the database with a short value would abort the process, so the
    /// constraint is what keeps it out.
    #[test]
    fn rejects_a_master_row_whose_blobs_are_the_wrong_length() {
        let conn = in_memory();
        let salt = vec![0u8; 16];
        let wrapped = vec![0u8; 32 + TAG_LEN];
        let nonce = vec![0u8; 12];

        for bad_salt in [0usize, 1, 15, 17, 32] {
            assert!(
                insert_master(&conn, &vec![0u8; bad_salt], &wrapped, &nonce).is_err(),
                "a {bad_salt}-byte salt must be rejected"
            );
        }
        for bad_wrapped in [0usize, 1, 32, 47, 49, 64] {
            assert!(
                insert_master(&conn, &salt, &vec![0u8; bad_wrapped], &nonce).is_err(),
                "a {bad_wrapped}-byte wrapped dek must be rejected"
            );
        }
        for bad_nonce in [0usize, 1, 8, 11, 13, 16] {
            assert!(
                insert_master(&conn, &salt, &wrapped, &vec![0u8; bad_nonce]).is_err(),
                "a {bad_nonce}-byte dek nonce must be rejected"
            );
        }
    }

    #[test]
    fn accepts_a_password_row_whose_nonce_is_the_right_length() {
        let conn = in_memory();
        insert_password(&conn, "p1", &[0u8; 12]).unwrap();
    }

    #[test]
    fn rejects_a_password_row_whose_nonce_is_the_wrong_length() {
        let conn = in_memory();
        for bad in [0usize, 1, 11, 13, 16] {
            assert!(
                insert_password(&conn, &format!("p{bad}"), &vec![0u8; bad]).is_err(),
                "a {bad}-byte password nonce must be rejected"
            );
        }
    }

    /// Same column, same reason, on the other table: a short nonce there is what
    /// reaches `decrypt_note_by_id`, so it needs the same door closed.
    #[test]
    fn rejects_a_note_row_whose_nonce_is_the_wrong_length() {
        let conn = in_memory();
        for bad in [0usize, 1, 11, 13, 16] {
            let id = format!("n{bad}");
            assert!(
                conn.execute(
                    "INSERT INTO notes (id, title, encrypted_content, nonce, created_at, updated_at) \
                     VALUES (?1, 't', ?2, ?3, '1', '1')",
                    rusqlite::params![id, vec![0u8; 1 + TAG_LEN], vec![0u8; bad]],
                )
                .is_err(),
                "a {bad}-byte note nonce must be rejected"
            );
        }
    }

    #[test]
    fn accepts_a_note_row_whose_nonce_is_the_right_length() {
        // Every other note test already goes through `insert_note`, which writes
        // `zeroblob(12)`, so this pins that fixture to the real shape rather than
        // to whatever the constraint happens to accept.
        let conn = in_memory();
        insert_note(&conn, "n1", &[0u8; 1 + TAG_LEN], None).unwrap();
    }

    #[test]
    fn accepts_the_longest_body_the_constraint_allows() {
        let conn = in_memory();
        // 256 characters of 4 UTF-8 bytes each, the worst case the limit assumes.
        insert_note(&conn, "max", &vec![0u8; 256 * 4 + TAG_LEN], Some("#fff740")).unwrap();
    }

    #[test]
    fn accepts_a_short_body() {
        let conn = in_memory();
        insert_note(&conn, "min", &[0u8; 1 + TAG_LEN], None).unwrap();
    }

    #[test]
    fn rejects_a_body_one_byte_over_the_limit() {
        let conn = in_memory();
        let over = 256 * 4 + TAG_LEN + 1;
        let result = insert_note(&conn, "over", &vec![0u8; over], None);
        assert!(result.is_err(), "a 257-character body must not fit");
    }

    #[test]
    fn rejects_a_body_with_no_plaintext_behind_the_tag() {
        let conn = in_memory();
        assert!(
            insert_note(&conn, "empty", &[0u8; TAG_LEN], None).is_err(),
            "a tag with no ciphertext behind it is not a body"
        );
    }

    #[test]
    fn accepts_both_colour_cases_and_null_on_notes() {
        let conn = in_memory();
        for (i, color) in [Some("#fff740"), Some("#FFF740"), None].iter().enumerate() {
            insert_note(&conn, &format!("n{i}"), &[0u8; 1 + TAG_LEN], *color).unwrap();
        }
    }

    #[test]
    fn accepts_both_colour_cases_and_null_on_categories() {
        let conn = in_memory();
        for (i, color) in [Some("#3ee0cf"), Some("#3EE0CF"), None].iter().enumerate() {
            insert_category(&conn, &format!("c{i}"), *color).unwrap();
        }
    }

    /// The colour reaches a CSS property, so the constraint is the last thing
    /// stopping a value that breaks out of the declaration. The Rust side rejects
    /// these too, but the constraint has to stand on its own for a writer that
    /// forgets to call `color::normalize`.
    #[test]
    fn rejects_colours_that_are_not_a_hex_triple() {
        let conn = in_memory();
        let bad = [
            "red",
            "#fff",
            "#22c55e00",
            "#22c55e; color: red",
            "#FFF740; background: url(x)",
            "#gggggg",
            "fff740 ",
            "0x22c55e",
        ];
        for (i, color) in bad.iter().enumerate() {
            assert!(
                insert_category(&conn, &format!("cat{i}"), Some(color)).is_err(),
                "category colour {color:?} must be rejected"
            );
            assert!(
                insert_note(&conn, &format!("note{i}"), &[0u8; 1 + TAG_LEN], Some(color)).is_err(),
                "note colour {color:?} must be rejected"
            );
        }
    }

    /// `ON DELETE SET NULL` is what keeps a note reachable after its category is
    /// gone: the note survives with a null reference instead of being cascaded
    /// away or left pointing at nothing.
    ///
    /// Nothing in the app runs `PRAGMA foreign_keys = ON`, so this only works
    /// because the bundled SQLite enables enforcement by default. The first assert
    /// pins that, so if dropping the `bundled` feature or a dependency bump ever
    /// turns enforcement off, this fails with a reason instead of quietly leaving
    /// dangling `category_id` values behind.
    #[test]
    fn a_deleted_category_clears_the_reference_without_cascading() {
        let conn = in_memory();

        let enforcement: i64 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(
            enforcement, 1,
            "foreign key enforcement is off, so ON DELETE SET NULL is inert"
        );

        insert_category(&conn, "c1", None).unwrap();
        conn.execute(
            "INSERT INTO notes (id, title, encrypted_content, nonce, category_id, created_at, updated_at) \
             VALUES ('n2', 't', ?1, zeroblob(12), 'c1', '1', '1')",
            [vec![0u8; 1 + TAG_LEN]],
        )
        .unwrap();

        conn.execute("DELETE FROM categories WHERE id = 'c1'", [])
            .unwrap();

        let reference: Option<String> = conn
            .query_row("SELECT category_id FROM notes WHERE id = 'n2'", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(reference, None, "the reference should be cleared");

        let surviving: i64 = conn
            .query_row("SELECT COUNT(*) FROM notes WHERE id = 'n2'", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(surviving, 1, "the note itself must not be cascaded away");
    }

    mod session {
        use super::Session;
        use zeroize::Zeroizing;

        fn wrapped(byte: u8) -> Zeroizing<[u8; 32]> {
            Zeroizing::new([byte; 32])
        }

        #[test]
        fn a_fresh_session_is_locked() {
            let session = Session::new();
            assert!(session.dek().is_err());
            assert!(session.require_unlocked().is_err());
        }

        #[test]
        fn dek_is_rejected_until_unlocked() {
            let session = Session::new();
            let err = session.dek().unwrap_err();
            assert_eq!(err, "No active session. Unlock first.");
        }

        #[test]
        fn unlock_publishes_the_key() {
            let session = Session::new();
            session.unlock(wrapped(7));
            assert_eq!(*session.dek().unwrap(), [7u8; 32]);
            assert!(session.require_unlocked().is_ok());
        }

        #[test]
        fn lock_withdraws_the_key() {
            let session = Session::new();
            session.unlock(wrapped(7));
            session.lock();
            assert!(session.dek().is_err());
            assert!(session.require_unlocked().is_err());
        }

        #[test]
        fn unlocking_again_replaces_the_key() {
            let session = Session::new();
            session.unlock(wrapped(1));
            session.unlock(wrapped(2));
            assert_eq!(*session.dek().unwrap(), [2u8; 32]);
        }

        #[test]
        fn a_successful_unlock_can_be_locked_repeatedly() {
            let session = Session::new();
            for _ in 0..3 {
                session.unlock(wrapped(9));
                assert!(session.require_unlocked().is_ok());
                session.lock();
                assert!(session.require_unlocked().is_err());
            }
        }

        /// `dek()` hands back an owned copy so a command can encrypt without
        /// holding the mutex. That is deliberate, but it also means a caller can
        /// hold the key after `lock()`, so the test pins the guarantee that
        /// actually matters: the session stops publishing it.
        #[test]
        fn a_key_handed_out_before_lock_stays_valid_but_is_no_longer_published() {
            let session = Session::new();
            session.unlock(wrapped(3));
            let held = session.dek().unwrap();
            session.lock();
            assert_eq!(*held, [3u8; 32]);
            assert!(session.dek().is_err());
        }
    }
}
