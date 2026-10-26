//! Color and style theme, loaded from `theme.toml`.

use std::collections::HashMap;

use lscolors::LsColors;
use ratatui::style;

use crate::{
    config::{
        theme::{app_tab_theme::AppTabTheme, app_theme_raw::AppThemeRaw, AppStyle},
        AppPersistentConfig, ConfigType,
    },
    constants::config::DEFAULT_THEME_CONFIG,
    error::{AppError, AppErrorCode, AppResult},
    utils::style_helper,
};

/// Resolved color/style theme for the whole UI, loaded from `theme.toml`.
#[derive(Clone, Debug)]
pub struct AppTheme {
    pub tabs: AppTabTheme,
    pub regular: AppStyle,
    pub selection: AppStyle,
    pub visual_mode_selection: AppStyle,
    pub directory: AppStyle,
    pub executable: AppStyle,
    pub link: AppStyle,
    pub link_invalid: AppStyle,
    pub socket: AppStyle,
    pub border: AppStyle,
    pub git_modified: AppStyle,
    pub git_staged: AppStyle,
    pub git_untracked: AppStyle,
    pub git_deleted: AppStyle,
    pub ext: HashMap<String, AppStyle>,
    pub lscolors: Option<LsColors>,
    pub preview_background: style::Color,
}

impl AppTheme {
    /// Parses the built-in default `theme.toml`, returning an error only if it's malformed.
    pub fn try_default() -> AppResult<Self> {
        let value: AppThemeRaw = toml::from_str(DEFAULT_THEME_CONFIG).map_err(|err| {
            let error_msg = "Failed to parse default config";
            tracing::error!(?err, "{error_msg}");
            AppError::new(AppErrorCode::DefaultConfigParse, error_msg.to_string())
        })?;
        Ok(Self::from(value))
    }
}

impl std::default::Default for AppTheme {
    fn default() -> Self {
        // This should not fail.
        // If it fails then there is a (syntax) error in the default config file
        Self::try_default().unwrap()
    }
}

impl From<AppThemeRaw> for AppTheme {
    fn from(raw: AppThemeRaw) -> Self {
        let tabs = raw.tabs;
        let selection = raw.selection.into();
        let visual_mode_selection = raw.visual_mode_selection.into();
        let executable = raw.executable.into();
        let regular = raw.regular.into();
        let directory = raw.directory.into();
        let link = raw.link.into();
        let link_invalid = raw.link_invalid.into();
        let socket = raw.socket.into();
        let border = raw.border.into();
        let git_modified = raw.git_modified.into();
        let git_staged = raw.git_staged.into();
        let git_untracked = raw.git_untracked.into();
        let git_deleted = raw.git_deleted.into();
        let ext: HashMap<String, AppStyle> = raw
            .ext
            .iter()
            .map(|(k, v)| {
                let style = v.clone().into();
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
        let preview_background = style_helper::str_to_color(&raw.preview_background);

        let tabs = AppTabTheme::from(tabs);
        Self {
            selection,
            visual_mode_selection,
            executable,
            regular,
            directory,
            link,
            link_invalid,
            socket,
            border,
            git_modified,
            git_staged,
            git_untracked,
            git_deleted,
            ext,
            tabs,
            lscolors,
            preview_background,
        }
    }
}

impl AppPersistentConfig for AppTheme {
    type Raw = AppThemeRaw;

    fn get_type() -> ConfigType {
        ConfigType::Theme
    }
}

#[cfg(test)]
mod test_border_theme {
    use ratatui::style::Color;

    use super::{AppTheme, AppThemeRaw};

    fn theme_from_str(s: &str) -> AppTheme {
        let raw: AppThemeRaw = toml::from_str(s).unwrap();
        AppTheme::from(raw)
    }

    #[test]
    fn border_style_defaults_to_reset_when_unset() {
        let theme = theme_from_str("");
        assert_eq!(theme.border.fg, Color::Reset);
        assert_eq!(theme.border.bg, Color::Reset);
    }

    #[test]
    fn border_style_reads_configured_color() {
        let theme = theme_from_str("[border]\nfg = \"red\"\nbg = \"black\"\n");
        assert_eq!(theme.border.fg, Color::Red);
        assert_eq!(theme.border.bg, Color::Black);
    }
}
