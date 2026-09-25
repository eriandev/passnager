mod commands;
mod crypto;
mod db;

use db::init_db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let _ = init_db(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::clipboard::copy_to_clipboard,
            commands::master::is_master_configured,
            commands::master::setup_master_password,
            commands::master::verify_master_password,
            commands::master::change_master_password,
            commands::passwords::add_password,
            commands::passwords::get_passwords,
            commands::passwords::update_password,
            commands::passwords::delete_password,
            commands::passwords::decrypt_password_by_id,
            commands::settings::get_app_name,
            commands::settings::get_app_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
