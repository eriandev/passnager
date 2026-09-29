use crate::color;
use crate::db::{Category, DbConn};
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn add_category(
    state: State<'_, DbConn>,
    name: String,
    icon: Option<String>,
    color: Option<String>,
) -> Result<Category, String> {
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
pub fn get_categories(state: State<'_, DbConn>) -> Result<Vec<Category>, String> {
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
    id: String,
    name: String,
    icon: Option<String>,
    color: Option<String>,
) -> Result<(), String> {
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
pub fn delete_category(state: State<'_, DbConn>, id: String) -> Result<(), String> {
    let conn = state.0.lock().unwrap();
    conn.execute(
        "DELETE FROM categories WHERE id = ?1",
        rusqlite::params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
