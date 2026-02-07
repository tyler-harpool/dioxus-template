use crate::ProfileState;
use dioxus::prelude::*;
use shared_ui::{
    use_toast, Accordion, AccordionContent, AccordionItem, AccordionTrigger, AlertDialogAction,
    AlertDialogActions, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogRoot, AlertDialogTitle, Badge, BadgeVariant, Button, ButtonVariant, Calendar,
    CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton,
    CalendarPreviousMonthButton, CalendarSelectMonth, CalendarSelectYear, Collapsible,
    CollapsibleContent, CollapsibleTrigger, Date, Form, Input, Label, MenubarContent, MenubarItem,
    MenubarMenu, MenubarRoot, MenubarSeparator, MenubarTrigger, SelectContent, SelectItem,
    SelectRoot, SelectTrigger, SelectValue, Separator, Sheet, SheetClose, SheetContent,
    SheetDescription, SheetFooter, SheetHeader, SheetSide, SheetTitle, Switch, SwitchThumb,
    Textarea, ToastOptions, Toggle, UtcDateTime,
};

/// Settings page with menubar navigation, accordion sections, and advanced collapsible.
#[component]
pub fn Settings() -> Element {
    // Theme state (shared with layout via context)
    let mut theme_state: shared_ui::theme::ThemeState = use_context();

    // Profile state (shared with layout via context)
    let profile: ProfileState = use_context();
    let mut profile_name = profile.display_name;
    let mut profile_email = profile.email;

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

                            Form {
                                onsubmit: move |_evt| {
                                    // Handle profile save
                                },

                                div {
                                    class: "settings-form",

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
                                    }

                                    Button {
                                        variant: ButtonVariant::Primary,
                                        onclick: move |_| {
                                            toast.success(
                                                format!("Profile updated: {}", profile_name()),
                                                ToastOptions::new(),
                                            );
                                        },
                                        "Save Profile"
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
