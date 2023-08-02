use std::str::FromStr;

use serde_derive::Deserialize;

use crate::{
    error::{JoshutoError, JoshutoErrorKind},
    tab::TabHomePage,
};

#[derive(Clone, Debug)]
pub struct TabOption {
    pub _home_page: TabHomePage,
    pub display: TabBarDisplayOption,
}

impl TabOption {
    pub fn new(_home_page: TabHomePage, display_mode: TabBarDisplayMode, max_len: usize) -> Self {
        Self {
            _home_page,
            display: TabBarDisplayOption {
                mode: display_mode,
                max_len,
            },
        }
    }
    pub fn home_page(&self) -> TabHomePage {
        self._home_page
    }
}

impl std::default::Default for TabOption {
    fn default() -> Self {
        Self {
            _home_page: TabHomePage::Home,
            display: TabBarDisplayOption::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TabBarDisplayOption {
    pub mode: TabBarDisplayMode,
    pub max_len: usize,
}

impl Default for TabBarDisplayOption {
    fn default() -> Self {
        Self {
            mode: Default::default(),
            max_len: 16,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Default)]
pub enum TabBarDisplayMode {
    #[serde(rename = "num")]
    Number,
    #[default]
    #[serde(rename = "dir")]
    Directory,
    #[serde(rename = "all")]
    All,
}

impl FromStr for TabBarDisplayMode {
    type Err = JoshutoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "num" => Ok(Self::Number),
            "dir" => Ok(Self::Directory),
            "all" => Ok(Self::All),
            s => Err(JoshutoError::new(
                JoshutoErrorKind::UnrecognizedArgument,
                format!("tab_bar_mode: `{}` unknown argument.", s),
            )),
        }
    }
}
