use dioxus::prelude::*;
use shared_ui::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, Button, ButtonVariant, Calendar,
    CalendarGrid, CalendarHeader, CalendarMonthTitle, CalendarNavigation, CalendarNextMonthButton,
    CalendarPreviousMonthButton, CalendarSelectMonth, CalendarSelectYear, Card, CardContent,
    CardHeader, CardTitle, Collapsible, CollapsibleContent, CollapsibleTrigger, Form, Input, Label,
    MenubarContent, MenubarItem, MenubarMenu, MenubarRoot, MenubarSeparator, MenubarTrigger,
    RadioGroup, RadioGroupItem, Separator, Switch, SwitchThumb, Toggle,
};

/// Settings page with menubar navigation, accordion sections, and advanced collapsible.
#[component]
pub fn Settings() -> Element {
    // Profile state
    let mut profile_name = use_signal(|| String::from("Admin User"));
    let mut profile_email = use_signal(|| String::from("admin@cyberapp.io"));

    // Appearance state
    let mut animations_enabled = use_signal(|| true);
    let mut compact_mode = use_signal(|| false);

    // Notification state
    let mut email_notifs = use_signal(|| true);
    let mut push_notifs = use_signal(|| false);
    let mut weekly_digest = use_signal(|| true);

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
            div {
                class: "scroll-x-mobile",
            MenubarRoot {
                MenubarMenu {
                    index: 0usize,
                    MenubarTrigger { "General" }
                    MenubarContent {
                        MenubarItem { index: 0usize, value: "profile", "Profile" }
                        MenubarItem { index: 1usize, value: "account", "Account" }
                        MenubarItem { index: 2usize, value: "security", "Security" }
                    }
                }

                MenubarSeparator {}

                MenubarMenu {
                    index: 1usize,
                    MenubarTrigger { "Appearance" }
                    MenubarContent {
                        MenubarItem { index: 0usize, value: "theme", "Theme" }
                        MenubarItem { index: 1usize, value: "layout", "Layout" }
                        MenubarItem { index: 2usize, value: "fonts", "Fonts" }
                    }
                }

                MenubarSeparator {}

                MenubarMenu {
                    index: 2usize,
                    MenubarTrigger { "Notifications" }
                    MenubarContent {
                        MenubarItem { index: 0usize, value: "email-notifs", "Email" }
                        MenubarItem { index: 1usize, value: "push-notifs", "Push" }
                        MenubarItem { index: 2usize, value: "digest", "Digest" }
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
                                            // Save profile action
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

                            // Theme selection via RadioGroup
                            div {
                                class: "settings-theme-group",
                                span {
                                    class: "settings-theme-label",
                                    "Theme"
                                }
                                RadioGroup {
                                    default_value: "cyberpunk",
                                    on_value_change: move |val: String| {
                                        shared_ui::theme::set_theme(&val);
                                    },
                                    RadioGroupItem { value: "cyberpunk", index: 0usize, "Cyberpunk" }
                                    RadioGroupItem { value: "light", index: 1usize, "Light" }
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

            // -- Collapsible Advanced section --
            Collapsible {
                CollapsibleTrigger {
                    Button {
                        variant: ButtonVariant::Outline,
                        "Advanced Settings"
                    }
                }

                CollapsibleContent {
                    div {
                        class: "settings-section-lg",

                        // Calendar widget for reference/demo
                        Card {
                            CardHeader {
                                CardTitle { "Schedule" }
                            }
                            CardContent {
                                div {
                                    class: "calendar-container",
                                    Calendar {
                                        CalendarHeader {
                                            CalendarNavigation {
                                                CalendarPreviousMonthButton {}
                                                CalendarMonthTitle {}
                                                CalendarNextMonthButton {}
                                            }
                                        }
                                        CalendarGrid {}
                                        CalendarSelectMonth {}
                                        CalendarSelectYear {}
                                    }
                                }
                            }
                        }

                        Separator {}

                        // Danger zone
                        Card {
                            CardHeader {
                                CardTitle { "Danger Zone" }
                            }
                            CardContent {
                                div {
                                    class: "danger-zone-stack",
                                    p {
                                        class: "danger-zone-text",
                                        "Irreversible actions that affect your account permanently."
                                    }
                                    Button {
                                        variant: ButtonVariant::Destructive,
                                        "Delete Account"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
