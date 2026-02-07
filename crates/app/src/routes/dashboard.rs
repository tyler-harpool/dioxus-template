use dioxus::prelude::*;
use server::api::get_dashboard_stats;
use shared_ui::{
    AspectRatio, Avatar, AvatarFallback, Badge, BadgeVariant, Button, ButtonVariant, Card,
    CardContent, CardDescription, CardHeader, CardTitle, HoverCard, HoverCardContent,
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
        div {
            style: "display: flex; flex-direction: column; gap: var(--space-lg);",

            h2 {
                style: "margin: 0; color: var(--color-on-surface); font-family: var(--cyber-font-mono);",
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
                                style: "color: var(--color-destructive);",
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
            style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: var(--space-md);",
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
            style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: var(--space-md);",

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
                    style: "display: flex; align-items: center; justify-content: space-between;",
                    CardTitle { "{title}" }
                    div {
                        style: "display: flex; align-items: center; gap: var(--space-xs);",
                        if let Some(label) = &badge_label {
                            Badge { variant: BadgeVariant::Primary, "{label}" }
                        }
                        Tooltip {
                            TooltipTrigger {
                                span {
                                    style: "display: inline-flex; align-items: center; justify-content: center; width: 1.25rem; height: 1.25rem; border-radius: 50%; background: var(--color-surface-raised); color: var(--color-on-surface-muted); font-size: 0.75rem; cursor: help;",
                                    "?"
                                }
                            }
                            TooltipContent { "{tooltip_text}" }
                        }
                    }
                }
            }
            CardContent {
                span {
                    style: "font-size: var(--font-size-2xl); font-weight: 700; color: var(--color-primary); font-family: var(--cyber-font-mono);",
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
                    style: "display: flex; flex-direction: column; gap: var(--space-lg);",

                    div {
                        style: "display: flex; flex-direction: column; gap: var(--space-xs);",
                        div {
                            style: "display: flex; justify-content: space-between; align-items: center;",
                            span {
                                style: "font-weight: 600; color: var(--color-on-surface);",
                                "Inventory Target"
                            }
                            span {
                                style: "color: var(--color-on-surface-muted); font-size: var(--font-size-sm); font-family: var(--cyber-font-mono);",
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
                        style: "display: flex; flex-direction: column; gap: var(--space-xs);",
                        div {
                            style: "display: flex; justify-content: space-between; align-items: center;",
                            span {
                                style: "font-weight: 600; color: var(--color-on-surface);",
                                "Active Products Ratio"
                            }
                            span {
                                style: "color: var(--color-on-surface-muted); font-size: var(--font-size-sm); font-family: var(--cyber-font-mono);",
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
                            style: "color: var(--color-on-surface-muted); text-align: center; padding: var(--space-lg);",
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
            style: "display: flex; align-items: center; gap: var(--space-md); padding: var(--space-sm) 0;",

            Avatar {
                AvatarFallback { "{fallback_initials}" }
            }

            HoverCard {
                HoverCardTrigger {
                    span {
                        style: "font-weight: 600; color: var(--color-on-surface); cursor: pointer; text-decoration: underline; text-decoration-style: dotted; text-underline-offset: 0.2em;",
                        "{user.display_name}"
                    }
                }
                HoverCardContent {
                    div {
                        style: "display: flex; flex-direction: column; gap: var(--space-sm); padding: var(--space-sm);",

                        div {
                            style: "width: 100%; max-width: 8rem;",
                            AspectRatio {
                                ratio: 1.0,
                                div {
                                    style: "width: 100%; height: 100%; background: var(--color-surface-raised); border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; color: var(--color-on-surface-muted); font-family: var(--cyber-font-mono);",
                                    "{fallback_initials}"
                                }
                            }
                        }

                        div {
                            style: "display: flex; flex-direction: column; gap: var(--space-2xs);",
                            span {
                                style: "font-weight: 700; color: var(--color-on-surface);",
                                "{user.display_name}"
                            }
                            span {
                                style: "color: var(--color-on-surface-muted); font-size: var(--font-size-sm);",
                                "@{user.username}"
                            }
                            span {
                                style: "color: var(--color-on-surface-muted); font-size: var(--font-size-xs); font-family: var(--cyber-font-mono);",
                                "ID: {user.id}"
                            }
                        }
                    }
                }
            }

            div { style: "flex: 1;" }

            span {
                style: "color: var(--color-on-surface-muted); font-size: var(--font-size-sm); font-family: var(--cyber-font-mono);",
                "@{user.username}"
            }
        }
    }
}
