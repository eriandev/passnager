mod color;
mod commands;
mod crypto;
mod db;
mod logging;
mod startup;

use db::init_db;
use startup::StartupError;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            // Before `init_db`, so that a vault that refuses to open is the one
            // failure that still gets written down.
            startup::init_logging(handle);

            // Propagated on purpose. `DbConn` and `Session` are registered at the
            // end of `init_db`, so carrying on after a failure would leave them
            // unmanaged: every command would then return a `State` error that the
            // frontend swallows, and the app would open looking like a fresh
            // install with a "Failed to configure master password" wall. Aborting
            // keeps one invariant: the app never runs against a vault it has not
            // finished opening.
            if let Err(error) = init_db(handle) {
                let message = error.to_string();
                startup::log_startup_failure(handle, &message);
                return Err(Box::new(StartupError(message)) as Box<dyn std::error::Error>);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::clipboard::copy_to_clipboard,
            commands::master::is_master_configured,
            commands::master::setup_master_password,
            commands::master::verify_master_password,
            commands::master::change_master_password,
            commands::master::lock_session,
            commands::passwords::add_password,
            commands::passwords::get_passwords,
            commands::passwords::update_password,
            commands::passwords::delete_password,
            commands::passwords::decrypt_password_by_id,
            commands::categories::add_category,
            commands::categories::get_categories,
            commands::categories::update_category,
            commands::categories::delete_category,
            commands::notes::add_note,
            commands::notes::get_notes,
            commands::notes::update_note,
            commands::notes::delete_note,
            commands::notes::decrypt_note_by_id,
            commands::settings::get_app_name,
            commands::settings::get_app_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
