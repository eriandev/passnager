//! What happens when the app refuses to start.
//!
//! Launching the app means opening a vault. That can fail for reasons the user
//! cannot see and cannot guess — a file in the way, a directory that is not
//! writable, a database left half-written by a crash. When it does, the app
//! aborts and leaves one line behind in the log saying why.
//!
//! The counterpart is that nothing here is allowed to be the reason startup fails.
//! The log is best-effort, and a failure to open it must not turn "the vault
//! could not be opened" into "the vault could not be opened and the app panicked
//! somewhere else".

use tauri::AppHandle;

use crate::db::config_dir;
use crate::logging::{self, log_error, log_info};

/// Errors that abort startup. Carrying the message in a real `Error` is what lets
/// `setup` return `Err` and take the whole launch down instead of running with
/// half-initialised state.
#[derive(Debug)]
pub struct StartupError(pub String);

impl std::fmt::Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for StartupError {}

/// Points the logger at the config directory, next to the vault.
///
/// Called at the very top of `setup`, before the vault is touched. If it ran
/// afterwards, a failure to open the vault would be the one error with nowhere to
/// go, which is the exact case the log exists for.
pub fn init_logging(app: &AppHandle) {
    // Only the directory is passed. What the log is called is the logger's own
    // decision, so renaming it or moving it into a subdirectory later does not
    // reach this file.
    match logging::set_dir(config_dir(app)) {
        Ok(()) => log_info!("logging to {}", logging::log_file().display()),
        // Nothing to log to yet, so this one is genuinely worth a console write:
        // it means every later message is going somewhere the user cannot find.
        // `log_file()` names the fallback in use rather than repeating a name that
        // is no longer visible from here.
        Err(error) => eprintln!(
            "passnager: could not open a log file in the config directory, \
             logging to {} instead and some messages will be lost: {error}",
            logging::log_file().display()
        ),
    }
}

/// Records why the app refused to start.
///
/// The wording is deliberately careful about damage. The only work that can have
/// run by the time this is called is opening the file and the
/// `CREATE TABLE IF NOT EXISTS` statements, so no password, note or category has
/// been read or written and nothing an existing vault holds has been changed. The
/// message also never suggests deleting the vault: that would destroy a working
/// password manager to work around a launch problem that has nothing to do with
/// the data.
///
/// The block is a single entry rather than several, so a half-written abort notice
/// cannot be mistaken for a complete one.
///
/// The metadata is appended after the message rather than in front of it, so on a
/// multiline entry the timestamp lands on the first line and the `(file:line)` on
/// the last. Splitting the two across four lines is deliberate: it keeps the first
/// line of a log scannable with `grep`, and the source of the abort is where a
/// reader reaches last, after reading what it says.
pub fn log_startup_failure(app: &AppHandle, message: &str) {
    log_error!(
        "STARTUP ABORTED: {message}\n  \
        vault: {}\n  \
        No password, note or category was read or written, and the app stopped \
        before it could reach them. The vault itself has not been altered.\n  \
        Do not delete it to get past the problem. Copy it somewhere safe first, \
        then act on the reason above.",
        db_file_display(app)
    );
}

fn db_file_display(app: &AppHandle) -> String {
    crate::db::db_path(app).display().to_string()
}
