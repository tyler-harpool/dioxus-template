// Phase 1: Standalone components (no primitives)
pub mod skeleton;
pub mod badge;
pub mod button;
pub mod input;
pub mod textarea;
pub mod card;
pub mod form;
pub mod sheet;

// Phase 2A: Simple primitive wrappers
pub mod separator;
pub mod label;
pub mod progress;
pub mod aspect_ratio;
pub mod toggle;
pub mod switch;
pub mod checkbox;

// Phase 2B: Compound primitive wrappers
pub mod accordion;
pub mod tabs;
pub mod collapsible;
pub mod radio_group;
pub mod toggle_group;
pub mod toolbar;
pub mod scroll_area;

// Phase 2C: Overlay/popup wrappers
pub mod tooltip;
pub mod popover;
pub mod hover_card;
pub mod dialog;
pub mod alert_dialog;
pub mod context_menu;
pub mod dropdown_menu;

// Phase 2D: Navigation & complex
pub mod menubar;
pub mod navbar;
pub mod select;
pub mod slider;

// Phase 2E: Special
pub mod avatar;
pub mod toast;
pub mod calendar;
pub mod date_picker;

// Phase 1 (last): Depends on button, sheet, separator, tooltip
pub mod sidebar;

// Re-exports for convenience
pub use skeleton::*;
pub use badge::*;
pub use button::*;
pub use input::*;
pub use textarea::*;
pub use card::*;
pub use form::*;
pub use sheet::*;
pub use separator::*;
pub use label::*;
pub use progress::*;
pub use aspect_ratio::*;
pub use toggle::*;
pub use switch::*;
pub use checkbox::*;
pub use accordion::*;
pub use tabs::*;
pub use collapsible::*;
pub use radio_group::*;
pub use toggle_group::*;
pub use toolbar::*;
pub use scroll_area::*;
pub use tooltip::*;
pub use popover::*;
pub use hover_card::*;
pub use dialog::*;
pub use alert_dialog::*;
pub use context_menu::*;
pub use dropdown_menu::*;
pub use menubar::*;
pub use navbar::*;
pub use select::*;
pub use slider::*;
pub use avatar::*;
pub use toast::*;
pub use calendar::*;
pub use date_picker::*;
pub use sidebar::*;
