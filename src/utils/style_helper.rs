use std::path::Path;

use ansi_to_tui::IntoText;
use colors_transform::{Color, Rgb};
use lscolors::LsColors;
use ratatui::style::{self, Style};

use crate::fs::{AppFileType, AppLinkType, GitFileStatus};
use crate::global;
use crate::utils::unix_helper;
use crate::{config::AppConfig, fs::AppDirEntry};

pub fn str_to_color(s: &str) -> style::Color {
    match s {
        "black" => style::Color::Black,
        "red" => style::Color::Red,
        "green" => style::Color::Green,
        "yellow" => style::Color::Yellow,
        "blue" => style::Color::Blue,
        "magenta" => style::Color::Magenta,
        "cyan" => style::Color::Cyan,
        "gray" => style::Color::Gray,
        "dark_gray" => style::Color::DarkGray,
        "light_red" => style::Color::LightRed,
        "light_green" => style::Color::LightGreen,
        "light_yellow" => style::Color::LightYellow,
        "light_blue" => style::Color::LightBlue,
        "light_magenta" => style::Color::LightMagenta,
        "light_cyan" => style::Color::LightCyan,
        "white" => style::Color::White,
        "reset" | "" => style::Color::Reset,
        s if s.starts_with('#') => {
            let rgb = match Rgb::from_hex_str(s) {
                Ok(s) => s,
                _ => return style::Color::Reset,
            };
            let r = rgb.get_red() as u8;
            let g = rgb.get_green() as u8;
            let b = rgb.get_blue() as u8;
            style::Color::Rgb(r, g, b)
        }
        s => match s.parse::<Rgb>() {
            Ok(rgb) => {
                let r = rgb.get_red() as u8;
                let g = rgb.get_green() as u8;
                let b = rgb.get_blue() as u8;
                style::Color::Rgb(r, g, b)
            }
            Err(_) => style::Color::Reset,
        },
    }
}

/// Returns the display style for `entry`: selection highlight takes priority, then LS_COLORS
/// (if enabled), then the theme's built-in per-type/per-extension styles.
pub fn entry_style(config: &AppConfig, entry: &AppDirEntry) -> style::Style {
    let metadata = &entry.metadata;
    let filetype = metadata.file_type();
    let linktype = metadata.link_type();

    if entry.is_visual_mode_selected() {
        return visual_mode_selected_style();
    }
    if entry.is_permanent_selected() {
        return permanent_selected_style();
    }

    match &global::APP_THEME.lscolors {
        Some(lscolors) => {
            let path = entry.file_path();
            lscolors_style(lscolors, path)
                .unwrap_or_else(|| default_style(config, entry, linktype, filetype))
        }
        None => default_style(config, entry, linktype, filetype),
    }
}

/// Returns the themed prefix string to display before `entry`'s name, based on its selection state.
pub fn entry_prefix(entry: &AppDirEntry) -> &str {
    if entry.is_visual_mode_selected() {
        &global::APP_THEME.visual_mode_selection.prefix
    } else if entry.is_permanent_selected() {
        &global::APP_THEME.selection.prefix
    } else {
        ""
    }
}

/// Style for a `git status --short` indicator, chosen by the kind of change.
pub fn git_status_style(status: GitFileStatus) -> Style {
    let theme = if status.is_untracked() {
        &global::APP_THEME.git_untracked
    } else if status.is_deleted() {
        &global::APP_THEME.git_deleted
    } else if status.is_staged_only() {
        &global::APP_THEME.git_staged
    } else {
        &global::APP_THEME.git_modified
    };
    Style::default()
        .fg(theme.fg)
        .bg(theme.bg)
        .add_modifier(theme.modifier)
}

fn default_style(
    config: &AppConfig,
    entry: &AppDirEntry,
    linktype: &AppLinkType,
    filetype: AppFileType,
) -> style::Style {
    match linktype {
        AppLinkType::Symlink { valid: true, .. } => Style::default()
            .fg(global::APP_THEME.link.fg)
            .bg(global::APP_THEME.link.bg)
            .add_modifier(global::APP_THEME.link.modifier),
        AppLinkType::Symlink { valid: false, .. } => Style::default()
            .fg(global::APP_THEME.link_invalid.fg)
            .bg(global::APP_THEME.link_invalid.bg)
            .add_modifier(global::APP_THEME.link_invalid.modifier),
        AppLinkType::Normal => match filetype {
            AppFileType::Directory => Style::default()
                .fg(global::APP_THEME.directory.fg)
                .bg(global::APP_THEME.directory.bg)
                .add_modifier(global::APP_THEME.directory.modifier),
            AppFileType::File => file_style(config, entry),
            AppFileType::Link => Style::default()
                .fg(global::APP_THEME.link.fg)
                .bg(global::APP_THEME.link.bg)
                .add_modifier(global::APP_THEME.link.modifier),
            _ => Style::default()
                .fg(global::APP_THEME.socket.fg)
                .bg(global::APP_THEME.socket.bg)
                .add_modifier(global::APP_THEME.socket.modifier),
        },
    }
}

fn visual_mode_selected_style() -> style::Style {
    style::Style::default()
        .fg(global::APP_THEME.visual_mode_selection.fg)
        .bg(global::APP_THEME.visual_mode_selection.bg)
        .add_modifier(global::APP_THEME.visual_mode_selection.modifier)
}

fn permanent_selected_style() -> style::Style {
    style::Style::default()
        .fg(global::APP_THEME.selection.fg)
        .bg(global::APP_THEME.selection.bg)
        .add_modifier(global::APP_THEME.selection.modifier)
}

fn file_style(config: &AppConfig, entry: &AppDirEntry) -> style::Style {
    let regular_style = style::Style::default()
        .fg(global::APP_THEME.regular.fg)
        .bg(global::APP_THEME.regular.bg)
        .add_modifier(global::APP_THEME.regular.modifier);
    let metadata = &entry.metadata;
    if unix_helper::is_executable(metadata.mode) {
        style::Style::default()
            .fg(global::APP_THEME.executable.fg)
            .bg(global::APP_THEME.executable.bg)
            .add_modifier(global::APP_THEME.executable.modifier)
    } else {
        entry
            .ext()
            .and_then(|s| {
                if config.case_insensitive_ext {
                    global::APP_THEME.ext.get(&s.to_lowercase())
                } else {
                    global::APP_THEME.ext.get(s)
                }
            })
            .map(|theme| {
                style::Style::default()
                    .fg(theme.fg)
                    .bg(theme.bg)
                    .add_modifier(theme.modifier)
            })
            .unwrap_or(regular_style)
    }
}

fn lscolors_style(lscolors: &LsColors, path: &Path) -> Option<style::Style> {
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
