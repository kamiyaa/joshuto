use std::collections::HashMap;
use std::path;

use crate::config::clean::app::display::tab::TabDisplayOption;
use crate::config::clean::app::display::DisplayOption;
use crate::context::UiContext;
use crate::fs::JoshutoDirList;
use crate::history::{DirectoryHistory, JoshutoHistory};
use crate::preview::preview_dir::PreviewDirState;
// use crate::HOSTNAME;

type HistoryMetadata = HashMap<path::PathBuf, PreviewDirState>;

pub struct JoshutoTab {
    _cwd: path::PathBuf,
    // history is just a HashMap, so we have this property to store last workdir
    _previous_dir: Option<path::PathBuf>,
    history: JoshutoHistory,
    history_metadata: HistoryMetadata,
    options: TabDisplayOption,
}

impl JoshutoTab {
    pub fn new(
        cwd: path::PathBuf,
        ui_context: &UiContext,
        options: &DisplayOption,
    ) -> std::io::Result<Self> {
        let mut history = JoshutoHistory::new();
        let tab_options = options.default_tab_display_option.clone();

        history.populate_to_root(cwd.as_path(), ui_context, options, &tab_options)?;
        let new_tab = Self {
            _cwd: cwd,
            _previous_dir: None,
            history,
            history_metadata: HashMap::new(),
            options: tab_options,
        };

        Ok(new_tab)
    }

    pub fn option_ref(&self) -> &TabDisplayOption {
        &self.options
    }

    pub fn option_mut(&mut self) -> &mut TabDisplayOption {
        &mut self.options
    }

    pub fn cwd(&self) -> &path::Path {
        self._cwd.as_path()
    }
    pub fn set_cwd(&mut self, cwd: &path::Path) {
        self._previous_dir = Some(self._cwd.to_path_buf());
        self._cwd = cwd.to_path_buf();

        // OSC 7: Escape sequence to set the working directory
        // print!("\x1b]7;file://{}{}\x1b\\", HOSTNAME.as_str(), cwd.display());
    }

    pub fn previous_dir(&self) -> Option<&path::Path> {
        // This converts PathBuf to Path
        match &self._previous_dir {
            Some(path) => Some(path),
            None => None,
        }
    }

    pub fn history_ref(&self) -> &JoshutoHistory {
        &self.history
    }
    pub fn history_mut(&mut self) -> &mut JoshutoHistory {
        &mut self.history
    }

    pub fn history_metadata_ref(&self) -> &HistoryMetadata {
        &self.history_metadata
    }
    pub fn history_metadata_mut(&mut self) -> &mut HistoryMetadata {
        &mut self.history_metadata
    }

    pub fn curr_list_ref(&self) -> Option<&JoshutoDirList> {
        self.history.get(self.cwd())
    }
    pub fn parent_list_ref(&self) -> Option<&JoshutoDirList> {
        let parent = self.cwd().parent()?;
        self.history.get(parent)
    }
    pub fn child_list_ref(&self) -> Option<&JoshutoDirList> {
        let curr_list = self.curr_list_ref()?;
        let index = curr_list.get_index()?;
        let path = curr_list.contents[index].file_path();
        self.history.get(path)
    }

    pub fn curr_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        self.history.get_mut(self._cwd.as_path())
    }
    pub fn parent_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        let parent = self._cwd.parent()?;
        self.history.get_mut(parent)
    }
    #[allow(dead_code)]
    pub fn child_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        let child_path = {
            let curr_list = self.curr_list_ref()?;
            let index = curr_list.get_index()?;
            curr_list.contents[index].file_path().to_path_buf()
        };

        self.history.get_mut(child_path.as_path())
    }
}
