use dioxus::prelude::*;
use server::api::get_dashboard_stats;
use shared_ui::{
    AspectRatio, Avatar, AvatarFallback, Badge, BadgeVariant, Button, ButtonVariant, Card,
    CardContent, CardDescription, CardHeader, CardTitle, ContentSide, HoverCard, HoverCardContent,
    HoverCardTrigger, Progress, ProgressIndicator, Separator, Skeleton, Tooltip, TooltipContent,
    TooltipTrigger,
};

/// Maximum value for progress bar display.
const PROGRESS_MAX: f64 = 100.0;

/// Number of skeleton placeholders shown while data is loading.
const SKELETON_COUNT: usize = 4;

/// Calculate a percentage from a numerator and denominator, capped at PROGRESS_MAX.
fn calc_percentage(numerator: i64, denominator: i64) -> f64 {
    if denominator == 0 {
        return 0.0;
    }
    let pct = (numerator as f64 / denominator as f64) * PROGRESS_MAX;
    pct.min(PROGRESS_MAX)
}

/// Extract the first two characters of a display name, uppercased, for avatar initials.
fn initials_from_name(name: &str) -> String {
    name.chars().take(2).collect::<String>().to_uppercase()
}

/// Dashboard page displaying stats, progress bars, and recent user activity.
#[component]
pub fn Dashboard() -> Element {
    let mut stats_resource = use_server_future(get_dashboard_stats)?;

    let stats_result = stats_resource();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./dashboard.css") }

        div {
            class: "dashboard-page",

            h2 {
                class: "dashboard-title",
                "Dashboard"
            }

            match stats_result {
                None => rsx! { LoadingSkeletons {} },

                Some(Err(err)) => rsx! {
                    Card {
                        CardHeader {
                            CardTitle { "Error" }
                            CardDescription { "Failed to load dashboard data." }
                        }
                        CardContent {
                            p {
                                class: "dashboard-error-text",
                                "{err}"
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                onclick: move |_| { stats_resource.restart(); },
                                "Retry"
                            }
                        }
                    }
                },

                Some(Ok(stats)) => rsx! {
                    StatsGrid { stats: stats.clone() }
                    ProgressSection { stats: stats.clone() }
                    RecentActivity { stats: stats.clone() }
                },
            }
        }
    }
}

/// Grid of skeleton placeholders shown during initial data load.
#[component]
fn LoadingSkeletons() -> Element {
    rsx! {
        div {
            class: "skeleton-grid",
            for _ in 0..SKELETON_COUNT {
                Card {
                    CardHeader {
                        Skeleton { style: "height: 1rem; width: 60%;" }
                    }
                    CardContent {
                        Skeleton { style: "height: 2rem; width: 40%;" }
                    }
                }
            }
        }
    }
}

/// Row of four stat cards displayed in a responsive CSS grid.
#[component]
fn StatsGrid(stats: shared_types::DashboardStats) -> Element {
    let growth_rate = calc_percentage(stats.active_products, stats.total_products);

    rsx! {
        div {
            class: "stats-grid",

            StatCard {
                title: "Total Users",
                value: "{stats.total_users}",
                tooltip_text: "The total number of registered user accounts.",
                badge_label: "live",
            }

            StatCard {
                title: "Total Products",
                value: "{stats.total_products}",
                tooltip_text: "Total products across all categories and statuses.",
            }

            StatCard {
                title: "Active Products",
                value: "{stats.active_products}",
                tooltip_text: "Products currently marked as active in the catalog.",
            }

            StatCard {
                title: "Growth Rate",
                value: "{growth_rate:.1}%",
                tooltip_text: "Percentage of products that are currently active.",
            }
        }
    }
}

/// A single stat card with an optional badge and a tooltip on an info icon.
#[component]
fn StatCard(
    title: String,
    value: String,
    tooltip_text: String,
    #[props(default)] badge_label: Option<String>,
) -> Element {
    rsx! {
        Card {
            CardHeader {
                div {
                    class: "stat-header-row",
                    CardTitle { "{title}" }
                    div {
                        class: "stat-actions",
                        if let Some(label) = &badge_label {
                            Badge { variant: BadgeVariant::Primary, "{label}" }
                        }
                        Tooltip {
                            TooltipTrigger {
                                span {
                                    class: "stat-info-icon",
                                    "?"
                                }
                            }
                            TooltipContent { side: ContentSide::Bottom, "{tooltip_text}" }
                        }
                    }
                }
            }
            CardContent {
                span {
                    class: "stat-value",
                    "{value}"
                }
            }
        }
    }
}

/// Section with two progress bars: inventory target and active products ratio.
#[component]
fn ProgressSection(stats: shared_types::DashboardStats) -> Element {
    let inventory_pct = calc_percentage(stats.active_products, stats.total_products);
    let active_ratio = calc_percentage(stats.active_products, stats.total_products);

    rsx! {
        Card {
            CardHeader {
                CardTitle { "Progress Overview" }
                CardDescription { "Key inventory and product metrics at a glance." }
            }
            CardContent {
                div {
                    class: "progress-stack",

                    div {
                        class: "progress-row",
                        div {
                            class: "progress-label-row",
                            span {
                                class: "progress-label",
                                "Inventory Target"
                            }
                            span {
                                class: "progress-value",
                                "{stats.active_products} / {stats.total_products}"
                            }
                        }
                        Progress {
                            value: Some(inventory_pct),
                            ProgressIndicator {}
                        }
                    }

                    Separator {}

                    div {
                        class: "progress-row",
                        div {
                            class: "progress-label-row",
                            span {
                                class: "progress-label",
                                "Active Products Ratio"
                            }
                            span {
                                class: "progress-value",
                                "{active_ratio:.1}%"
                            }
                        }
                        Progress {
                            value: Some(active_ratio),
                            ProgressIndicator {}
                        }
                    }
                }
            }
        }
    }
}

/// Card listing recent user activity in a scrollable area with hover cards.
#[component]
fn RecentActivity(stats: shared_types::DashboardStats) -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Recent Activity" }
                CardDescription { "Newly registered users." }
            }
            CardContent {
                div {
                    for (idx, user) in stats.recent_users.iter().enumerate() {
                        if idx > 0 {
                            Separator {}
                        }
                        UserRow { user: user.clone() }
                    }
                    if stats.recent_users.is_empty() {
                        p {
                            class: "empty-text",
                            "No recent users."
                        }
                    }
                }
            }
        }
    }
}

/// A single user row with avatar, hover card, and user details.
#[component]
fn UserRow(user: shared_types::User) -> Element {
    let fallback_initials = initials_from_name(&user.display_name);

    rsx! {
        div {
            class: "user-row",

            Avatar {
                AvatarFallback { "{fallback_initials}" }
            }

            HoverCard {
                HoverCardTrigger {
                    span {
                        class: "user-name-link",
                        "{user.display_name}"
                    }
                }
                HoverCardContent {
                    div {
                        class: "hover-card-body",

                        div {
                            class: "hover-card-avatar-wrap",
                            AspectRatio {
                                ratio: 1.0,
                                div {
                                    class: "hover-card-avatar-placeholder",
                                    "{fallback_initials}"
                                }
                            }
                        }

                        div {
                            class: "hover-card-details",
                            span {
                                class: "hover-card-name",
                                "{user.display_name}"
                            }
                            span {
                                class: "hover-card-username",
                                "@{user.username}"
                            }
                            span {
                                class: "hover-card-id",
                                "ID: {user.id}"
                            }
                        }
                    }
                }
            }

            div { class: "user-row-spacer" }

            span {
                class: "hide-mobile user-row-username",
                "@{user.username}"
            }
        }
    }
}
