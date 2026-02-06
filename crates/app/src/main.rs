use dioxus::prelude::*;

mod routes;
use routes::Route;

const GLOBAL_CSS: Asset = asset!("/assets/variables.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: GLOBAL_CSS }
        Router::<Route> {}
    }
}
