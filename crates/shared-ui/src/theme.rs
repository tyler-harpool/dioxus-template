use dioxus::prelude::*;

/// Available themes for the application.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Theme {
    #[default]
    Cyberpunk,
    Light,
}

impl Theme {
    /// CSS attribute value for the data-theme attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Cyberpunk => "cyberpunk",
            Theme::Light => "light",
        }
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
    fn theme_default_is_cyberpunk() {
        assert_eq!(Theme::default(), Theme::Cyberpunk);
    }

    #[test]
    fn theme_as_str_values() {
        assert_eq!(Theme::Cyberpunk.as_str(), "cyberpunk");
        assert_eq!(Theme::Light.as_str(), "light");
    }
}
