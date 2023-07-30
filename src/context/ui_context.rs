use ratatui::layout::Rect;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiContext {
    pub layout: Vec<Rect>,
}
