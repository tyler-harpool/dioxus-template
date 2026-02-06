use dioxus::prelude::*;

mod routes;
use routes::Route;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
