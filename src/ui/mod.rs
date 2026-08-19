//! Terminal backend setup and the widgets/views that render joshuto's TUI.

mod backend;
mod preview_area;
mod rect;
mod tab_list_builder;

pub mod views;
pub mod widgets;

pub use backend::*;
pub use preview_area::*;
pub use rect::*;
