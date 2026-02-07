use dioxus::prelude::*;

/// Theme families available in the application.
///
/// Each family provides both a dark and light variant.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ThemeFamily {
    #[default]
    Cyberpunk,
    Solar,
}

impl ThemeFamily {
    /// Internal key used for storage and Select values.
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeFamily::Cyberpunk => "cyberpunk",
            ThemeFamily::Solar => "solar",
        }
    }

    /// Human-readable name for display in UI.
    pub fn display_name(&self) -> &'static str {
        match self {
            ThemeFamily::Cyberpunk => "Cyberpunk",
            ThemeFamily::Solar => "Solarized",
        }
    }

    /// Parse a family key string, falling back to Cyberpunk.
    pub fn from_key(s: &str) -> Self {
        match s {
            "solar" => ThemeFamily::Solar,
            _ => ThemeFamily::Cyberpunk,
        }
    }

    /// Resolve to the CSS `data-theme` attribute value.
    pub fn resolve(&self, is_dark: bool) -> &'static str {
        match (self, is_dark) {
            (ThemeFamily::Cyberpunk, true) => "cyberpunk",
            (ThemeFamily::Cyberpunk, false) => "light",
            (ThemeFamily::Solar, true) => "solar",
            (ThemeFamily::Solar, false) => "solar-light",
        }
    }
}

/// Shared theme state provided as context.
///
/// Both the sidebar (dark/light toggle) and settings (family picker)
/// read and write these signals. Changes call [`set_theme`] to apply.
#[derive(Clone, Copy)]
pub struct ThemeState {
    pub family: Signal<String>,
    pub is_dark: Signal<bool>,
}

impl ThemeState {
    /// Apply the current family + mode to the document.
    pub fn apply(&self) {
        let family = ThemeFamily::from_key(&self.family.read());
        let theme = family.resolve(*self.is_dark.read());
        set_theme(theme);
    }
}

/// Seed the theme on application startup.
///
/// Reads the persisted theme from a cookie and applies it to the document root.
/// Call this once in your top-level App component.
#[component]
pub fn ThemeSeed() -> Element {
    use_effect(|| {
        // Read theme cookie and apply data-theme attribute to <html>
        document::eval(
            r#"
            (function() {
                var match = document.cookie.match(/(?:^|;\s*)theme=([^;]*)/);
                var theme = match ? match[1] : 'cyberpunk';
                document.documentElement.setAttribute('data-theme', theme);
            })();
            "#,
        );
    });

    rsx! {}
}

/// Set the active theme, persisting to a cookie and updating the document.
///
/// Uses BroadcastChannel to sync across tabs when available.
pub fn set_theme(theme: &str) {
    document::eval(&format!(
        r#"
        (function() {{
            document.cookie = 'theme={theme};path=/;max-age=2592000;SameSite=Lax';
            document.documentElement.setAttribute('data-theme', '{theme}');
            try {{
                var bc = new BroadcastChannel('theme-sync');
                bc.postMessage('{theme}');
                bc.close();
            }} catch(e) {{}}
        }})();
        "#,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_family_default_is_cyberpunk() {
        assert_eq!(ThemeFamily::default(), ThemeFamily::Cyberpunk);
    }

    #[test]
    fn theme_family_as_str_values() {
        assert_eq!(ThemeFamily::Cyberpunk.as_str(), "cyberpunk");
        assert_eq!(ThemeFamily::Solar.as_str(), "solar");
    }

    #[test]
    fn theme_family_from_key() {
        assert_eq!(ThemeFamily::from_key("cyberpunk"), ThemeFamily::Cyberpunk);
        assert_eq!(ThemeFamily::from_key("solar"), ThemeFamily::Solar);
        assert_eq!(ThemeFamily::from_key("unknown"), ThemeFamily::Cyberpunk);
    }

    #[test]
    fn theme_family_resolve_variants() {
        assert_eq!(ThemeFamily::Cyberpunk.resolve(true), "cyberpunk");
        assert_eq!(ThemeFamily::Cyberpunk.resolve(false), "light");
        assert_eq!(ThemeFamily::Solar.resolve(true), "solar");
        assert_eq!(ThemeFamily::Solar.resolve(false), "solar-light");
    }
}
