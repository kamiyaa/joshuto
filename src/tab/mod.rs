//! A single joshuto tab: its current directory, navigation history, and per-tab display options.

mod nav_history;
mod new_tab_mode;

pub use new_tab_mode::*;

use nav_history::NavigationHistory;

use std::collections::HashMap;
use std::path;

use crate::config::TabDisplayOption;
use crate::fs::{AppDirectoryHistory, AppDirectoryList};
use crate::tasks::PreviewDirState;

type HistoryMetadata = HashMap<path::PathBuf, PreviewDirState>;

/// A single tab: its current directory, directory-listing cache, and navigation history.
pub struct JoshutoTab {
    pub cwd: path::PathBuf,
    // history is just a HashMap, so we have this property to store last workdir
    pub previous_dir: Option<path::PathBuf>,
    pub history: AppDirectoryHistory,
    pub history_metadata: HistoryMetadata,
    pub options: TabDisplayOption,
    pub navigation_history: NavigationHistory,
}

impl JoshutoTab {
    /// Creates a new tab rooted at `cwd`, sharing the given directory-listing cache.
    pub fn new(
        cwd: path::PathBuf,
        history: AppDirectoryHistory,
        tab_options: TabDisplayOption,
    ) -> std::io::Result<Self> {
        let navigation_history = NavigationHistory::from(&cwd);
        let new_tab = Self {
            cwd,
            previous_dir: None,
            history,
            history_metadata: HashMap::new(),
            navigation_history,
            options: tab_options,
        };

        Ok(new_tab)
    }

    /// Returns this tab's current working directory.
    pub fn get_cwd(&self) -> &path::Path {
        self.cwd.as_path()
    }
    /// Changes this tab's current directory, recording the previous one and, if
    /// `history_update` is set, pushing it onto the back/forward navigation history.
    pub fn set_cwd(&mut self, cwd: &path::Path, history_update: bool) {
        self.previous_dir = Some(self.cwd.to_path_buf());
        self.cwd = cwd.to_path_buf();

        if history_update {
            self.navigation_history.push(cwd);
        }

        // OSC 7: Escape sequence to set the working directory
        // print!("\x1b]7;file://{}{}\x1b\\", HOSTNAME.as_str(), cwd.display());
    }

    /// Returns the directory this tab was in before its current one, if any.
    pub fn previous_dir(&self) -> Option<&path::Path> {
        self.previous_dir.as_deref()
    }

    /// Returns this tab's back/forward navigation history.

    /// Returns the cached listing for the current directory, if present.
    pub fn curr_list_ref(&self) -> Option<&AppDirectoryList> {
        self.history.get(self.get_cwd())
    }
    /// Returns the cached listing for the parent of the current directory, if present.
    pub fn parent_list_ref(&self) -> Option<&AppDirectoryList> {
        let parent = self.get_cwd().parent()?;
        self.history.get(parent)
    }
    /// Returns the cached listing for the entry under the cursor, if it's a directory and present.
    pub fn child_list_ref(&self) -> Option<&AppDirectoryList> {
        let curr_list = self.curr_list_ref()?;
        let index = curr_list.get_index()?;
        let path = curr_list.contents[index].file_path();
        self.history.get(path)
    }

    /// Returns a mutable reference to the cached listing for the current directory, if present.
    pub fn curr_list_mut(&mut self) -> Option<&mut AppDirectoryList> {
        self.history.get_mut(self.cwd.as_path())
    }
    /// Returns a mutable reference to the cached listing for the parent directory, if present.
    pub fn parent_list_mut(&mut self) -> Option<&mut AppDirectoryList> {
        let parent = self.cwd.parent()?;
        self.history.get_mut(parent)
    }
    /// Returns a mutable reference to the cached listing for the entry under the cursor, if
    /// it's a directory and present.
    #[allow(dead_code)]
    pub fn child_list_mut(&mut self) -> Option<&mut AppDirectoryList> {
        let child_path = {
            let curr_list = self.curr_list_ref()?;
            let index = curr_list.get_index()?;
            curr_list.contents[index].file_path().to_path_buf()
        };

        self.history.get_mut(child_path.as_path())
    }
}
