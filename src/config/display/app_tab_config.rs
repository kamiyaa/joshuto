use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::config::{
    display::app_dirlist_option::AppDirListDisplayOptions, FileSortConfig, LineMode,
};

const fn default_home_page() -> TabHomePage {
    TabHomePage::Home
}

/// Config-level defaults applied to new tabs.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppTabOption {
    #[serde(default = "default_home_page")]
    pub home_page: TabHomePage,
}

impl std::default::Default for AppTabOption {
    fn default() -> Self {
        Self {
            home_page: default_home_page(),
        }
    }
}

/// Where a newly-opened tab starts, when not overridden by [`NewTabMode`](super::NewTabMode).
#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub enum TabHomePage {
    #[default]
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "inherit")]
    Inherit,
    #[serde(rename = "root")]
    Root,
}

/// Display options valid per JoshutoTab
#[derive(Clone, Debug, Default)]
pub struct TabDisplayOption {
    pub dirlist_options: HashMap<PathBuf, AppDirListDisplayOptions>,
    pub sort_options: FileSortConfig,
    pub linemode: LineMode,
}

impl TabDisplayOption {
    /// Returns the per-directory display options for `path`, if set.
    pub fn dirlist_options_ref(&self, path: &Path) -> Option<&AppDirListDisplayOptions> {
        self.dirlist_options.get(path)
    }

    /// Returns a mutable reference to `path`'s display options, inserting the default if unset.
    pub fn dirlist_options_mut(&mut self, path: &PathBuf) -> &mut AppDirListDisplayOptions {
        if !self.dirlist_options.contains_key(path) {
            self.dirlist_options
                .insert(path.to_owned(), Default::default());
        }
        self.dirlist_options.get_mut(path).unwrap()
    }
}
