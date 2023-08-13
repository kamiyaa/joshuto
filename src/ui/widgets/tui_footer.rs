use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use crate::config::option::TabDisplayOption;
use crate::fs::{JoshutoDirList, LinkType};
use crate::util::format;
use crate::util::unix;
use crate::{THEME_T, TIMEZONE_STR};

pub struct TuiFooter<'a> {
    dirlist: &'a JoshutoDirList,
    tab_options: &'a TabDisplayOption,
}

impl<'a> TuiFooter<'a> {
    pub fn new(dirlist: &'a JoshutoDirList, tab_options: &'a TabDisplayOption) -> Self {
        Self {
            dirlist,
            tab_options,
        }
    }
}

impl<'a> Widget for TuiFooter<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        #[cfg(unix)]
        use std::os::unix::fs::PermissionsExt;

        let visual_mode_style = Style::default().fg(Color::Black).bg(Color::LightRed);
        let mode_style = Style::default().fg(Color::Cyan);

        // flat and filter commands indicator style
        let indicator_style = Style::default()
            .fg(Color::LightBlue)
            .add_modifier(THEME_T.selection.modifier);

        let selection_style = Style::default()
            .fg(THEME_T.selection.fg)
            .bg(THEME_T.selection.bg)
            .add_modifier(THEME_T.selection.modifier);
        let selected_count = self.dirlist.selected_count();

        match self.dirlist.get_index() {
            Some(i) if i < self.dirlist.len() => {
                let entry = &self.dirlist.contents[i];

                let mode_str = unix::mode_to_string(entry.metadata.permissions_ref().mode());

                let user_str = unix::uid_to_string(entry.metadata.uid).unwrap_or("unknown".into());
                let group_str = unix::gid_to_string(entry.metadata.gid).unwrap_or("unknown".into());

                let mtime_str = format::mtime_to_string(entry.metadata.modified());
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
                    Span::raw(TIMEZONE_STR.as_str()),
                    Span::raw(size_str),
                    Span::raw("  "),
                    Span::styled(
                        match self.tab_options.dirlist_options_ref(&path.to_path_buf()) {
                            Some(opt) if opt.depth() > 0 => format!("flat:{} ", opt.depth()),
                            _ => "".to_owned(),
                        },
                        indicator_style,
                    ),
                    Span::styled(
                        match self.tab_options.dirlist_options_ref(&path.to_path_buf()) {
                            Some(opt) if !opt.filter_context_ref().is_none() => {
                                format!("filter:{} ", opt.filter_context_ref())
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

                if let LinkType::Symlink { target, valid } = entry.metadata.link_type() {
                    let link_style = if *valid {
                        Style::default()
                            .fg(THEME_T.link.fg)
                            .bg(THEME_T.link.bg)
                            .add_modifier(THEME_T.link.modifier)
                    } else {
                        Style::default()
                            .fg(THEME_T.link_invalid.fg)
                            .bg(THEME_T.link_invalid.bg)
                            .add_modifier(THEME_T.link_invalid.modifier)
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
