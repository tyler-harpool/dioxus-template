use dioxus::prelude::*;
use server::api::{create_user, delete_user, list_users, update_user};
use shared_types::User;
use shared_ui::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle, Avatar, AvatarFallback, Badge,
    BadgeVariant, Button, ButtonVariant, Checkbox, CheckboxIndicator, CheckboxState, ContextMenu,
    ContextMenuContent, ContextMenuItem, ContextMenuTrigger, DialogContent, DialogDescription,
    DialogRoot, DialogTitle, Form, Input, Label, PopoverContent, PopoverRoot, PopoverTrigger,
    ScrollArea, Separator, ToastOptions, Toolbar, ToolbarButton, ToolbarSeparator, use_toast,
};

/// Extract the first two characters of a name as uppercase initials.
fn initials(name: &str) -> String {
    name.chars().take(2).collect::<String>().to_uppercase()
}

/// Users management page with CRUD operations.
#[component]
pub fn Users() -> Element {
    let mut users = use_server_future(list_users)?;
    let toast = use_toast();

    let mut show_create_dialog = use_signal(|| false);
    let mut editing_user: Signal<Option<User>> = use_signal(|| None);
    let mut show_delete_confirm = use_signal(|| false);
    let mut selected_ids: Signal<Vec<i64>> = use_signal(Vec::new);
    let mut form_username = use_signal(String::new);
    let mut form_display_name = use_signal(String::new);

    let has_selection = !selected_ids.read().is_empty();

    // Handle form save (create or update)
    let handle_save = move |_: FormEvent| {
        let username = form_username.read().clone();
        let display_name = form_display_name.read().clone();
        let editing = editing_user.read().clone();

        spawn(async move {
            let result = if let Some(user) = editing {
                update_user(user.id, username, display_name).await
            } else {
                create_user(username, display_name).await
            };

            match result {
                Ok(_) => {
                    let msg = if editing_user.read().is_some() {
                        "User updated"
                    } else {
                        "User created"
                    };
                    toast.success(msg.to_string(), ToastOptions::new());
                    show_create_dialog.set(false);
                    editing_user.set(None);
                    users.restart();
                }
                Err(err) => {
                    toast.error(format!("Error: {err}"), ToastOptions::new());
                }
            }
        });
    };

    // Handle delete of selected users
    let handle_delete_selected = move |_: MouseEvent| {
        let ids = selected_ids.read().clone();

        spawn(async move {
            let mut had_error = false;
            for id in &ids {
                if let Err(err) = delete_user(*id).await {
                    toast.error(
                        format!("Failed to delete user {id}: {err}"),
                        ToastOptions::new(),
                    );
                    had_error = true;
                }
            }
            if !had_error {
                let count = ids.len();
                toast.success(format!("{count} user(s) deleted"), ToastOptions::new());
            }
            selected_ids.set(Vec::new());
            show_delete_confirm.set(false);
            users.restart();
        });
    };

    let user_list = users.read();
    let user_list = user_list.as_ref().and_then(|r| r.as_ref().ok());

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: var(--space-lg); width: 100%;",

            // Toolbar
            Toolbar {
                aria_label: "User actions",
                ToolbarButton {
                    index: 0usize,
                    on_click: move |_| {
                        editing_user.set(None);
                        form_username.set(String::new());
                        form_display_name.set(String::new());
                        show_create_dialog.set(true);
                    },
                    "Add User"
                }
                ToolbarSeparator {}
                ToolbarButton {
                    index: 1usize,
                    disabled: !has_selection,
                    on_click: move |_| {
                        show_delete_confirm.set(true);
                    },
                    "Delete Selected"
                }
            }

            // User List
            ScrollArea {
                div {
                    style: "display: flex; flex-direction: column; width: 100%; max-height: 600px;",

                    if let Some(user_vec) = user_list {
                        if user_vec.is_empty() {
                            div {
                                style: "padding: var(--space-xl); text-align: center; color: var(--color-on-surface-muted);",
                                "No users found. Click \"Add User\" to create one."
                            }
                        } else {
                            for user in user_vec.iter() {
                                {
                                    let user_id = user.id;
                                    let user_clone = user.clone();
                                    let user_for_edit = user.clone();
                                    let user_for_ctx_edit = user.clone();
                                    let display_initials = initials(&user.display_name);
                                    let is_checked = selected_ids.read().contains(&user_id);

                                    rsx! {
                                        ContextMenu {
                                            ContextMenuTrigger {
                                                div {
                                                    style: "display: flex; align-items: center; gap: var(--space-md); padding: var(--space-sm) var(--space-md); width: 100%; cursor: default;",

                                                    Checkbox {
                                                        default_checked: if is_checked { CheckboxState::Checked } else { CheckboxState::Unchecked },
                                                        on_checked_change: move |state: CheckboxState| {
                                                            let mut ids = selected_ids.write();
                                                            match state {
                                                                CheckboxState::Checked => {
                                                                    if !ids.contains(&user_id) {
                                                                        ids.push(user_id);
                                                                    }
                                                                }
                                                                _ => {
                                                                    ids.retain(|&id| id != user_id);
                                                                }
                                                            }
                                                        },
                                                        CheckboxIndicator {
                                                            span { "\u{2713}" }
                                                        }
                                                    }

                                                    Avatar {
                                                        AvatarFallback { "{display_initials}" }
                                                    }

                                                    div {
                                                        style: "display: flex; flex-direction: column; flex: 1; min-width: 0;",
                                                        span {
                                                            style: "font-weight: 600; color: var(--color-on-surface); overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                                            "{user_clone.display_name}"
                                                        }
                                                        span {
                                                            style: "font-size: var(--font-size-sm); color: var(--color-on-surface-muted); overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                                            "@{user_clone.username}"
                                                        }
                                                    }

                                                    Badge { variant: BadgeVariant::Secondary, "user" }

                                                    PopoverRoot {
                                                        PopoverTrigger {
                                                            Button {
                                                                variant: ButtonVariant::Ghost,
                                                                "\u{2026}"
                                                            }
                                                        }
                                                        PopoverContent {
                                                            div {
                                                                style: "padding: var(--space-md); display: flex; flex-direction: column; gap: var(--space-sm);",
                                                                span {
                                                                    style: "font-weight: 700; color: var(--color-primary);",
                                                                    "{user_for_edit.display_name}"
                                                                }
                                                                span {
                                                                    style: "font-size: var(--font-size-sm); color: var(--color-on-surface-muted);",
                                                                    "Username: {user_for_edit.username}"
                                                                }
                                                                span {
                                                                    style: "font-size: var(--font-size-sm); color: var(--color-on-surface-muted);",
                                                                    "ID: {user_id}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            ContextMenuContent {
                                                ContextMenuItem {
                                                    value: "edit",
                                                    index: 0usize,
                                                    on_select: move |_: String| {
                                                        let u = user_for_ctx_edit.clone();
                                                        form_username.set(u.username.clone());
                                                        form_display_name.set(u.display_name.clone());
                                                        editing_user.set(Some(u));
                                                        show_create_dialog.set(true);
                                                    },
                                                    "Edit"
                                                }
                                                ContextMenuItem {
                                                    value: "delete",
                                                    index: 1usize,
                                                    on_select: move |_: String| {
                                                        spawn(async move {
                                                            match delete_user(user_id).await {
                                                                Ok(()) => {
                                                                    toast.success("User deleted".to_string(), ToastOptions::new());
                                                                    selected_ids.write().retain(|&id| id != user_id);
                                                                    users.restart();
                                                                }
                                                                Err(err) => {
                                                                    toast.error(format!("Error: {err}"), ToastOptions::new());
                                                                }
                                                            }
                                                        });
                                                    },
                                                    "Delete"
                                                }
                                            }
                                        }

                                        Separator {}
                                    }
                                }
                            }
                        }
                    } else {
                        div {
                            style: "padding: var(--space-xl); text-align: center; color: var(--color-on-surface-muted);",
                            "Loading users..."
                        }
                    }
                }
            }

            // Create / Edit Dialog
            DialogRoot {
                open: show_create_dialog(),
                on_open_change: move |open: bool| show_create_dialog.set(open),
                DialogContent {
                    DialogTitle {
                        if editing_user.read().is_some() { "Edit User" } else { "Add User" }
                    }
                    DialogDescription {
                        if editing_user.read().is_some() {
                            "Update the user details below."
                        } else {
                            "Fill in the details to create a new user."
                        }
                    }

                    Form {
                        onsubmit: handle_save,
                        div {
                            style: "display: flex; flex-direction: column; gap: var(--space-md); padding: var(--space-md) 0;",

                            div {
                                style: "display: flex; flex-direction: column; gap: var(--space-xs);",
                                Label { html_for: "username-field", "Username" }
                                Input {
                                    value: form_username(),
                                    placeholder: "Enter username",
                                    label: "",
                                    on_input: move |evt: FormEvent| form_username.set(evt.value()),
                                }
                            }

                            div {
                                style: "display: flex; flex-direction: column; gap: var(--space-xs);",
                                Label { html_for: "display-name-field", "Display Name" }
                                Input {
                                    value: form_display_name(),
                                    placeholder: "Enter display name",
                                    label: "",
                                    on_input: move |evt: FormEvent| form_display_name.set(evt.value()),
                                }
                            }

                            div {
                                style: "display: flex; justify-content: flex-end; gap: var(--space-sm); padding-top: var(--space-sm);",
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    onclick: move |_| {
                                        show_create_dialog.set(false);
                                        editing_user.set(None);
                                    },
                                    "Cancel"
                                }
                                Button {
                                    variant: ButtonVariant::Primary,
                                    "Save"
                                }
                            }
                        }
                    }
                }
            }

            // Delete Confirmation Alert Dialog
            AlertDialogRoot {
                open: show_delete_confirm(),
                on_open_change: move |open: bool| show_delete_confirm.set(open),
                AlertDialogContent {
                    AlertDialogTitle { "Confirm Deletion" }
                    AlertDialogDescription {
                        {
                            let count = selected_ids.read().len();
                            format!("Are you sure you want to delete {count} selected user(s)? This action cannot be undone.")
                        }
                    }
                    AlertDialogActions {
                        AlertDialogCancel { "Cancel" }
                        AlertDialogAction {
                            on_click: handle_delete_selected,
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}
