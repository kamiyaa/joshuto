use crate::config::raw::theme::tab::{TabThemeCharsRaw, TabThemeColorRaw, TabThemeRaw};
use crate::util::style::PathStyleIfSome;
use ratatui::style::{Color, Modifier, Style};
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Debug)]
pub struct TabTheme {
    pub styles: TabThemeColors,
    pub chars: TabThemeChars,
    pub inference: TabThemeCharsInference,
}

impl From<TabThemeRaw> for TabTheme {
    fn from(crude: TabThemeRaw) -> Self {
        let chars = TabThemeChars::from(crude.chars);
        Self {
            styles: TabThemeColors::from(crude.styles),
            inference: TabThemeCharsInference::from_chars(&chars),
            chars,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TabThemeChars {
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

impl From<TabThemeCharsRaw> for TabThemeChars {
    fn from(crude: TabThemeCharsRaw) -> Self {
        Self {
            divider: crude.divider.unwrap_or(" ".to_string()),
            prefix_i: crude.inactive_prefix.unwrap_or("[".to_string()),
            postfix_i: crude.inactive_postfix.unwrap_or("]".to_string()),
            prefix_a: crude.active_prefix.unwrap_or(" ".to_string()),
            postfix_a: crude.active_postfix.unwrap_or(" ".to_string()),
            scroll_front_prefix: crude.scroll_front_prefix.unwrap_or("".to_string()),
            scroll_front_postfix: crude.scroll_front_postfix.unwrap_or("".to_string()),
            scroll_front_prestring: crude.scroll_front_prestring.unwrap_or("«".to_string()),
            scroll_front_poststring: crude.scroll_front_poststring.unwrap_or(" ".to_string()),
            scroll_back_prefix: crude.scroll_back_prefix.unwrap_or("".to_string()),
            scroll_back_postfix: crude.scroll_back_postfix.unwrap_or("".to_string()),
            scroll_back_prestring: crude.scroll_back_prestring.unwrap_or(" ".to_string()),
            scroll_back_poststring: crude.scroll_back_poststring.unwrap_or("»".to_string()),
            padding_prefix: crude.padding_prefix.unwrap_or(' '),
            padding_postfix: crude.padding_postfix.unwrap_or(' '),
            padding_fill: crude.padding_fill.unwrap_or(' '),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TabThemeCharsInference {
    pub tab_divider_length: usize,
    pub tab_prefix_i_length: usize,
    pub tab_postfix_i_length: usize,
    pub tab_prefix_a_length: usize,
    pub tab_postfix_a_length: usize,
    pub scroll_front_static_length: usize,
    pub scroll_back_static_length: usize,
    pub active_tab_extra_width: usize,
    pub inactive_tab_extra_width: usize,
}

impl TabThemeCharsInference {
    fn from_chars(chars: &TabThemeChars) -> Self {
        Self {
            tab_divider_length: chars.divider.width(),
            tab_prefix_i_length: chars.prefix_i.width(),
            tab_prefix_a_length: chars.prefix_a.width(),
            tab_postfix_i_length: chars.postfix_i.width(),
            tab_postfix_a_length: chars.postfix_a.width(),
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

    pub fn calc_scroll_tags_width(&self, num_tabs: usize) -> usize {
        let max_num_width = num_tabs.checked_ilog10().unwrap_or(0) as usize + 1;
        2 * max_num_width + self.scroll_front_static_length + self.scroll_back_static_length
    }
}

#[derive(Clone, Debug)]
pub struct TabThemeColors {
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

impl From<TabThemeColorRaw> for TabThemeColors {
    fn from(crude: TabThemeColorRaw) -> Self {
        let tab_a = crude.active.map(|s| s.as_style()).unwrap_or(
            Style::new()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
        let prefix_a = tab_a.patch_optionally(crude.active_prefix.map(|s| s.as_style()));
        let postfix_a = prefix_a.patch_optionally(crude.active_postfix.map(|s| s.as_style()));

        let tab_i = crude.inactive.map(|s| s.as_style()).unwrap_or_default();
        let prefix_i = tab_i.patch_optionally(crude.inactive_prefix.map(|s| s.as_style()));
        let postfix_i = prefix_i.patch_optionally(crude.inactive_postfix.map(|s| s.as_style()));

        let divider_ii = crude.divider_ii.map(|s| s.as_style()).unwrap_or_default();
        let divider_ia = divider_ii.patch_optionally(crude.divider_ia.map(|s| s.as_style()));
        let divider_ai = divider_ia.patch_optionally(crude.divider_ai.map(|s| s.as_style()));

        let scroll_front = crude
            .scroll_front
            .map(|s| s.as_style())
            .unwrap_or(Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD));
        let scroll_front_prefix =
            scroll_front.patch_optionally(crude.scroll_front_prefix.map(|s| s.as_style()));
        let scroll_front_postfix =
            scroll_front_prefix.patch_optionally(crude.scroll_front_postfix.map(|s| s.as_style()));

        let scroll_back = crude
            .scroll_back
            .map(|s| s.as_style())
            .unwrap_or(Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD));
        let scroll_back_prefix =
            scroll_back.patch_optionally(crude.scroll_back_prefix.map(|s| s.as_style()));
        let scroll_back_postfix =
            scroll_back_prefix.patch_optionally(crude.scroll_back_postfix.map(|s| s.as_style()));

        let padding_fill = crude.padding_fill.map(|s| s.as_style()).unwrap_or_default();
        let padding_prefix =
            padding_fill.patch_optionally(crude.padding_prefix.map(|s| s.as_style()));
        let padding_postfix =
            padding_prefix.patch_optionally(crude.padding_postfix.map(|s| s.as_style()));
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
