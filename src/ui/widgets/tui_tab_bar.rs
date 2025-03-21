use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget};

use tab_list_builder::factor_tab_bar_spans;

use crate::config::app::AppConfig;
use crate::config::theme::tab_raw::TabStyle;
use crate::tab::JoshutoTab;
use crate::ui::tab_list_builder::{self, TabLabel};
use crate::THEME_T;

pub struct TuiTabBar<'a> {
    pub config: &'a AppConfig,
    pub tabs: Vec<&'a JoshutoTab>,
    pub index: usize,
}

impl<'a> TuiTabBar<'a> {
    pub fn new(config: &'a AppConfig, tabs: Vec<&'a JoshutoTab>, index: usize) -> Self {
        Self {
            config,
            tabs,
            index,
        }
    }
}

impl Widget for TuiTabBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tab_labels: Vec<_> = match self.config.tab_options.style {
            TabStyle::FolderName => self
                .tabs
                .iter()
                .map(|tab| tab.get_cwd())
                .map(TabLabel::from_file_name)
                .collect(),
            TabStyle::FullPath => self
                .tabs
                .iter()
                .map(|tab| tab.get_cwd())
                .map(TabLabel::from_path)
                .collect(),
        };

        let tab_bar_spans =
            factor_tab_bar_spans(&tab_labels, area.width as usize, self.index, &THEME_T.tabs);
        Paragraph::new(Line::from(tab_bar_spans)).render(area, buf);
    }
}
