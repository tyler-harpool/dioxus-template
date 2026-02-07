use dioxus::prelude::*;

mod routes;
use routes::Route;

/// Shared profile state accessible across all routes.
#[derive(Clone, Debug, PartialEq)]
pub struct ProfileState {
    pub display_name: Signal<String>,
    pub email: Signal<String>,
}

const CYBERPUNK_THEME: Asset = asset!("/assets/cyberpunk-theme.css");

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        let router = dioxus::server::router(App).merge(server::openapi::api_router());
        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| ProfileState {
        display_name: Signal::new("Admin User".to_string()),
        email: Signal::new("admin@cyberapp.io".to_string()),
    });

    rsx! {
        document::Link { rel: "stylesheet", href: CYBERPUNK_THEME }
        shared_ui::theme::ThemeSeed {}
        shared_ui::ToastProvider {
            Router::<Route> {}
        }
    }
}
