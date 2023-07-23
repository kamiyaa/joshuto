use ratatui::style::Style;

use crate::fs::{FileType, JoshutoDirEntry, LinkType};
use crate::util::unix;

use crate::THEME_T;

pub fn entry_style(entry: &JoshutoDirEntry) -> Style {
    let metadata = &entry.metadata;
    let filetype = &metadata.file_type();
    let linktype = &metadata.link_type();

    if entry.is_visual_mode_selected() {
        Style::default()
            .fg(THEME_T.visual_mode_selection.fg)
            .bg(THEME_T.visual_mode_selection.bg)
            .add_modifier(THEME_T.visual_mode_selection.modifier)
    } else if entry.is_permanent_selected() {
        Style::default()
            .fg(THEME_T.selection.fg)
            .bg(THEME_T.selection.bg)
            .add_modifier(THEME_T.selection.modifier)
    } else {
        match linktype {
            LinkType::Symlink { valid: true, .. } => Style::default()
                .fg(THEME_T.link.fg)
                .bg(THEME_T.link.bg)
                .add_modifier(THEME_T.link.modifier),
            LinkType::Symlink { valid: false, .. } => Style::default()
                .fg(THEME_T.link_invalid.fg)
                .bg(THEME_T.link_invalid.bg)
                .add_modifier(THEME_T.link_invalid.modifier),
            LinkType::Normal => match filetype {
                FileType::Directory => Style::default()
                    .fg(THEME_T.directory.fg)
                    .bg(THEME_T.directory.bg)
                    .add_modifier(THEME_T.directory.modifier),
                FileType::File => file_style(entry),
            },
        }
    }
}

fn file_style(entry: &JoshutoDirEntry) -> Style {
    let regular_style = Style::default()
        .fg(THEME_T.regular.fg)
        .bg(THEME_T.regular.bg)
        .add_modifier(THEME_T.regular.modifier);
    let metadata = &entry.metadata;
    if unix::is_executable(metadata.mode) {
        Style::default()
            .fg(THEME_T.executable.fg)
            .bg(THEME_T.executable.bg)
            .add_modifier(THEME_T.executable.modifier)
    } else {
        entry
            .file_path()
            .extension()
            .and_then(|s| s.to_str())
            .and_then(|s| THEME_T.ext.get(s))
            .map(|theme| {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.bg)
                    .add_modifier(theme.modifier)
            })
            .unwrap_or(regular_style)
    }
}
