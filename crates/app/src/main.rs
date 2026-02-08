use dioxus::prelude::*;

mod auth;
mod routes;
pub mod tier_gate;
use auth::{use_auth, AuthState};
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
        server::telemetry::init_telemetry();
        server::health::record_start_time();

        let router = dioxus::server::router(App)
            .merge(server::openapi::api_router())
            .layer(axum::middleware::from_fn(
                server::auth::middleware::auth_middleware,
            ))
            .layer(server::telemetry::OtelTraceLayer)
            .layer(tower_http::request_id::PropagateRequestIdLayer::x_request_id())
            .layer(tower_http::request_id::SetRequestIdLayer::x_request_id(
                tower_http::request_id::MakeRequestUuid,
            ));
        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(AuthState::new);
    auth::use_auth_init();

    // Derive profile state from auth — updates when user logs in/out
    let auth = use_auth();
    let display_name = use_memo(move || {
        auth.current_user
            .read()
            .as_ref()
            .map(|u| u.display_name.clone())
            .unwrap_or_else(|| "Guest".to_string())
    });
    let email = use_memo(move || {
        auth.current_user
            .read()
            .as_ref()
            .map(|u| u.email.clone())
            .unwrap_or_else(|| "guest@cyberapp.io".to_string())
    });

    use_context_provider(|| ProfileState {
        display_name: Signal::new(display_name()),
        email: Signal::new(email()),
    });

    // Keep profile in sync when auth changes
    let mut profile: ProfileState = use_context();
    use_effect(move || {
        profile.display_name.set(display_name());
        profile.email.set(email());
    });

    rsx! {
        document::Link { rel: "stylesheet", href: CYBERPUNK_THEME }
        shared_ui::theme::ThemeSeed {}
        shared_ui::ToastProvider {
            Router::<Route> {}
        }
    }
}
