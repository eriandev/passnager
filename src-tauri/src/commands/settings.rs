use tauri::AppHandle;

#[tauri::command]
pub fn get_app_name(app: AppHandle) -> Result<String, String> {
    Ok(app.package_info().name.to_string())
}

#[tauri::command]
pub fn get_app_version(app: AppHandle) -> Result<String, String> {
    Ok(app.package_info().version.to_string())
}
