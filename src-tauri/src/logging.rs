//! The app's logger.
//!
//! Every entry carries a timestamp and the call site, so a line in the log can be
//! found in the source without guessing.
//!
//! Usage:
//!
//! ```text
//! use crate::logging::{log_error, log_info, log_warn};
//!
//! log_info!("vault opened");
//! log_warn!("category {id} was empty");
//! log_error!("could not open the vault: {reason}");
//! ```

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// The name of the log file, inside whichever directory the app configures with
/// [`set_dir`].
///
/// Deliberately a bare name and not a path. This module is handed a directory and
/// owns the decision of what the file inside it is called, so that changing the
/// name, adding a date, or putting logs in a `logs/` subdirectory never reaches
/// the callers.
const DEFAULT_FILE: &str = "passnager.log";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Error,
    // No caller yet. The logger is meant to outlive the one module that uses it,
    // and a level invented later lands with no evidence that anyone agreed on its
    // name or its position. Kept deliberately.
    #[allow(dead_code)]
    Warn,
    Info,
}

impl Level {
    /// Padded to a common width so the messages line up when read down a file.
    fn label(self) -> &'static str {
        match self {
            Level::Error => "ERROR",
            Level::Warn => "WARN ",
            Level::Info => "INFO ",
        }
    }
}

/// The configured log file, or `None` for [`DEFAULT_FILE`]. A `RwLock` rather than
/// a `OnceLock` because a logger that cannot be re-pointed is a nuisance the first
/// time a test or a future "move the vault" feature needs it.
static FILE: RwLock<Option<PathBuf>> = RwLock::new(None);

/// Points the logger at a file, creating the parent directory if needed.
///
/// Called once from `setup` before anything that can fail, so a startup abort
/// still has somewhere to land. Returns the error instead of logging it, since
/// this is what decides where logging goes.
pub fn set_file(path: impl Into<PathBuf>) -> std::io::Result<()> {
    let path = path.into();
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    // A poisoned lock would mean some other thread panicked while holding it.
    // Recovering the inner value is strictly better than a second panic: this
    // function exists so that logging cannot be the reason something fails.
    *FILE
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(path);
    Ok(())
}

/// Points the logger at a directory, where the log will be [`DEFAULT_FILE`].
///
/// The intended entry point for the app: the caller says where the app's other
/// files live, and this module owns the log's name. That split is what lets the
/// file be renamed, dated, or moved into a `logs/` subdirectory later without
/// reaching the callers.
pub fn set_dir(dir: impl Into<PathBuf>) -> std::io::Result<()> {
    let dir: PathBuf = dir.into();
    set_file(dir.join(DEFAULT_FILE))
}

/// The file the logger is currently writing to.
///
/// With no directory configured this is a bare name in the process's current
/// working directory. That is not a good place for a log — for a GUI app the
/// working directory is set by whoever launched it, and may not even be
/// writable — but it is better than refusing to log, which would leave a failed
/// startup with no explanation at all.
pub fn log_file() -> PathBuf {
    FILE.read()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone()
        .unwrap_or_else(|| PathBuf::from(DEFAULT_FILE))
}

/// Logs one entry, wherever this build is configured to put it.
pub fn record(level: Level, message: &str, source: &str, line: u32) {
    emit(
        &format_entry(level, message, source, line),
        cfg!(debug_assertions),
    );
}

/// The one place that decides where an entry goes.
///
/// `to_console` is a parameter rather than a `cfg!` read here so that both
/// branches stay reachable from a test. `cfg!(debug_assertions)` is fixed when the
/// test binary is compiled, so a normal `cargo test` run would never touch the
/// release branch — which is the one that writes the file a user actually has to
/// read when reporting a bug.
fn emit(entry: &str, to_console: bool) {
    if to_console {
        eprintln!("{entry}");
    } else {
        append_to(&log_file(), entry);
    }
}

/// One entry, with the metadata a bug report needs: when, how bad, and where.
fn format_entry(level: Level, message: &str, source: &str, line: u32) -> String {
    format!(
        "[{}] {} {message} ({source}:{line})",
        timestamp(),
        level.label()
    )
}

/// A local timestamp, because a report saying "it happened at 03:12" is only
/// useful if 03:12 is the reporter's 03:12. Falls back to UTC when the system has
/// no timezone configured.
fn timestamp() -> String {
    chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S%.3f")
        .to_string()
}

/// Best-effort append. A log that cannot be written must never be the reason the
/// thing being logged fails.
fn append_to(path: &Path, entry: &str) {
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(file, "{entry}");
    }
}

macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logging::record($crate::logging::Level::Error, &format!($($arg)*), file!(), line!())
    };
}

// See the note on `Level::Warn`: no caller yet, on purpose.
#[allow(unused_macros)]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logging::record($crate::logging::Level::Warn, &format!($($arg)*), file!(), line!())
    };
}

macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logging::record($crate::logging::Level::Info, &format!($($arg)*), file!(), line!())
    };
}

pub(crate) use log_error;
pub(crate) use log_info;
#[allow(unused_imports)]
pub(crate) use log_warn;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// `set_file` is process-wide and `cargo test` runs these in parallel, so the
    /// tests that use it take turns. Without this they would point the logger at
    /// each other's files and fail at random.
    static LOG_FILE: Mutex<()> = Mutex::new(());

    /// A scratch directory that cleans itself up, so the real config dir and the
    /// working directory are never involved.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!("passnager-log-{name}"));
            let _ = std::fs::remove_dir_all(&path);
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn an_entry_carries_a_timestamp_a_level_the_message_and_the_call_site() {
        let entry = format_entry(Level::Error, "could not open vault", "src/db.rs", 174);

        assert!(entry.contains("ERROR"), "got {entry:?}");
        assert!(entry.contains("could not open vault"), "got {entry:?}");
        assert!(entry.ends_with("(src/db.rs:174)"), "got {entry:?}");
    }

    /// The timestamp is meant to be read by a person, so it has to be a date, not
    /// an epoch. A shape check rather than an exact value: asserting on the
    /// current second would make the test flaky.
    #[test]
    fn the_timestamp_is_readable_rather_than_an_epoch() {
        let entry = format_entry(Level::Info, "vault opened", "src/startup.rs", 1);
        let stamp = &entry[1..entry.find(']').unwrap()];

        assert!(
            chrono::NaiveDateTime::parse_from_str(stamp, "%Y-%m-%d %H:%M:%S%.3f").is_ok(),
            "not a readable timestamp: {stamp:?}"
        );
    }

    /// Same width for every level, so the messages line up down the file.
    #[test]
    fn every_level_is_the_same_width() {
        let widths = [Level::Error, Level::Warn, Level::Info].map(|l| l.label().len());
        assert_eq!(widths, [5, 5, 5]);
    }

    #[test]
    fn the_log_file_defaults_to_the_documented_name() {
        assert_eq!(PathBuf::from(DEFAULT_FILE), PathBuf::from("passnager.log"));
    }

    /// The only test that touches the global, so nothing races it.
    #[test]
    fn the_configured_file_is_remembered_and_its_directory_created() {
        let _guard = LOG_FILE.lock().unwrap_or_else(|p| p.into_inner());
        let dir = TempDir::new("set-file");
        let nested = dir.0.join("deeper").join("still").join("app.log");
        assert!(!nested.parent().unwrap().exists());

        set_file(&nested).unwrap();

        assert_eq!(log_file(), nested);
        assert!(nested.parent().unwrap().is_dir());
    }

    /// A caller that knows where the app's files live should not have to know what
    /// the log is called. This is the case that lets `DEFAULT_FILE` stay private to
    /// this module: `set_dir` takes a folder and the name is added here, so nothing
    /// outside has to repeat it and nothing outside breaks if it changes.
    #[test]
    fn a_directory_becomes_the_directory_with_the_default_name() {
        let _guard = LOG_FILE.lock().unwrap_or_else(|p| p.into_inner());
        let dir = TempDir::new("set-dir");

        set_dir(&dir.0).unwrap();

        assert_eq!(log_file(), dir.0.join(DEFAULT_FILE));
        assert!(dir.0.is_dir());
    }

    #[test]
    fn entries_land_in_the_file() {
        let dir = TempDir::new("writes");
        let log = dir.0.join(DEFAULT_FILE);

        append_to(&log, "first");
        append_to(&log, "second");

        let contents = std::fs::read_to_string(&log).unwrap();
        assert!(contents.contains("first"), "got {contents:?}");
        assert!(contents.contains("second"), "got {contents:?}");
        assert_eq!(
            contents.lines().count(),
            2,
            "later entries must not clobber"
        );
    }

    /// Best-effort means best-effort: an unwritable path must not panic, because
    /// this is the path taken when the filesystem says no and the caller still has
    /// real work to finish.
    #[test]
    fn an_unwritable_path_is_ignored_rather_than_panicking() {
        let dir = TempDir::new("unwritable");
        let blocker = dir.0.join("blocker");
        std::fs::write(&blocker, b"not a directory").unwrap();

        append_to(&blocker.join("nested.log"), "boom");
    }

    /// The release branch, which is the one that matters to a user filing a bug
    /// report: the entry has to reach the file and nowhere else.
    #[test]
    fn a_release_build_writes_the_entry_to_the_log_file() {
        let _guard = LOG_FILE.lock().unwrap_or_else(|p| p.into_inner());
        let dir = TempDir::new("release-file");
        let log = dir.0.join(DEFAULT_FILE);
        set_file(&log).unwrap();

        emit("could not open the vault (src/startup.rs:74)", false);

        let contents = std::fs::read_to_string(&log).unwrap();
        assert_eq!(contents.lines().count(), 1, "got {contents:?}");
        assert!(
            contents.contains("could not open the vault"),
            "got {contents:?}"
        );
        assert!(contents.contains("src/startup.rs:74"), "got {contents:?}");
    }

    /// The dev branch: printed, and deliberately no file. A dev build that also
    /// wrote a file would litter the config dir and train the developer to ignore
    /// the one place the output really is.
    #[test]
    fn a_dev_build_prints_instead_of_creating_a_file() {
        let _guard = LOG_FILE.lock().unwrap_or_else(|p| p.into_inner());
        let dir = TempDir::new("dev-console");
        let log = dir.0.join(DEFAULT_FILE);
        set_file(&log).unwrap();

        emit("vault opened (src/startup.rs:20)", true);

        assert!(
            !log.exists(),
            "a dev build must print, not write {}",
            log.display()
        );
    }

    /// The whole point of the macros is that the caller does not have to name the
    /// file and the line, so that is the part that has to be proven rather than
    /// assumed: `file!()` has to resolve to a path relative to the crate root.
    ///
    /// An absolute path would still "work", but it would be useless in a bug
    /// report — the reader does not have this machine's directory layout — and it
    /// would vary between builds, so the same failure would not grep the same way
    /// twice. This is the check that would catch a change in how Cargo hands
    /// source paths to `file!()`.
    ///
    /// The macro is expanded here rather than in a caller so the expected value
    /// stays in one file with the assertion; `log_info!` itself is exercised for
    /// real in the next test.
    #[test]
    fn the_call_site_is_relative_to_the_crate_root() {
        // Bound on one line on purpose: `file!()` and `line!()` report the line
        // they are *written* on, so reading them again at the assertion below
        // would compare the entry against a different line number and always fail.
        let (here, line) = (file!(), line!());
        let entry = format_entry(Level::Info, "vault opened", here, line);

        assert!(
            entry.ends_with(&format!("({here}:{line})")),
            "the call site should close the entry, got {entry:?}"
        );
        assert!(
            !here.starts_with('/'),
            "file!() gave an absolute path, which is useless in a report: {here}"
        );
        assert!(
            here.starts_with("src/"),
            "file!() should be crate-relative, got {here}"
        );
        assert!(
            here.ends_with("logging.rs"),
            "expected the test's own file, got {here}"
        );
    }

    /// Calls the macros for real, so the test above cannot pass while the macros
    /// themselves are broken. In a debug test build the console branch is taken, so
    /// this asserts only that the expansion compiles, runs and does not panic:
    /// `record` returns `()` and writes to stderr.
    #[test]
    fn the_macros_expand_and_run() {
        log_info!("vault opened");
        log_error!("could not open the vault: {reason}", reason = "disk full");
        log_warn!("category was empty");
    }
}
