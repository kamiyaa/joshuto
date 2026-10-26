use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};
use unicode_width::UnicodeWidthStr;

use crate::config::theme::app_tab_theme_raw::{
    AppTabThemeCharsRaw, AppTabThemeColorRaw, AppTabThemeRaw,
};

fn style_merge_optional(s: Style, op: Option<Style>) -> Style {
    if let Some(other) = op {
        s.patch(other)
    } else {
        s
    }
}

/// Resolved styling for the tab bar: colors, decoration characters, and their inferred widths.
#[derive(Clone, Debug)]
pub struct AppTabTheme {
    pub styles: AppTabThemeColors,
    pub chars: AppTabThemeChars,
    pub inference: TabThemeCharsInference,
}

impl From<AppTabThemeRaw> for AppTabTheme {
    fn from(value: AppTabThemeRaw) -> Self {
        let chars = AppTabThemeChars::from(value.chars);
        Self {
            styles: AppTabThemeColors::from(value.styles),
            inference: TabThemeCharsInference::from_chars(&chars),
            chars,
        }
    }
}

/// Resolved decoration characters/strings for the tab bar (dividers, prefixes, scroll markers).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppTabThemeChars {
    pub divider: String,
    pub prefix_i: String,
    pub postfix_i: String,
    pub prefix_a: String,
    pub postfix_a: String,
    pub scroll_front_prefix: String,
    pub scroll_front_postfix: String,
    pub scroll_front_prestring: String,
    pub scroll_front_poststring: String,
    pub scroll_back_prefix: String,
    pub scroll_back_postfix: String,
    pub scroll_back_prestring: String,
    pub scroll_back_poststring: String,
    pub padding_prefix: char,
    pub padding_postfix: char,
    pub padding_fill: char,
}

impl From<AppTabThemeCharsRaw> for AppTabThemeChars {
    fn from(value: AppTabThemeCharsRaw) -> Self {
        Self {
            divider: value.divider.unwrap_or(" ".to_string()),
            prefix_i: value.inactive_prefix.unwrap_or("[".to_string()),
            postfix_i: value.inactive_postfix.unwrap_or("]".to_string()),
            prefix_a: value.active_prefix.unwrap_or(" ".to_string()),
            postfix_a: value.active_postfix.unwrap_or(" ".to_string()),
            scroll_front_prefix: value.scroll_front_prefix.unwrap_or("".to_string()),
            scroll_front_postfix: value.scroll_front_postfix.unwrap_or("".to_string()),
            scroll_front_prestring: value.scroll_front_prestring.unwrap_or("«".to_string()),
            scroll_front_poststring: value.scroll_front_poststring.unwrap_or(" ".to_string()),
            scroll_back_prefix: value.scroll_back_prefix.unwrap_or("".to_string()),
            scroll_back_postfix: value.scroll_back_postfix.unwrap_or("".to_string()),
            scroll_back_prestring: value.scroll_back_prestring.unwrap_or(" ".to_string()),
            scroll_back_poststring: value.scroll_back_poststring.unwrap_or("»".to_string()),
            padding_prefix: value.padding_prefix.unwrap_or(' '),
            padding_postfix: value.padding_postfix.unwrap_or(' '),
            padding_fill: value.padding_fill.unwrap_or(' '),
        }
    }
}

/// Display widths derived from [`TabThemeChars`], precomputed to avoid recalculating per frame.
#[derive(Clone, Debug)]
pub struct TabThemeCharsInference {
    pub tab_divider_length: usize,
    pub _tab_prefix_i_length: usize,
    pub _tab_postfix_i_length: usize,
    pub _tab_prefix_a_length: usize,
    pub _tab_postfix_a_length: usize,
    pub scroll_front_static_length: usize,
    pub scroll_back_static_length: usize,
    pub active_tab_extra_width: usize,
    pub inactive_tab_extra_width: usize,
}

impl TabThemeCharsInference {
    fn from_chars(chars: &AppTabThemeChars) -> Self {
        Self {
            tab_divider_length: chars.divider.width(),
            _tab_prefix_i_length: chars.prefix_i.width(),
            _tab_prefix_a_length: chars.prefix_a.width(),
            _tab_postfix_i_length: chars.postfix_i.width(),
            _tab_postfix_a_length: chars.postfix_a.width(),
            scroll_front_static_length: chars.scroll_front_prefix.width()
                + chars.scroll_front_postfix.width()
                + chars.scroll_front_prestring.width()
                + chars.scroll_front_poststring.width(),
            scroll_back_static_length: chars.scroll_back_prefix.width()
                + chars.scroll_back_postfix.width()
                + chars.scroll_back_prestring.width()
                + chars.scroll_back_poststring.width(),
            active_tab_extra_width: chars.prefix_a.width() + chars.postfix_a.width(),
            inactive_tab_extra_width: chars.prefix_i.width() + chars.postfix_i.width(),
        }
    }

    /// Returns the total display width of the scroll-position tags shown when there are more
    /// tabs than fit on screen, given `num_tabs` total tabs.
    pub fn calc_scroll_tags_width(&self, num_tabs: usize) -> usize {
        let max_num_width = num_tabs.checked_ilog10().unwrap_or(0) as usize + 1;
        2 * max_num_width + self.scroll_front_static_length + self.scroll_back_static_length
    }
}

/// Resolved styles for every element of the tab bar, with per-element overrides already
/// patched onto their base active/inactive/divider/scroll styles.
#[derive(Clone, Debug)]
pub struct AppTabThemeColors {
    pub prefix_a: Style,
    pub postfix_a: Style,
    pub tab_a: Style,
    pub prefix_i: Style,
    pub postfix_i: Style,
    pub tab_i: Style,
    pub divider_ii: Style,
    pub divider_ia: Style,
    pub divider_ai: Style,
    pub scroll_front_prefix: Style,
    pub scroll_front_postfix: Style,
    pub scroll_front: Style,
    pub scroll_back_prefix: Style,
    pub scroll_back_postfix: Style,
    pub scroll_back: Style,
    pub padding_prefix: Style,
    pub padding_postfix: Style,
    pub padding_fill: Style,
}

impl From<AppTabThemeColorRaw> for AppTabThemeColors {
    fn from(value: AppTabThemeColorRaw) -> Self {
        let tab_a = value.active.map(|s| s.into()).unwrap_or(
            Style::new()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
        let prefix_a = style_merge_optional(tab_a, value.active_prefix.map(|s| s.into()));
        let postfix_a = style_merge_optional(prefix_a, value.active_postfix.map(|s| s.into()));

        let tab_i = value.inactive.map(|s| s.into()).unwrap_or_default();
        let prefix_i = style_merge_optional(tab_i, value.inactive_prefix.map(|s| s.into()));
        let postfix_i = style_merge_optional(prefix_i, value.inactive_postfix.map(|s| s.into()));

        let divider_ii = value.divider_ii.map(|s| s.into()).unwrap_or_default();
        let divider_ia = style_merge_optional(divider_ii, value.divider_ia.map(|s| s.into()));
        let divider_ai = style_merge_optional(divider_ia, value.divider_ai.map(|s| s.into()));

        let scroll_front = value
            .scroll_front
            .map(|s| s.into())
            .unwrap_or(Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD));
        let scroll_front_prefix =
            style_merge_optional(scroll_front, value.scroll_front_prefix.map(|s| s.into()));
        let scroll_front_postfix = style_merge_optional(
            scroll_front_prefix,
            value.scroll_front_postfix.map(|s| s.into()),
        );

        let scroll_back = value
            .scroll_back
            .map(|s| s.into())
            .unwrap_or(Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD));
        let scroll_back_prefix =
            style_merge_optional(scroll_back, value.scroll_back_prefix.map(|s| s.into()));
        let scroll_back_postfix = style_merge_optional(
            scroll_back_prefix,
            value.scroll_back_postfix.map(|s| s.into()),
        );

        let padding_fill = value.padding_fill.map(|s| s.into()).unwrap_or_default();
        let padding_prefix =
            style_merge_optional(padding_fill, value.padding_prefix.map(|s| s.into()));
        let padding_postfix =
            style_merge_optional(padding_prefix, value.padding_postfix.map(|s| s.into()));
        Self {
            prefix_a,
            postfix_a,
            tab_a,
            prefix_i,
            postfix_i,
            tab_i,
            divider_ii,
            divider_ia,
            divider_ai,
            scroll_front_prefix,
            scroll_front_postfix,
            scroll_front,
            scroll_back_prefix,
            scroll_back_postfix,
            scroll_back,
            padding_prefix,
            padding_postfix,
            padding_fill,
        }
    }
}
