pub mod dashboard;
pub mod products;
pub mod settings;
pub mod users;

use crate::ProfileState;
use dioxus::prelude::*;
use shared_ui::{
    Avatar, AvatarFallback, DropdownMenu, DropdownMenuContent, DropdownMenuItem,
    DropdownMenuSeparator, DropdownMenuTrigger, Navbar, Separator, Sidebar, SidebarContent,
    SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader,
    SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarRail,
    SidebarSeparator, SidebarTrigger, Switch, SwitchThumb,
};

use dashboard::Dashboard;
use products::Products;
use settings::Settings;
use users::Users;

/// Application routes.
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Dashboard {},
    #[route("/users")]
    Users {},
    #[route("/products")]
    Products {},
    #[route("/settings")]
    Settings {},
}

/// Main app layout with sidebar and top navbar.
#[component]
fn AppLayout() -> Element {
    let route: Route = use_route();
    let mut theme_is_light = use_signal(|| false);
    let profile: ProfileState = use_context();

    let page_title = match &route {
        Route::Dashboard {} => "Dashboard",
        Route::Users {} => "Users",
        Route::Products {} => "Products",
        Route::Settings {} => "Settings",
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
                    div {
                        class: "sidebar-footer-row",
                        span {
                            class: "sidebar-footer-label",
                            "Light Mode"
                        }
                        Switch {
                            checked: theme_is_light(),
                            on_checked_change: move |checked: bool| {
                                theme_is_light.set(checked);
                                if checked {
                                    shared_ui::theme::set_theme("light");
                                } else {
                                    shared_ui::theme::set_theme("cyberpunk");
                                }
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
