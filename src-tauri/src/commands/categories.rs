use crate::color;
use crate::db::{
    check_max_chars, Category, DbConn, Session, CATEGORY_ICON_MAX_CHARS, CATEGORY_NAME_MAX_CHARS,
};
use tauri::State;
use uuid::Uuid;

/// Checks the two free-text columns `add_category` and `update_category` write.
fn check_metadata(name: &str, icon: Option<&str>) -> Result<(), String> {
    check_max_chars("Name", name, CATEGORY_NAME_MAX_CHARS)?;
    if let Some(icon) = icon {
        check_max_chars("Icon", icon, CATEGORY_ICON_MAX_CHARS)?;
    }

    Ok(())
}

#[tauri::command]
pub fn add_category(
    state: State<'_, DbConn>,
    session: State<'_, Session>,
    name: String,
    icon: Option<String>,
    color: Option<String>,
) -> Result<Category, String> {
    session.require_unlocked()?;
    check_metadata(&name, icon.as_deref())?;

    let conn = state.0.lock().unwrap();
    let id = Uuid::new_v4().to_string();
    let color = color::normalize(color)?;

    conn.execute(
        "INSERT INTO categories (id, name, icon, color) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![id, name, icon, color],
    )
    .map_err(|e| e.to_string())?;

    Ok(Category {
        id,
        name,
        icon,
        color,
    })
}

#[tauri::command]
pub fn get_categories(
    state: State<'_, DbConn>,
    session: State<'_, Session>,
) -> Result<Vec<Category>, String> {
    session.require_unlocked()?;
    let conn = state.0.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name, icon, color FROM categories ORDER BY name ASC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                icon: row.get(2)?,
                color: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

#[tauri::command]
pub fn update_category(
    state: State<'_, DbConn>,
    session: State<'_, Session>,
    id: String,
    name: String,
    icon: Option<String>,
    color: Option<String>,
) -> Result<(), String> {
    session.require_unlocked()?;
    check_metadata(&name, icon.as_deref())?;

    let conn = state.0.lock().unwrap();
    let color = color::normalize(color)?;
    conn.execute(
        "UPDATE categories SET name = ?1, icon = ?2, color = ?3 WHERE id = ?4",
        rusqlite::params![name, icon, color, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_category(
    state: State<'_, DbConn>,
    session: State<'_, Session>,
    id: String,
) -> Result<(), String> {
    session.require_unlocked()?;
    let conn = state.0.lock().unwrap();
    conn.execute(
        "DELETE FROM categories WHERE id = ?1",
        rusqlite::params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
