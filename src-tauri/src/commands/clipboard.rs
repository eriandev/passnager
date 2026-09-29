use crate::db::Session;
use arboard::Clipboard;
use tauri::State;

#[tauri::command]
pub fn copy_to_clipboard(session: State<'_, Session>, text: String) -> Result<(), String> {
    session.require_unlocked()?;

    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(text).map_err(|e| e.to_string())?;
    Ok(())
}
