use dioxus::prelude::*;

mod routes;
use routes::Route;

const CYBERPUNK_THEME: Asset = asset!("/assets/cyberpunk-theme.css");

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        let router = dioxus::server::router(App).merge(server::openapi::swagger_router());
        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CYBERPUNK_THEME }
        shared_ui::theme::ThemeSeed {}
        shared_ui::ToastProvider {
            Router::<Route> {}
        }
    }
}
