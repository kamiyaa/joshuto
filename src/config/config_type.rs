#[derive(Copy, Clone, Debug)]
pub enum ConfigType {
    App,
    Mimetype,
    Keymap,
    Theme,
    Preview,
    Bookmarks,
    Icons,
    StateContext,
}

impl std::fmt::Display for ConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl clap::ValueEnum for ConfigType {
    fn value_variants<'a>() -> &'a [Self] {
        Self::enumerate()
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.as_str()))
    }
}

impl ConfigType {
    pub const fn enumerate() -> &'static [Self] {
        &[
            Self::App,
            Self::Mimetype,
            Self::Keymap,
            Self::Theme,
            Self::Preview,
            Self::Bookmarks,
            Self::Icons,
            Self::StateContext,
        ]
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::App => "joshuto",
            Self::Mimetype => "mimetype",
            Self::Keymap => "keymap",
            Self::Theme => "theme",
            Self::Preview => "preview",
            Self::Bookmarks => "bookmarks",
            Self::Icons => "icons",
            Self::StateContext => "state_context",
        }
    }

    pub const fn as_filename(&self) -> &'static str {
        match self {
            Self::App => "joshuto.toml",
            Self::Mimetype => "mimetype.toml",
            Self::Keymap => "keymap.toml",
            Self::Theme => "theme.toml",
            Self::Preview => "preview.toml",
            Self::Bookmarks => "bookmarks.toml",
            Self::Icons => "icons.toml",
            Self::StateContext => "state_context.toml"
        }
    }

    pub const fn embedded_config(&self) -> Option<&'static str> {
        use super::clean;
        match self {
            Self::App => Some(clean::app::DEFAULT_CONFIG_FILE_PATH),
            Self::Keymap => Some(clean::keymap::DEFAULT_CONFIG_FILE_PATH),
            Self::Theme => Some(clean::theme::DEFAULT_CONFIG_FILE_PATH),
            Self::Icons => Some(clean::icon::DEFAULT_CONFIG_FILE_PATH),
            Self::Mimetype | Self::Preview | Self::Bookmarks | Self::StateContext => None,
        }
    }
}
