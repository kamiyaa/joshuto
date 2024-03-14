use ansi_to_tui::IntoText;
use lscolors::LsColors;
use ratatui::style::Style;
use std::path::Path;

use crate::config::clean::app::AppConfig;
use crate::fs::{FileType, JoshutoDirEntry, LinkType};
use crate::util::unix;

use crate::THEME_T;

/// Allows patching a ratatui style if there is `Some` style, otherwise returns a clone.
pub trait PathStyleIfSome {
    fn patch_optionally(&self, other: Option<Style>) -> Style;
}

/// Path a ratatui style with another optional style.
/// If the other optional style is `None`, a clone of the original style is returned.
impl PathStyleIfSome for Style {
    fn patch_optionally(&self, other_option: Option<Style>) -> Style {
        if let Some(other) = other_option {
            self.patch(other)
        } else {
            *self
        }
    }
}

pub fn entry_style(config: &AppConfig, entry: &JoshutoDirEntry) -> Style {
    let metadata = &entry.metadata;
    let filetype = metadata.file_type();
    let linktype = metadata.link_type();

    if entry.is_visual_mode_selected() {
        return visual_mode_selected_style();
    }
    if entry.is_permanent_selected() {
        return permanent_selected_style();
    }

    match &THEME_T.lscolors {
        Some(lscolors) => {
            let path = entry.file_path();
            lscolors_style(lscolors, path)
                .unwrap_or_else(|| default_style(config, entry, linktype, filetype))
        }
        None => default_style(config, entry, linktype, filetype),
    }
}

pub fn entry_prefix(entry: &JoshutoDirEntry) -> &str {
    if entry.is_visual_mode_selected() {
        &THEME_T.visual_mode_selection.prefix
    } else if entry.is_permanent_selected() {
        &THEME_T.selection.prefix
    } else {
        ""
    }
}

fn default_style(
    config: &AppConfig,
    entry: &JoshutoDirEntry,
    linktype: &LinkType,
    filetype: &FileType,
) -> Style {
    match linktype {
        LinkType::Symlink { valid: true, .. } => symlink_valid_style(),
        LinkType::Symlink { valid: false, .. } => symlink_invalid_style(),
        LinkType::Normal => match filetype {
            FileType::Directory => directory_style(),
            FileType::File => file_style(config, entry),
        },
    }
}

fn visual_mode_selected_style() -> Style {
    Style::default()
        .fg(THEME_T.visual_mode_selection.fg)
        .bg(THEME_T.visual_mode_selection.bg)
        .add_modifier(THEME_T.visual_mode_selection.modifier)
}

fn permanent_selected_style() -> Style {
    Style::default()
        .fg(THEME_T.selection.fg)
        .bg(THEME_T.selection.bg)
        .add_modifier(THEME_T.selection.modifier)
}

fn symlink_valid_style() -> Style {
    Style::default()
        .fg(THEME_T.link.fg)
        .bg(THEME_T.link.bg)
        .add_modifier(THEME_T.link.modifier)
}

fn symlink_invalid_style() -> Style {
    Style::default()
        .fg(THEME_T.link_invalid.fg)
        .bg(THEME_T.link_invalid.bg)
        .add_modifier(THEME_T.link_invalid.modifier)
}

fn directory_style() -> Style {
    Style::default()
        .fg(THEME_T.directory.fg)
        .bg(THEME_T.directory.bg)
        .add_modifier(THEME_T.directory.modifier)
}

fn file_style(config: &AppConfig, entry: &JoshutoDirEntry) -> Style {
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
            .ext()
            .and_then(|s| {
                if config.case_insensitive_ext {
                    THEME_T.ext.get(&s.to_lowercase())
                } else {
                    THEME_T.ext.get(s)
                }
            })
            .map(|theme| {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.bg)
                    .add_modifier(theme.modifier)
            })
            .unwrap_or(regular_style)
    }
}

fn lscolors_style(lscolors: &LsColors, path: &Path) -> Option<Style> {
    let nu_ansi_term_style = lscolors.style_for_path(path)?.to_nu_ansi_term_style();
    // Paths that are not valid UTF-8 are not styled by LS_COLORS.
    let str = path.to_str()?;
    let text = nu_ansi_term_style
        .paint(str)
        .to_string()
        .into_bytes()
        .into_text()
        .ok()?;
    // Extract the first Style from the returned Text.
    let style = text.lines.first()?.spans.first()?.style;
    Some(style)
}
