use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use crate::config::TabDisplayOption;
use crate::fs::{AppDirectoryList, AppLinkType};
use crate::global;
use crate::utils::format;
use crate::utils::unix_helper;

/// The bottom status line: current entry's permissions, owner, group, position, mtime, size,
/// active flat/filter indicators, and selection count.
pub struct TuiFooter<'a> {
    dirlist: &'a AppDirectoryList,
    tab_options: &'a TabDisplayOption,
}

impl<'a> TuiFooter<'a> {
    /// Creates the footer widget for the given directory listing and tab options.
    pub fn new(dirlist: &'a AppDirectoryList, tab_options: &'a TabDisplayOption) -> Self {
        Self {
            dirlist,
            tab_options,
        }
    }
}

impl Widget for TuiFooter<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let visual_mode_style = Style::default().fg(Color::Black).bg(Color::LightRed);
        let mode_style = Style::default().fg(Color::Cyan);

        // flat and filter commands indicator style
        let indicator_style = Style::default()
            .fg(Color::LightBlue)
            .add_modifier(global::APP_THEME.selection.modifier);

        let selection_style = Style::default()
            .fg(global::APP_THEME.selection.fg)
            .bg(global::APP_THEME.selection.bg)
            .add_modifier(global::APP_THEME.selection.modifier);
        let selected_count = self.dirlist.selected_count();

        match self.dirlist.get_index() {
            Some(i) if i < self.dirlist.len() => {
                let entry = &self.dirlist.contents[i];

                let mode_str: String =
                    unix_helper::mode_to_char_array(entry.metadata.mode, entry.metadata.file_type)
                        .iter()
                        .collect();

                let user_str =
                    unix_helper::uid_to_string(entry.metadata.uid).unwrap_or("unknown".into());
                let group_str =
                    unix_helper::gid_to_string(entry.metadata.gid).unwrap_or("unknown".into());

                let mtime_str = format::time_to_string(entry.metadata.modified());
                let size_str = format::file_size_to_string(entry.metadata.len());

                let path = self.dirlist.file_path();

                let mut text = vec![
                    Span::styled(
                        if self.dirlist.get_visual_mode_anchor_index().is_none() {
                            ""
                        } else {
                            "VIS"
                        },
                        visual_mode_style,
                    ),
                    Span::raw(if self.dirlist.get_visual_mode_anchor_index().is_none() {
                        ""
                    } else {
                        " "
                    }),
                    Span::styled(mode_str, mode_style),
                    Span::raw("  "),
                    Span::raw(user_str),
                    Span::raw(" "),
                    Span::raw(group_str),
                    Span::raw("  "),
                    Span::raw(format!("{}/{}", i + 1, self.dirlist.len())),
                    Span::raw("  "),
                    Span::raw(mtime_str),
                    Span::raw(global::TIMEZONE_STR.as_str()),
                    Span::raw(size_str),
                    Span::raw("  "),
                    Span::styled(
                        match self.tab_options.dirlist_options.get(path) {
                            Some(opt) if opt.depth() > 0 => format!("flat:{} ", opt.depth()),
                            _ => "".to_owned(),
                        },
                        indicator_style,
                    ),
                    Span::styled(
                        match self.tab_options.dirlist_options.get(path) {
                            Some(opt) if !opt.filter_state.is_none() => {
                                format!("filter:{} ", opt.filter_state)
                            }
                            _ => "".to_owned(),
                        },
                        indicator_style,
                    ),
                    Span::styled(
                        if selected_count > 0 {
                            format!("{} selected", selected_count)
                        } else {
                            " ".to_string()
                        },
                        selection_style,
                    ),
                ];

                if let AppLinkType::Symlink { target, valid } = entry.metadata.link_type() {
                    let link_style = if *valid {
                        Style::default()
                            .fg(global::APP_THEME.link.fg)
                            .bg(global::APP_THEME.link.bg)
                            .add_modifier(global::APP_THEME.link.modifier)
                    } else {
                        Style::default()
                            .fg(global::APP_THEME.link_invalid.fg)
                            .bg(global::APP_THEME.link_invalid.bg)
                            .add_modifier(global::APP_THEME.link_invalid.modifier)
                    };
                    text.push(Span::styled(" -> ", link_style));
                    text.push(Span::styled(target, link_style));
                }

                Paragraph::new(Line::from(text)).render(area, buf);
            }
            _ => {}
        }
    }
}
