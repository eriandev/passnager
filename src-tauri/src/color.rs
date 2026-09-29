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

#[cfg(test)]
mod tests {
    use super::normalize;

    fn ok(color: &str) -> String {
        normalize(Some(color.to_string())).unwrap().unwrap()
    }

    fn err(color: &str) -> String {
        normalize(Some(color.to_string())).unwrap_err()
    }

    #[test]
    fn an_absent_or_blank_colour_stays_absent() {
        assert_eq!(normalize(None).unwrap(), None);
        for blank in ["", "   ", "\t", "\n  "] {
            assert_eq!(
                normalize(Some(blank.to_string())).unwrap(),
                None,
                "{blank:?} should mean no colour"
            );
        }
    }

    #[test]
    fn a_hash_is_added_when_it_is_missing() {
        assert_eq!(ok("fff740"), "#fff740");
        assert_eq!(ok("#fff740"), "#fff740");
    }

    #[test]
    fn surrounding_whitespace_is_trimmed() {
        assert_eq!(ok("  #fff740 "), "#fff740");
        // Whitespace around a value is a paste artefact, not an attempt at
        // something, so trimming it beats rejecting it. Trimming also cannot
        // smuggle anything into the CSS: what reaches the style attribute is
        // rebuilt as `#` plus six validated hex digits.
        assert_eq!(ok("fff740 "), "#fff740");
        assert_eq!(ok("\t#3ee0cf\n"), "#3ee0cf");
    }

    /// The native colour input emits lowercase and so do the defaults in
    /// `src/lib/consts.ts`, so lowercase is the only case the app should ever
    /// store. Canonicalising here is what makes a picked colour and a typed one
    /// the same value in the database.
    #[test]
    fn stored_output_is_always_lowercase() {
        assert_eq!(ok("#FFF740"), "#fff740");
        assert_eq!(ok("#FfF740"), "#fff740");
        assert_eq!(ok("FFFFFF"), "#ffffff");
        assert_eq!(ok("#AbCdEf"), "#abcdef");
    }

    #[test]
    fn digits_and_both_hex_letters_are_accepted() {
        for valid in ["#000000", "#ffffff", "#012345", "#abcdef", "#ABCDEF"] {
            assert!(
                normalize(Some(valid.to_string())).is_ok(),
                "{valid:?} is a valid hex triple"
            );
        }
    }

    #[test]
    fn anything_that_is_not_a_six_digit_triple_is_rejected() {
        for bad in [
            "red",
            "#fff",
            "#ffff",
            "#fffffff",
            "#22c55e00",
            "#gggggg",
            "#",
            "0x22c55e",
            "rgb(1,2,3)",
            "hsl(0,0%,0%)",
        ] {
            assert_eq!(
                err(bad),
                "Color must be a hex value like #02abb2",
                "{bad:?} should be rejected"
            );
        }
    }

    /// The value ends up inside a CSS declaration, so anything that closes the
    /// declaration and starts a new one has to be refused rather than sanitised.
    #[test]
    fn a_value_that_breaks_out_of_the_css_is_rejected() {
        for hostile in [
            "#22c55e; color: red",
            "#fff740; background: url(https://evil.test)",
            "#fff740} body{display:none",
            "red; }",
            "#fff740\n; }",
        ] {
            assert!(err(hostile).contains("hex value"), "{hostile:?}");
        }
    }

    #[test]
    fn the_error_is_the_same_one_the_schema_backs_up() {
        // The constraint on each table rejects the same values, but it can only
        // report a bare SQLite string. Keeping the message in one place means the
        // two cannot drift apart.
        assert_eq!(
            err("nope"),
            super::normalize(Some("nope".into())).unwrap_err()
        );
        assert_eq!(err("nope"), "Color must be a hex value like #02abb2");
    }
}
