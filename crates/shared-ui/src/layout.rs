use dioxus::prelude::*;
use dioxus_style::with_css;

/// A page layout wrapper providing consistent structure.
#[with_css(style, "layout.css")]
#[component]
pub fn PageLayout(#[props(default = String::new())] title: String, children: Element) -> Element {
    rsx! {
        div { class: style::page,
            if !title.is_empty() {
                header { class: style::header,
                    h1 { class: style::title, "{title}" }
                }
            }
            main { class: style::content,
                {children}
            }
        }
    }
}
