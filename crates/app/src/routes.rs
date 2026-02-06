use dioxus::prelude::*;
use server::api::{create_user, list_users};
use shared_types::ButtonVariant;
use shared_ui::{Button, Card, PageLayout, TextInput};

/// Application routes.
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/users")]
    Users {},
}

/// Home page component.
#[component]
fn Home() -> Element {
    rsx! {
        PageLayout { title: "Dioxus Fullstack Template".to_string(),
            Card { title: "Welcome".to_string(), subtitle: "Get started by editing this template.".to_string(),
                p { "This is a fullstack Dioxus application with:" }
                ul {
                    li { "Shared types across frontend and backend" }
                    li { "Branded UI component library" }
                    li { "Server functions with SQLite" }
                    li { "CSS design tokens and theming" }
                }
                Link { to: Route::Users {},
                    Button {
                        label: "View Users".to_string(),
                        variant: ButtonVariant::Primary,
                        on_click: move |_| {},
                    }
                }
            }
        }
    }
}

/// Users page component with CRUD operations.
#[component]
fn Users() -> Element {
    let mut users = use_server_future(list_users)?;
    let mut new_username = use_signal(|| String::new());
    let mut new_display_name = use_signal(|| String::new());

    rsx! {
        PageLayout { title: "Users".to_string(),
            div { style: "display: flex; flex-direction: column; gap: 1rem;",
                Card { title: "Add User".to_string(),
                    div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
                        TextInput {
                            value: new_username(),
                            placeholder: "Username".to_string(),
                            label: "Username".to_string(),
                            on_input: move |e: FormEvent| new_username.set(e.value()),
                        }
                        TextInput {
                            value: new_display_name(),
                            placeholder: "Display Name".to_string(),
                            label: "Display Name".to_string(),
                            on_input: move |e: FormEvent| new_display_name.set(e.value()),
                        }
                        Button {
                            label: "Create User".to_string(),
                            variant: ButtonVariant::Primary,
                            on_click: move |_| async move {
                                let username = new_username();
                                let display_name = new_display_name();
                                if !username.is_empty() && !display_name.is_empty() {
                                    if let Ok(_) = create_user(username, display_name).await {
                                        users.restart();
                                        new_username.set(String::new());
                                        new_display_name.set(String::new());
                                    }
                                }
                            },
                        }
                    }
                }

                Card { title: "User List".to_string(),
                    match users() {
                        Some(Ok(list)) => rsx! {
                            if list.is_empty() {
                                p { "No users yet. Create one above!" }
                            } else {
                                ul { style: "list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 0.5rem;",
                                    for user in list {
                                        li { key: "{user.id}",
                                            style: "padding: 0.5rem; border-radius: var(--radius-sm); background: var(--color-surface-raised);",
                                            strong { "{user.display_name}" }
                                            span { style: "color: var(--color-on-surface-muted); margin-left: 0.5rem;",
                                                " @{user.username}"
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! {
                            p { style: "color: var(--color-danger);", "Error: {e}" }
                        },
                        None => rsx! {
                            p { "Loading..." }
                        },
                    }
                }

                Link { to: Route::Home {},
                    Button {
                        label: "Back to Home".to_string(),
                        variant: ButtonVariant::Secondary,
                        on_click: move |_| {},
                    }
                }
            }
        }
    }
}
