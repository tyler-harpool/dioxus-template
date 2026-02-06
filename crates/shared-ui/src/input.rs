use dioxus::prelude::*;
use dioxus_style::with_css;

/// A branded text input component.
#[with_css(style, "input.css")]
#[component]
pub fn TextInput(
    value: String,
    on_input: EventHandler<FormEvent>,
    #[props(default = String::new())] placeholder: String,
    #[props(default = String::new())] label: String,
    #[props(default = false)] disabled: bool,
) -> Element {
    rsx! {
        div { class: style::field,
            if !label.is_empty() {
                label { class: style::label, "{label}" }
            }
            input {
                class: style::input,
                r#type: "text",
                value: value,
                placeholder: placeholder,
                disabled: disabled,
                oninput: move |e| on_input.call(e),
            }
        }
    }
}
