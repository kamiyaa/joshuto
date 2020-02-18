use tui::layout::Constraint;

mod tui_backend;
pub mod widgets;

pub use tui_backend::*;

pub const DEFAULT_LAYOUT: [tui::layout::Constraint; 3] = [
    Constraint::Ratio(1, 8),
    Constraint::Ratio(3, 8),
    Constraint::Ratio(4, 8),
];

pub const NO_PREVIEW_LAYOUT: [tui::layout::Constraint; 3] = [
    Constraint::Ratio(1, 8),
    Constraint::Ratio(7, 8),
    Constraint::Ratio(0, 8),
];
