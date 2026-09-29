//! Colours written to the database are interpolated into CSS properties by the
//! frontend, so every command that stores one goes through here first.
//!
//! Both `categories.color` and `notes.color` are plain text, because they are
//! display metadata a list is built from. That is also why the shape has to be
//! pinned down on the way in: see [`normalize`].

/// Checks a colour before it is stored, returning `None` for "no colour".
pub fn normalize(color: Option<String>) -> Result<Option<String>, String> {
    let Some(color) = color else {
        return Ok(None);
    };

    let trimmed = color.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let hex = trimmed.strip_prefix('#').unwrap_or(trimmed);
    let valid = hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit());

    if !valid {
        return Err("Color must be a hex value like #02abb2".to_string());
    }

    Ok(Some(format!("#{}", hex.to_ascii_lowercase())))
}
