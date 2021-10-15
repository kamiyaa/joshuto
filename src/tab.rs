use std::path;

use crate::fs::JoshutoDirList;
use crate::history::{DirectoryHistory, JoshutoHistory};
use crate::util::display_option::DisplayOption;

#[derive(Clone, Copy, Debug)]
pub enum TabHomePage {
    Inherit,
    Home,
    Root,
}

pub struct JoshutoTab {
    history: JoshutoHistory,
    _cwd: path::PathBuf,
}

impl JoshutoTab {
    pub fn new(cwd: path::PathBuf, options: &DisplayOption) -> std::io::Result<Self> {
        let mut history = JoshutoHistory::new();
        history.populate_to_root(cwd.as_path(), options)?;

        Ok(Self { _cwd: cwd, history })
    }

    pub fn cwd(&self) -> &path::Path {
        self._cwd.as_path()
    }

    pub fn set_cwd(&mut self, cwd: &path::Path) {
        self._cwd = cwd.to_path_buf();
    }

    pub fn history_ref(&self) -> &JoshutoHistory {
        &self.history
    }

    pub fn history_mut(&mut self) -> &mut JoshutoHistory {
        &mut self.history
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
        let index = curr_list.index?;
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
            let index = curr_list.index?;
            curr_list.contents[index].file_path().to_path_buf()
        };

        self.history.get_mut(child_path.as_path())
    }
}
