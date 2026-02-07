use dioxus::prelude::*;

mod routes;
use routes::Route;

const GLOBAL_CSS: Asset = asset!("/assets/variables.css");

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
        document::Link { rel: "stylesheet", href: GLOBAL_CSS }
        Router::<Route> {}
    }
}
