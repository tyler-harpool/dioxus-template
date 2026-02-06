/// Design token constants for use in Rust code when CSS variables aren't accessible.
///
/// These values mirror the CSS custom properties in `variables.css`.
/// Prefer using CSS variables in component stylesheets — use these constants
/// only for programmatic styling or platform-specific rendering.
/// Dark theme is the default.
pub const DEFAULT_THEME: &str = "dark";

/// Toggle between light and dark themes.
pub fn toggle_theme(current: &str) -> &str {
    if current == "dark" {
        "light"
    } else {
        "dark"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_from_dark_to_light() {
        assert_eq!(toggle_theme("dark"), "light");
    }

    #[test]
    fn toggle_from_light_to_dark() {
        assert_eq!(toggle_theme("light"), "dark");
    }
}
