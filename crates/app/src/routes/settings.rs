use crate::auth::use_auth;
use crate::ProfileState;
use dioxus::prelude::*;
use shared_ui::{
    use_toast, Accordion, AccordionContent, AccordionItem, AccordionTrigger, AlertDialogAction,
    AlertDialogActions, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogRoot, AlertDialogTitle, Avatar, AvatarFallback, AvatarImage, Badge, BadgeVariant,
    Button, ButtonVariant, Calendar, CalendarGrid, CalendarHeader, CalendarMonthTitle,
    CalendarNavigation, CalendarNextMonthButton, CalendarPreviousMonthButton, CalendarSelectMonth,
    CalendarSelectYear, Collapsible, CollapsibleContent, CollapsibleTrigger, Date, Form, Input,
    Label, MenubarContent, MenubarItem, MenubarMenu, MenubarRoot, MenubarSeparator, MenubarTrigger,
    SelectContent, SelectItem, SelectRoot, SelectTrigger, SelectValue, Separator, Sheet,
    SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetSide, SheetTitle,
    Switch, SwitchThumb, Textarea, ToastOptions, Toggle, UtcDateTime,
};

/// Settings page with menubar navigation, accordion sections, and advanced collapsible.
#[component]
pub fn Settings() -> Element {
    // Auth state (for updating user after save)
    let mut auth = use_auth();

    // Theme state (shared with layout via context)
    let mut theme_state: shared_ui::theme::ThemeState = use_context();

    // Profile state (shared with layout via context)
    let profile: ProfileState = use_context();
    // Local editable signals for form fields, initialized from profile memos
    let mut profile_name = use_signal(move || (profile.display_name)());
    let mut profile_email = use_signal(move || (profile.email)());

    // Profile save state
    let mut saving = use_signal(|| false);
    let mut profile_error = use_signal(|| Option::<String>::None);
    let mut profile_field_errors = use_signal(std::collections::HashMap::<String, String>::new);

    // Avatar upload state
    let mut uploading_avatar = use_signal(|| false);

    // Avatar popup state
    let mut avatar_popup_open = use_signal(|| false);

    // Appearance state
    let mut animations_enabled = use_signal(|| true);
    let mut compact_mode = use_signal(|| false);

    // Notification state
    let mut email_notifs = use_signal(|| true);
    let mut push_notifs = use_signal(|| false);
    let mut weekly_digest = use_signal(|| true);

    // Calendar state
    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| UtcDateTime::now().date());

    // Event sheet state
    let mut event_sheet_open = use_signal(|| false);
    let mut event_title = use_signal(String::new);
    let mut event_notes = use_signal(String::new);

    // Delete account dialog state
    let mut delete_dialog_open = use_signal(|| false);

    let toast = use_toast();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./settings.css") }

        div {
            class: "settings-page",

            // Page heading
            h1 {
                class: "settings-title",
                "Settings"
            }

            // -- Menubar at top --
            MenubarRoot {
                MenubarMenu {
                    index: 0usize,
                    MenubarTrigger { "General" }
                    MenubarContent {
                        MenubarItem { index: 0usize, value: "profile",
                            on_select: move |_: String| { toast.info("Profile selected".to_string(), ToastOptions::new()); },
                            "Profile"
                        }
                        MenubarItem { index: 1usize, value: "account",
                            on_select: move |_: String| { toast.info("Account selected".to_string(), ToastOptions::new()); },
                            "Account"
                        }
                        MenubarItem { index: 2usize, value: "security",
                            on_select: move |_: String| { toast.info("Security selected".to_string(), ToastOptions::new()); },
                            "Security"
                        }
                    }
                }

                MenubarSeparator {}

                MenubarMenu {
                    index: 1usize,
                    MenubarTrigger { "Appearance" }
                    MenubarContent {
                        MenubarItem { index: 0usize, value: "theme",
                            on_select: move |_: String| { toast.info("Theme selected".to_string(), ToastOptions::new()); },
                            "Theme"
                        }
                        MenubarItem { index: 1usize, value: "layout",
                            on_select: move |_: String| { toast.info("Layout selected".to_string(), ToastOptions::new()); },
                            "Layout"
                        }
                        MenubarItem { index: 2usize, value: "fonts",
                            on_select: move |_: String| { toast.info("Fonts selected".to_string(), ToastOptions::new()); },
                            "Fonts"
                        }
                    }
                }

                MenubarSeparator {}

                MenubarMenu {
                    index: 2usize,
                    MenubarTrigger { "Notifications" }
                    MenubarContent {
                        MenubarItem { index: 0usize, value: "email-notifs",
                            on_select: move |_: String| { toast.info("Email notifications selected".to_string(), ToastOptions::new()); },
                            "Email"
                        }
                        MenubarItem { index: 1usize, value: "push-notifs",
                            on_select: move |_: String| { toast.info("Push notifications selected".to_string(), ToastOptions::new()); },
                            "Push"
                        }
                        MenubarItem { index: 2usize, value: "digest",
                            on_select: move |_: String| { toast.info("Digest selected".to_string(), ToastOptions::new()); },
                            "Digest"
                        }
                    }
                }
            }

            Separator {}

            // -- Accordion as main settings container --
            Accordion {
                // AccordionItem 0: Profile
                AccordionItem {
                    index: 0usize,
                    default_open: true,

                    AccordionTrigger { "Profile" }
                    AccordionContent {
                        div {
                            class: "settings-section",

                            // Avatar preview and upload
                            div {
                                class: "settings-avatar-section",
                                div {
                                    class: "settings-avatar-preview",
                                    onclick: move |_| {
                                        if profile.avatar_url.read().is_some() {
                                            avatar_popup_open.set(true);
                                        }
                                    },
                                    Avatar {
                                        if let Some(url) = profile.avatar_url.read().as_ref() {
                                            AvatarImage { src: url.clone() }
                                        }
                                        AvatarFallback {
                                            {profile_name().split_whitespace().filter_map(|w| w.chars().next()).take(2).collect::<String>().to_uppercase()}
                                        }
                                    }
                                }
                                label {
                                    class: if uploading_avatar() { "button avatar-upload-label disabled" } else { "button avatar-upload-label" },
                                    "data-style": "outline",
                                    input {
                                        r#type: "file",
                                        accept: "image/jpeg,image/png,image/webp",
                                        class: "avatar-upload-input",
                                        onchange: move |evt: FormEvent| async move {
                                            uploading_avatar.set(true);
                                            let files = evt.files();
                                            if let Some(file) = files.first() {
                                                if file.size() > 2 * 1024 * 1024 {
                                                    toast.error("Avatar must be under 2 MB".to_string(), ToastOptions::new());
                                                } else {
                                                    let content_type = file.content_type()
                                                        .unwrap_or_else(|| "image/jpeg".to_string());
                                                    match file.read_bytes().await {
                                                        Ok(bytes) => {
                                                            use base64::Engine as _;
                                                            let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
                                                            match server::api::upload_user_avatar(encoded, content_type).await {
                                                                Ok(user) => {
                                                                    auth.set_user(user);
                                                                    toast.success("Avatar uploaded".to_string(), ToastOptions::new());
                                                                }
                                                                Err(e) => {
                                                                    toast.error(
                                                                        shared_types::AppError::friendly_message(&e.to_string()),
                                                                        ToastOptions::new(),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                        Err(_) => {
                                                            toast.error("Failed to read file".to_string(), ToastOptions::new());
                                                        }
                                                    }
                                                }
                                            }
                                            uploading_avatar.set(false);
                                        },
                                    }
                                    if uploading_avatar() { "Uploading..." } else { "Upload Avatar" }
                                }
                            }

                            Form {
                                onsubmit: move |_evt| async move {
                                    saving.set(true);
                                    profile_error.set(None);
                                    profile_field_errors.set(std::collections::HashMap::new());

                                    match server::api::update_profile(
                                        profile_name(),
                                        profile_email(),
                                    )
                                    .await
                                    {
                                        Ok(user) => {
                                            auth.set_user(user);
                                            toast.success(
                                                "Profile updated successfully".to_string(),
                                                ToastOptions::new(),
                                            );
                                        }
                                        Err(e) => {
                                            let err_str = e.to_string();
                                            let field_errs =
                                                shared_types::AppError::parse_field_errors(
                                                    &err_str,
                                                );
                                            if field_errs.is_empty() {
                                                profile_error.set(Some(
                                                    shared_types::AppError::friendly_message(
                                                        &err_str,
                                                    ),
                                                ));
                                            } else {
                                                profile_field_errors.set(field_errs);
                                            }
                                            toast.error(
                                                "Failed to update profile".to_string(),
                                                ToastOptions::new(),
                                            );
                                        }
                                    }
                                    saving.set(false);
                                },

                                div {
                                    class: "settings-form",

                                    if let Some(err) = profile_error() {
                                        div { class: "auth-error", "{err}" }
                                    }

                                    div {
                                        class: "settings-field",
                                        Label { html_for: "profile-name", "Display Name" }
                                        Input {
                                            value: profile_name(),
                                            placeholder: "Enter your name",
                                            label: "",
                                            on_input: move |evt: FormEvent| {
                                                profile_name.set(evt.value());
                                            },
                                        }
                                        if let Some(err) = profile_field_errors().get("display_name") {
                                            div { class: "settings-field-error", "{err}" }
                                        }
                                    }

                                    div {
                                        class: "settings-field",
                                        Label { html_for: "profile-email", "Email Address" }
                                        Input {
                                            value: profile_email(),
                                            placeholder: "Enter your email",
                                            label: "",
                                            on_input: move |evt: FormEvent| {
                                                profile_email.set(evt.value());
                                            },
                                        }
                                        if let Some(err) = profile_field_errors().get("email") {
                                            div { class: "settings-field-error", "{err}" }
                                        }
                                    }

                                    button {
                                        r#type: "submit",
                                        class: "auth-submit button",
                                        disabled: saving(),
                                        if saving() { "Saving..." } else { "Save Profile" }
                                    }
                                }
                            }
                        }
                    }
                }

                // AccordionItem 1: Appearance
                AccordionItem {
                    index: 1usize,

                    AccordionTrigger { "Appearance" }
                    AccordionContent {
                        div {
                            class: "settings-section-lg",

                            // Theme family selector
                            div {
                                class: "settings-theme-group",
                                span {
                                    class: "settings-theme-label",
                                    "Theme"
                                }
                                SelectRoot::<String> {
                                    default_value: Some((theme_state.family)()),
                                    on_value_change: move |val: Option<String>| {
                                        if let Some(v) = val {
                                            theme_state.family.set(v);
                                            theme_state.apply();
                                        }
                                    },
                                    SelectTrigger {
                                        SelectValue {}
                                    }
                                    SelectContent {
                                        SelectItem::<String> { value: "cyberpunk", index: 0usize, "Cyberpunk" }
                                        SelectItem::<String> { value: "solar", index: 1usize, "Solarized" }
                                    }
                                }
                            }

                            Separator {}

                            // Animations toggle
                            div {
                                class: "settings-toggle-row",
                                span {
                                    class: "settings-toggle-label",
                                    "Enable animations"
                                }
                                Toggle {
                                    pressed: Some(animations_enabled()),
                                    on_pressed_change: move |val: bool| {
                                        animations_enabled.set(val);
                                    },
                                    "Animations"
                                }
                            }

                            Separator {}

                            // Compact mode switch
                            div {
                                class: "settings-toggle-row",
                                span {
                                    class: "settings-toggle-label",
                                    "Compact mode"
                                }
                                Switch {
                                    checked: Some(compact_mode()),
                                    on_checked_change: move |val: bool| {
                                        compact_mode.set(val);
                                    },
                                    SwitchThumb {}
                                }
                            }
                        }
                    }
                }

                // AccordionItem 2: Notifications
                AccordionItem {
                    index: 2usize,

                    AccordionTrigger { "Notifications" }
                    AccordionContent {
                        div {
                            class: "settings-section",

                            // Email notifications
                            div {
                                class: "settings-toggle-row",
                                span {
                                    class: "settings-toggle-label",
                                    "Email notifications"
                                }
                                Switch {
                                    checked: Some(email_notifs()),
                                    on_checked_change: move |val: bool| {
                                        email_notifs.set(val);
                                    },
                                    SwitchThumb {}
                                }
                            }

                            Separator {}

                            // Push notifications
                            div {
                                class: "settings-toggle-row",
                                span {
                                    class: "settings-toggle-label",
                                    "Push notifications"
                                }
                                Switch {
                                    checked: Some(push_notifs()),
                                    on_checked_change: move |val: bool| {
                                        push_notifs.set(val);
                                    },
                                    SwitchThumb {}
                                }
                            }

                            Separator {}

                            // Weekly digest
                            div {
                                class: "settings-toggle-row",
                                span {
                                    class: "settings-toggle-label",
                                    "Weekly digest"
                                }
                                Switch {
                                    checked: Some(weekly_digest()),
                                    on_checked_change: move |val: bool| {
                                        weekly_digest.set(val);
                                    },
                                    SwitchThumb {}
                                }
                            }
                        }
                    }
                }
            }

            Separator {}

            // -- Advanced Settings --
            Collapsible {
                CollapsibleTrigger {
                    Button {
                        variant: ButtonVariant::Outline,
                        "Show Advanced Settings"
                    }
                }

                CollapsibleContent {
                    div {
                        class: "settings-section-lg",

                        // Calendar widget
                        div {
                            class: "calendar-container",
                            Calendar {
                                selected_date: selected_date,
                                on_date_change: move |date: Option<Date>| {
                                    selected_date.set(date);
                                    if let Some(d) = date {
                                        toast.info(
                                            format!("Selected: {} {}-{:02}-{:02}", d.weekday(), d.year(), d.month() as u8, d.day()),
                                            ToastOptions::new(),
                                        );
                                        event_title.set(String::new());
                                        event_notes.set(String::new());
                                        event_sheet_open.set(true);
                                    }
                                },
                                view_date: view_date,
                                on_view_change: move |new_view: Date| {
                                    view_date.set(new_view);
                                },
                                CalendarHeader {
                                    CalendarNavigation {
                                        CalendarPreviousMonthButton { "\u{2039}" }
                                        CalendarMonthTitle {}
                                        CalendarNextMonthButton { "\u{203a}" }
                                    }
                                }
                                CalendarGrid {}
                                CalendarSelectMonth {}
                                CalendarSelectYear {}
                            }

                            if let Some(date) = selected_date() {
                                div {
                                    class: "selected-date-display",
                                    span { "Selected date:" }
                                    Badge {
                                        variant: BadgeVariant::Primary,
                                        "{date.year()}-{date.month() as u8:02}-{date.day():02}"
                                    }
                                }
                            }
                        }

                        Separator {}

                        // Danger zone
                        div {
                            class: "danger-zone-stack",
                            p {
                                class: "danger-zone-text",
                                "Irreversible actions that affect your account permanently."
                            }
                            Button {
                                variant: ButtonVariant::Destructive,
                                onclick: move |_| {
                                    delete_dialog_open.set(true);
                                },
                                "Delete Account"
                            }
                        }
                    }
                }
            }

            // -- Event Sheet: slides in when a date is selected --
            Sheet {
                open: event_sheet_open(),
                on_close: move |_| event_sheet_open.set(false),
                side: SheetSide::Right,

                SheetHeader {
                    SheetTitle {
                        if selected_date().is_some() {
                            "Schedule Event"
                        }
                    }
                    SheetDescription {
                        if let Some(date) = selected_date() {
                            span {
                                "{date.weekday()}, {date.month()} {date.day()}, {date.year()}"
                            }
                        }
                    }
                }

                SheetContent {
                    Form {
                        onsubmit: move |_| {},
                        div {
                            class: "settings-form",
                            div {
                                class: "settings-field",
                                Label { html_for: "event-title", "Event Title" }
                                Input {
                                    value: event_title(),
                                    placeholder: "Meeting, Deadline, Reminder...",
                                    label: "",
                                    on_input: move |evt: FormEvent| {
                                        event_title.set(evt.value());
                                    },
                                }
                            }
                            div {
                                class: "settings-field",
                                Label { html_for: "event-notes", "Notes" }
                                Textarea {
                                    value: event_notes(),
                                    placeholder: "Add details about this event...",
                                    on_input: move |evt: FormEvent| {
                                        event_notes.set(evt.value());
                                    },
                                }
                            }
                        }
                    }
                }

                SheetFooter {
                    SheetClose {
                        on_close: move |_| event_sheet_open.set(false),
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| {
                            if let Some(d) = selected_date() {
                                let title = if event_title().is_empty() {
                                    "Untitled Event".to_string()
                                } else {
                                    event_title()
                                };
                                toast.success(
                                    format!("\"{}\" scheduled for {}-{:02}-{:02}", title, d.year(), d.month() as u8, d.day()),
                                    ToastOptions::new(),
                                );
                                event_sheet_open.set(false);
                            }
                        },
                        "Save Event"
                    }
                }
            }

            // -- Avatar popup overlay --
            if avatar_popup_open() {
                div {
                    class: "avatar-popup-overlay",
                    onclick: move |_| avatar_popup_open.set(false),

                    div {
                        class: "avatar-popup-frame",
                        onclick: move |evt: MouseEvent| evt.stop_propagation(),

                        if let Some(url) = profile.avatar_url.read().as_ref() {
                            img {
                                class: "avatar-popup-image",
                                src: url.clone(),
                                alt: "Avatar",
                            }
                        }
                    }
                }
            }

            // -- Delete Account confirmation dialog --
            AlertDialogRoot {
                open: delete_dialog_open(),
                on_open_change: move |val: bool| delete_dialog_open.set(val),

                AlertDialogContent {
                    AlertDialogTitle { "Delete Account" }
                    AlertDialogDescription {
                        "This action cannot be undone. This will permanently delete your account and remove all associated data."
                    }
                    AlertDialogActions {
                        AlertDialogCancel { "Cancel" }
                        AlertDialogAction {
                            on_click: move |_| {
                                toast.error(
                                    "Account deletion is not available in this demo.".to_string(),
                                    ToastOptions::new(),
                                );
                                delete_dialog_open.set(false);
                            },
                            "Yes, Delete"
                        }
                    }
                }
            }
        }
    }
}
