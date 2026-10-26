use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::config::AppConfig;
use crate::fs::{AppDirEntry, AppDirectoryList};
#[cfg(feature = "devicons")]
use crate::fs::{AppFileMetadata, AppFileType};
#[cfg(feature = "devicons")]
use crate::global;
use crate::ui::widgets::trim_file_label;
use crate::utils::style_helper;

/// Renders a simple (non-detailed) directory listing: one entry name per line, current entry
/// reverse-highlighted when `focused`. Used for the parent and preview panes.
pub struct TuiDirList<'a> {
    pub config: &'a AppConfig,
    pub dirlist: &'a AppDirectoryList,
    pub focused: bool,
}

impl<'a> TuiDirList<'a> {
    /// Creates a directory-list widget for `dirlist`.
    pub fn new(config: &'a AppConfig, dirlist: &'a AppDirectoryList, focused: bool) -> Self {
        Self {
            config,
            dirlist,
            focused,
        }
    }
}

impl Widget for TuiDirList<'_> {
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
                    style_helper::entry_style(self.config, entry)
                } else if ix == curr_index {
                    style_helper::entry_style(self.config, entry).add_modifier(Modifier::REVERSED)
                } else {
                    style_helper::entry_style(self.config, entry)
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
    entry: &AppDirEntry,
    style: Style,
    (x, y): (u16, u16),
    drawing_width: usize,
) {
    let name = entry.file_name();
    #[cfg(feature = "devicons")]
    let (label, label_width) = {
        if config.display_options.show_icons {
            let icon = get_entry_icon(config, entry.file_name(), entry.ext(), &entry.metadata);
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
/// Looks up the icon glyph for an entry: exact-name match, falling back to extension, falling
/// back to the configured default file/directory icon.
pub fn get_entry_icon(
    config: &AppConfig,
    name: &str,
    ext: Option<&str>,
    metadata: &AppFileMetadata,
) -> &'static str {
    if let AppFileType::Directory = metadata.file_type() {
        return global::APP_ICONS
            .directory_exact
            .get(name)
            .map(|s| s.as_str())
            .unwrap_or(global::APP_ICONS.default_dir.as_str());
    }
    global::APP_ICONS
        .file_exact
        .get(name)
        .map(|s| s.as_str())
        .unwrap_or_else(|| {
            ext.and_then(|ext| {
                let ext: String = if config.case_insensitive_ext {
                    ext.to_lowercase()
                } else {
                    ext.to_owned()
                };
                global::APP_ICONS.ext.get(&ext).map(|s| s.as_str())
            })
            .unwrap_or_else(|| global::APP_ICONS.default_file.as_str())
        })
}
