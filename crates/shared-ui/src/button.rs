use dioxus::prelude::*;
use dioxus_style::with_css;
use shared_types::ButtonVariant;

/// A branded button component with variant styling.
#[with_css(style, "button.css")]
#[component]
pub fn Button(
    label: String,
    #[props(default = ButtonVariant::Primary)] variant: ButtonVariant,
    on_click: EventHandler<MouseEvent>,
    #[props(default = false)] disabled: bool,
) -> Element {
    let variant_class = match variant {
        ButtonVariant::Primary => style::primary,
        ButtonVariant::Secondary => style::secondary,
        ButtonVariant::Danger => style::danger,
    };

    rsx! {
        button {
            class: style::btn + variant_class,
            onclick: move |e| on_click.call(e),
            disabled: disabled,
            "{label}"
        }
    }
}
