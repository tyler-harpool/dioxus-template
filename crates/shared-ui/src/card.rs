use dioxus::prelude::*;
use dioxus_style::with_css;

/// A branded card component for grouping content.
#[with_css(style, "card.css")]
#[component]
pub fn Card(
    title: String,
    #[props(default)] subtitle: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        div { class: style::card,
            div { class: style::header,
                h3 { class: style::title, "{title}" }
                if let Some(sub) = subtitle {
                    p { class: style::subtitle, "{sub}" }
                }
            }
            div { class: style::body, {children} }
        }
    }
}
