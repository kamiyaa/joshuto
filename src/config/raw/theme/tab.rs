use serde::Deserialize;

use super::style::AppStyleOptionsRaw;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct TabThemeRaw {
    #[serde(default)]
    pub styles: TabThemeColorRaw,
    #[serde(default)]
    pub chars: TabThemeCharsRaw,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct TabThemeColorRaw {
    pub active_prefix: Option<AppStyleOptionsRaw>,
    pub active_postfix: Option<AppStyleOptionsRaw>,
    pub active: Option<AppStyleOptionsRaw>,
    pub inactive_prefix: Option<AppStyleOptionsRaw>,
    pub inactive_postfix: Option<AppStyleOptionsRaw>,
    pub inactive: Option<AppStyleOptionsRaw>,
    pub divider_ii: Option<AppStyleOptionsRaw>,
    pub divider_ia: Option<AppStyleOptionsRaw>,
    pub divider_ai: Option<AppStyleOptionsRaw>,
    pub scroll_front_prefix: Option<AppStyleOptionsRaw>,
    pub scroll_front_postfix: Option<AppStyleOptionsRaw>,
    pub scroll_front: Option<AppStyleOptionsRaw>,
    pub scroll_back_prefix: Option<AppStyleOptionsRaw>,
    pub scroll_back_postfix: Option<AppStyleOptionsRaw>,
    pub scroll_back: Option<AppStyleOptionsRaw>,
    pub padding_prefix: Option<AppStyleOptionsRaw>,
    pub padding_postfix: Option<AppStyleOptionsRaw>,
    pub padding_fill: Option<AppStyleOptionsRaw>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct TabThemeCharsRaw {
    pub active_prefix: Option<String>,
    pub active_postfix: Option<String>,
    pub inactive_prefix: Option<String>,
    pub inactive_postfix: Option<String>,
    pub divider: Option<String>,
    pub scroll_front_prefix: Option<String>,
    pub scroll_front_postfix: Option<String>,
    pub scroll_front_prestring: Option<String>,
    pub scroll_front_poststring: Option<String>,
    pub scroll_back_prefix: Option<String>,
    pub scroll_back_postfix: Option<String>,
    pub scroll_back_prestring: Option<String>,
    pub scroll_back_poststring: Option<String>,
    pub padding_prefix: Option<char>,
    pub padding_postfix: Option<char>,
    pub padding_fill: Option<char>,
}
