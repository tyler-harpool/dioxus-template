use crate::auth::use_auth;
use crate::routes::Route;
use dioxus::prelude::*;
use shared_ui::{
    Button, ButtonVariant, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
    Input, Label, Separator,
};

/// Register page with email/password and OAuth options.
#[component]
pub fn Register() -> Element {
    let mut auth = use_auth();
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut display_name = use_signal(String::new);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);

    // Redirect to dashboard if already authenticated
    if auth.is_authenticated() {
        navigator().push(Route::Dashboard {});
    }

    let handle_register = move |evt: FormEvent| async move {
        evt.prevent_default();
        loading.set(true);
        error_msg.set(None);

        match server::api::register(username(), email(), password(), display_name()).await {
            Ok(user) => {
                auth.set_user(user);
                navigator().push(Route::Dashboard {});
            }
            Err(e) => {
                error_msg.set(Some(e.to_string()));
            }
        }
        loading.set(false);
    };

    let handle_oauth = move |provider: &'static str| {
        move |_: MouseEvent| {
            let provider = provider.to_string();
            spawn(async move {
                match server::api::oauth_authorize_url(provider).await {
                    Ok(url) => {
                        navigator().push(NavigationTarget::<Route>::External(url));
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("OAuth error: {}", e)));
                    }
                }
            });
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./login.css") }

        div { class: "auth-page",
            Card {
                class: "auth-card",

                CardHeader {
                    CardTitle { "Create Account" }
                    CardDescription { "Join CyberApp to get started" }
                }

                CardContent {
                    if let Some(err) = error_msg() {
                        div { class: "auth-error", "{err}" }
                    }

                    // OAuth buttons
                    div { class: "auth-oauth-buttons",
                        Button {
                            variant: ButtonVariant::Outline,
                            class: "auth-oauth-btn",
                            onclick: handle_oauth("google"),
                            "Continue with Google"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            class: "auth-oauth-btn",
                            onclick: handle_oauth("github"),
                            "Continue with GitHub"
                        }
                    }

                    // Divider
                    div { class: "auth-divider",
                        Separator {}
                        span { class: "auth-divider-text", "or" }
                        Separator {}
                    }

                    // Registration form
                    form { onsubmit: handle_register,
                        div { class: "auth-field",
                            Label { html_for: "display_name", "Display Name" }
                            Input {
                                id: "display_name",
                                placeholder: "Your display name",
                                value: display_name(),
                                on_input: move |e: FormEvent| display_name.set(e.value()),
                            }
                        }
                        div { class: "auth-field",
                            Label { html_for: "username", "Username" }
                            Input {
                                id: "username",
                                placeholder: "Choose a username",
                                value: username(),
                                on_input: move |e: FormEvent| username.set(e.value()),
                            }
                        }
                        div { class: "auth-field",
                            Label { html_for: "email", "Email" }
                            Input {
                                input_type: "email",
                                id: "email",
                                placeholder: "you@example.com",
                                value: email(),
                                on_input: move |e: FormEvent| email.set(e.value()),
                            }
                        }
                        div { class: "auth-field",
                            Label { html_for: "password", "Password" }
                            Input {
                                input_type: "password",
                                id: "password",
                                placeholder: "Create a password",
                                value: password(),
                                on_input: move |e: FormEvent| password.set(e.value()),
                            }
                        }
                        button {
                            r#type: "submit",
                            class: "auth-submit button",
                            disabled: loading(),
                            if loading() { "Creating account..." } else { "Create Account" }
                        }
                    }
                }

                CardFooter {
                    p { class: "auth-link",
                        "Already have an account? "
                        Link { to: Route::Login {}, "Sign in" }
                    }
                }
            }
        }
    }
}
