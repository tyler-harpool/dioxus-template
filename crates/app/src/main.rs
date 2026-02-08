use dioxus::prelude::*;

mod auth;
mod routes;
pub mod tier_gate;
use auth::{use_auth, AuthState};
use routes::Route;

/// Shared profile state accessible across all routes.
/// Backed by `Memo`s that read directly from `AuthState` — always in sync.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProfileState {
    pub display_name: Memo<String>,
    pub email: Memo<String>,
    pub avatar_url: Memo<Option<String>>,
}

const CYBERPUNK_THEME: Asset = asset!("/assets/cyberpunk-theme.css");

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        server::telemetry::init_telemetry();
        server::health::record_start_time();

        let pool = server::db::create_pool();
        server::db::run_migrations(&pool).await;
        server::s3::ensure_bucket().await;
        let state = server::db::AppState { pool: pool.clone() };

        let router = dioxus::server::router(App)
            .merge(server::openapi::api_router(pool))
            .layer(server::telemetry::OtelTraceLayer)
            .layer(axum::middleware::from_fn_with_state(
                state,
                server::auth::middleware::auth_middleware,
            ))
            .layer(tower_http::request_id::PropagateRequestIdLayer::x_request_id())
            .layer(tower_http::request_id::SetRequestIdLayer::x_request_id(
                tower_http::request_id::MakeRequestUuid,
            ));
        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

/// Detect the client platform from compile-time feature flags.
fn client_platform() -> &'static str {
    if cfg!(feature = "web") {
        "web"
    } else if cfg!(feature = "desktop") {
        "desktop"
    } else if cfg!(feature = "mobile") {
        "mobile"
    } else {
        "unknown"
    }
}

#[component]
fn App() -> Element {
    // Set the X-Client-Platform header on all server function calls
    use_hook(|| {
        use dioxus::fullstack::{set_request_headers, HeaderMap, HeaderValue};

        let mut headers = HeaderMap::new();
        headers.insert(
            "x-client-platform",
            HeaderValue::from_static(client_platform()),
        );
        set_request_headers(headers);
    });

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
    let avatar_url = use_memo(move || {
        auth.current_user
            .read()
            .as_ref()
            .and_then(|u| u.avatar_url.clone())
    });

    use_context_provider(|| ProfileState {
        display_name,
        email,
        avatar_url,
    });

    rsx! {
        document::Link { rel: "stylesheet", href: CYBERPUNK_THEME }
        shared_ui::theme::ThemeSeed {}
        shared_ui::ToastProvider {
            Router::<Route> {}
        }
    }
}
