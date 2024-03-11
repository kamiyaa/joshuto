use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::config::clean::app::AppConfig;
use crate::fs::{FileType, JoshutoMetadata};
use crate::fs::{JoshutoDirEntry, JoshutoDirList};
use crate::ui::widgets::trim_file_label;
use crate::util::style;

pub struct TuiDirList<'a> {
    pub config: &'a AppConfig,
    pub dirlist: &'a JoshutoDirList,
    pub focused: bool,
}

impl<'a> TuiDirList<'a> {
    pub fn new(config: &'a AppConfig, dirlist: &'a JoshutoDirList, focused: bool) -> Self {
        Self {
            config,
            dirlist,
            focused,
        }
    }
}

impl<'a> Widget for TuiDirList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 4 || area.height < 1 {
            return;
        }
        let x = area.left();
        let y = area.top();

        if self.dirlist.contents.is_empty() {
            let style = Style::default().bg(Color::Red).fg(Color::White);
            buf.set_stringn(x, y, "empty", area.width as usize, style);
            return;
        }

        let curr_index = self.dirlist.get_index().unwrap();
        let skip_dist = self.dirlist.first_index_for_viewport();

        let drawing_width = area.width as usize;

        let space_fill = " ".repeat(drawing_width);

        self.dirlist
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
            .for_each(|(i, entry)| {
                let ix = skip_dist + i;

                let style = if !self.focused {
                    style::entry_style(entry)
                } else if ix == curr_index {
                    style::entry_style(entry).add_modifier(Modifier::REVERSED)
                } else {
                    style::entry_style(entry)
                };

                buf.set_string(x, y + i as u16, space_fill.as_str(), style);

                print_entry(
                    self.config,
                    buf,
                    entry,
                    style,
                    (x + 1, y + i as u16),
                    drawing_width - 1,
                );
            });
    }
}

fn print_entry(
    config: &AppConfig,
    buf: &mut Buffer,
    entry: &JoshutoDirEntry,
    style: Style,
    (x, y): (u16, u16),
    drawing_width: usize,
) {
    let name = entry.file_name();
    #[cfg(feature = "devicons")]
    let (label, label_width) = {
        if config.display_options_ref().show_icons() {
            let icon = get_entry_icon(&config, entry.file_name(), entry.ext(), &entry.metadata);
            let label = format!("{icon} {name}");
            let label_width = label.width();
            (label, label_width)
        } else {
            (name.to_string(), name.width())
        }
    };

    #[cfg(not(feature = "devicons"))]
    let (label, label_width) = {
        let label = name.to_string();
        let label_width = label.width();
        (label, label_width)
    };

    let label = if label_width > drawing_width {
        trim_file_label(&label, drawing_width)
    } else {
        label.to_string()
    };
    buf.set_string(x, y, label, style);
}

#[cfg(feature = "devicons")]
pub fn get_entry_icon(
    config: &AppConfig,
    name: &str,
    ext: Option<&str>,
    metadata: &JoshutoMetadata,
) -> &'static str {
    use crate::ICONS_T;

    if let FileType::Directory = metadata.file_type() {
        return ICONS_T
            .directory_exact
            .get(name)
            .map(|s| s.as_str())
            .unwrap_or(ICONS_T.default_dir.as_str());
    }
    ICONS_T
        .file_exact
        .get(name)
        .map(|s| s.as_str())
        .unwrap_or_else(|| {
            ext.and_then(|ext| {
                let ext: String = if config.case_sensitive_ext {
                    ext.to_owned()
                } else {
                    ext.to_lowercase()
                };
                ICONS_T.ext.get(&ext).map(|s| s.as_str())
            })
            .unwrap_or_else(|| ICONS_T.default_file.as_str())
        })
}
