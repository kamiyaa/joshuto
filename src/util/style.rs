use tui::style::Style;

use crate::fs::{FileType, JoshutoDirEntry, LinkType};
use crate::util::unix;

use crate::THEME_T;

pub fn entry_style(entry: &JoshutoDirEntry) -> Style {
    let metadata = &entry.metadata;
    let filetype = &metadata.file_type();
    let linktype = &metadata.link_type();

    if entry.is_selected() {
        Style::default()
            .fg(THEME_T.selection.fg)
            .bg(THEME_T.selection.bg)
            .add_modifier(THEME_T.selection.modifier)
    } else {
        match linktype {
            LinkType::Symlink(_) => Style::default()
                .fg(THEME_T.link.fg)
                .bg(THEME_T.link.bg)
                .add_modifier(THEME_T.link.modifier),
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
    let metadata = &entry.metadata;
    if unix::is_executable(metadata.mode) {
        Style::default()
            .fg(THEME_T.executable.fg)
            .bg(THEME_T.executable.bg)
            .add_modifier(THEME_T.executable.modifier)
    } else {
        match entry.file_path().extension() {
            None => Style::default(),
            Some(os_str) => match os_str.to_str() {
                None => Style::default(),
                Some(s) => match THEME_T.ext.get(s) {
                    None => Style::default(),
                    Some(t) => Style::default().fg(t.fg).bg(t.bg).add_modifier(t.modifier),
                },
            },
        }
    }
}
