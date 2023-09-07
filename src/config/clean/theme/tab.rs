use crate::config::raw::theme::tab::TabThemeRaw;

use super::style::AppStyle;

#[derive(Clone, Debug)]
pub struct TabTheme {
    pub inactive: AppStyle,
    pub active: AppStyle,
}

impl From<TabThemeRaw> for TabTheme {
    fn from(crude: TabThemeRaw) -> Self {
        let inactive = crude.inactive.to_style_theme();
        let active = crude.active.to_style_theme();
        Self { inactive, active }
    }
}
