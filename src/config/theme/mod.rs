pub mod style;
pub mod style_raw;
pub mod tab;
pub mod tab_raw;
pub mod theme_raw;

use std::collections::HashMap;

use lscolors::LsColors;
use ratatui::style::Color;

use crate::constants::config::THEME_CONFIG;
use crate::error::AppResult;
use crate::traits::config::TomlConfigFile;
use crate::types::config_type::ConfigType;

use style::AppStyle;
use tab::TabTheme;
use theme_raw::AppThemeRaw;

use self::style_raw::AppStyleRaw;

#[derive(Clone, Debug)]
pub struct AppTheme {
    pub tabs: TabTheme,
    pub regular: AppStyle,
    pub selection: AppStyle,
    pub visual_mode_selection: AppStyle,
    pub directory: AppStyle,
    pub executable: AppStyle,
    pub link: AppStyle,
    pub link_invalid: AppStyle,
    pub socket: AppStyle,
    pub ext: HashMap<String, AppStyle>,
    pub lscolors: Option<LsColors>,
    pub preview_background: Color,

    pub msg_info    : AppStyle,
    pub msg_error   : AppStyle,
    pub msg_success : AppStyle,
    pub username    : AppStyle,
    pub prompt      : AppStyle,
    pub indicator   : AppStyle,
}

impl AppTheme {
    pub fn default_res() -> AppResult<Self> {
        let raw: AppThemeRaw = toml::from_str(THEME_CONFIG)?;
        Ok(Self::from(raw))
    }
}

impl TomlConfigFile for AppTheme {
    type Raw = AppThemeRaw;

    fn get_type() -> ConfigType {
        ConfigType::Theme
    }
}

impl std::default::Default for AppTheme {
    fn default() -> Self {
        // This should not fail.
        // If it fails then there is a (syntax) error in the default config file
        Self::default_res().unwrap()
    }
}

impl From<AppThemeRaw> for AppTheme {
    fn from(raw: AppThemeRaw) -> Self {
        let tabs = raw.tabs;
        let selection = raw.selection.to_style_theme();
        let visual_mode_selection = raw.visual_mode_selection.to_style_theme();
        let executable = raw.executable.to_style_theme();
        let regular = raw.regular.to_style_theme();
        let directory = raw.directory.to_style_theme();
        let link = raw.link.to_style_theme();
        let link_invalid = raw.link_invalid.to_style_theme();
        let socket = raw.socket.to_style_theme();
        let ext: HashMap<String, AppStyle> = raw
            .ext
            .iter()
            .map(|(k, v)| {
                let style = v.to_style_theme();
                (k.clone(), style)
            })
            .collect();
        let lscolors = if raw.lscolors_enabled {
            let lscolors = LsColors::from_env();
            let default = Some(LsColors::default());
            lscolors.or(default)
        } else {
            None
        };
        let preview_background = AppStyleRaw::str_to_color(&raw.preview_background);

        let msg_info    = raw.msg_info.to_style_theme();
        let msg_success = raw.msg_success.to_style_theme();
        let msg_error   = raw.msg_error.to_style_theme();
        let username    = raw.username.to_style_theme();
        let prompt      = raw.prompt.to_style_theme();
        let indicator   = raw.indicator.to_style_theme();

        Self {
            selection,
            visual_mode_selection,
            executable,
            regular,
            directory,
            link,
            link_invalid,
            socket,
            ext,
            tabs: TabTheme::from(tabs),
            lscolors,
            preview_background,

            msg_info,
            msg_error,
            msg_success,
            username,
            prompt,
            indicator,
        }
    }
}
