pub mod dashboard;
pub mod products;
pub mod settings;
pub mod users;

use dioxus::prelude::*;
use shared_ui::{
    Avatar, AvatarFallback, Button, ButtonVariant, DropdownMenu, DropdownMenuContent,
    DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger, Navbar, Separator, Sidebar,
    SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel,
    SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider,
    SidebarRail, SidebarSeparator, SidebarTrigger, Switch, SwitchThumb,
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

    let page_title = match &route {
        Route::Dashboard {} => "Dashboard",
        Route::Users {} => "Users",
        Route::Products {} => "Products",
        Route::Settings {} => "Settings",
    };

    rsx! {
        SidebarProvider {
            Sidebar {
                SidebarHeader {
                    div {
                        style: "display: flex; align-items: center; gap: var(--space-sm); padding: var(--space-sm);",
                        span {
                            style: "font-size: var(--font-size-lg); font-weight: 700; color: var(--color-primary); font-family: var(--cyber-font-mono);",
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
                        style: "display: flex; align-items: center; gap: var(--space-sm); padding: var(--space-xs);",
                        span {
                            style: "font-size: var(--font-size-sm); color: var(--color-on-surface-muted);",
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
                        style: "display: flex; align-items: center; gap: var(--space-md); width: 100%; padding: 0 var(--space-md);",

                        SidebarTrigger {
                            span { style: "font-size: var(--font-size-lg);", "\u{2630}" }
                        }

                        Separator { horizontal: false }

                        span {
                            style: "font-weight: 600; font-size: var(--font-size-md); color: var(--color-on-surface);",
                            "{page_title}"
                        }

                        // Spacer
                        div { style: "flex: 1;" }

                        // User dropdown
                        DropdownMenu {
                            DropdownMenuTrigger {
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    Avatar {
                                        AvatarFallback { "AD" }
                                    }
                                }
                            }
                            DropdownMenuContent {
                                DropdownMenuItem::<String> {
                                    value: "profile".to_string(),
                                    index: 0usize,
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
                                        style: "width: 100%; cursor: pointer;",
                                        "API Docs"
                                    }
                                }
                            }
                        }
                    }
                }

                // Page content
                div {
                    style: "padding: var(--space-lg); max-width: 1400px; margin: 0 auto; width: 100%;",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
