pub mod dashboard;
pub mod login;
pub mod not_found;
pub mod products;
pub mod register;
pub mod settings;
pub mod users;

use crate::auth::use_auth;
use crate::ProfileState;
use dioxus::prelude::*;
use shared_types::UserTier;
use shared_ui::{
    Avatar, AvatarFallback, Badge, BadgeVariant, DropdownMenu, DropdownMenuContent,
    DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger, Navbar, Separator, Sidebar,
    SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel,
    SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider,
    SidebarRail, SidebarSeparator, SidebarTrigger, Switch, SwitchThumb,
};

use dashboard::Dashboard;
use login::Login;
use not_found::NotFound;
use products::Products;
use register::Register;
use settings::Settings;
use users::Users;

/// Application routes.
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[layout(AuthGuard)]
    #[layout(AppLayout)]
    #[route("/")]
    Dashboard {},
    #[route("/users")]
    Users {},
    #[route("/products")]
    Products {},
    #[route("/settings")]
    Settings {},
    #[end_layout]
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

/// Auth guard layout — redirects to /login if not authenticated.
#[component]
fn AuthGuard() -> Element {
    let auth = use_auth();

    if !auth.is_authenticated() {
        navigator().push(Route::Login {});
        return rsx! {
            div { class: "auth-guard-loading",
                p { "Redirecting to login..." }
            }
        };
    }

    rsx! { Outlet::<Route> {} }
}

/// Main app layout with sidebar and top navbar.
#[component]
fn AppLayout() -> Element {
    let route: Route = use_route();
    let profile: ProfileState = use_context();
    let mut auth = use_auth();

    let mut theme_state = use_context_provider(|| shared_ui::theme::ThemeState {
        family: Signal::new("cyberpunk".to_string()),
        is_dark: Signal::new(true),
    });

    let page_title = match &route {
        Route::Dashboard {} => "Dashboard",
        Route::Users {} => "Users",
        Route::Products {} => "Products",
        Route::Settings {} => "Settings",
        Route::Login {} | Route::Register {} => "Auth",
        _ => "",
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./layout.css") }

        SidebarProvider { default_open: false,
            Sidebar {
                SidebarHeader {
                    div {
                        class: "sidebar-brand",
                        span {
                            class: "sidebar-brand-name",
                            "CyberApp"
                        }
                    }
                }

                SidebarSeparator {}

                SidebarContent {
                    SidebarGroup {
                        SidebarGroupLabel { "Navigation" }
                        SidebarGroupContent {
                            SidebarMenu {
                                SidebarMenuItem {
                                    Link { to: Route::Dashboard {},
                                        SidebarMenuButton { active: matches!(route, Route::Dashboard {}),
                                            "Dashboard"
                                        }
                                    }
                                }
                                SidebarMenuItem {
                                    Link { to: Route::Users {},
                                        SidebarMenuButton { active: matches!(route, Route::Users {}),
                                            "Users"
                                        }
                                    }
                                }
                                SidebarMenuItem {
                                    Link { to: Route::Products {},
                                        SidebarMenuButton { active: matches!(route, Route::Products {}),
                                            "Products"
                                        }
                                    }
                                }
                                SidebarMenuItem {
                                    Link { to: Route::Settings {},
                                        SidebarMenuButton { active: matches!(route, Route::Settings {}),
                                            "Settings"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                SidebarFooter {
                    TierBadge {}
                    div {
                        class: "sidebar-footer-row",
                        span {
                            class: "sidebar-footer-label",
                            "Light Mode"
                        }
                        Switch {
                            checked: (theme_state.is_dark)(),
                            on_checked_change: move |checked: bool| {
                                theme_state.is_dark.set(checked);
                                theme_state.apply();
                            },
                            SwitchThumb {}
                        }
                    }
                }

                SidebarRail {}
            }

            SidebarInset {
                // Top navbar
                Navbar {
                    div {
                        class: "navbar-bar",

                        SidebarTrigger {
                            span { class: "navbar-trigger-icon", "\u{2630}" }
                        }

                        Separator { horizontal: false }

                        span {
                            class: "navbar-title",
                            "{page_title}"
                        }

                        // Spacer
                        div { class: "navbar-spacer" }

                        // User dropdown
                        DropdownMenu {
                            DropdownMenuTrigger {
                                Avatar {
                                    AvatarFallback {
                                        {profile.display_name.read().split_whitespace().filter_map(|w| w.chars().next()).take(2).collect::<String>().to_uppercase()}
                                    }
                                }
                            }
                            DropdownMenuContent {
                                DropdownMenuItem::<String> {
                                    value: "profile".to_string(),
                                    index: 0usize,
                                    on_select: move |_: String| {
                                        navigator().push(Route::Settings {});
                                    },
                                    "Profile"
                                }
                                DropdownMenuSeparator {}
                                DropdownMenuItem::<String> {
                                    value: "docs".to_string(),
                                    index: 1usize,
                                    div {
                                        onclick: move |_| {
                                            navigator().push(
                                                NavigationTarget::<Route>::External(
                                                    "/docs".to_string(),
                                                ),
                                            );
                                        },
                                        class: "dropdown-docs-link",
                                        "API Docs"
                                    }
                                }
                                DropdownMenuSeparator {}
                                DropdownMenuItem::<String> {
                                    value: "logout".to_string(),
                                    index: 2usize,
                                    on_select: move |_: String| {
                                        spawn(async move {
                                            let _ = server::api::logout().await;
                                        });
                                        auth.clear_auth();
                                        navigator().push(Route::Login {});
                                    },
                                    "Sign Out"
                                }
                            }
                        }
                    }
                }

                // Page content
                div {
                    class: "page-content",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

/// Displays the current user's tier as a badge in the sidebar footer.
#[component]
fn TierBadge() -> Element {
    let auth = use_auth();
    let tier = use_memo(move || {
        auth.current_user
            .read()
            .as_ref()
            .map(|u| u.tier.clone())
            .unwrap_or(UserTier::Free)
    });

    let (variant, label) = match tier() {
        UserTier::Free => (BadgeVariant::Secondary, "FREE"),
        UserTier::Premium => (BadgeVariant::Primary, "PREMIUM"),
        UserTier::Elite => (BadgeVariant::Destructive, "ELITE"),
    };

    rsx! {
        div { class: "sidebar-footer-row sidebar-tier-row",
            span { class: "sidebar-footer-label", "Tier" }
            Badge { variant: variant, "{label}" }
        }
    }
}
