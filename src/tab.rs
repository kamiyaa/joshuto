use std::path::{Path, PathBuf};

use crate::fs::JoshutoDirList;
use crate::history::{DirectoryHistory, JoshutoHistory};
use crate::util::sort;

pub struct JoshutoTab {
    history: JoshutoHistory,
    curr_pwd: PathBuf,
}

impl JoshutoTab {
    pub fn new(curr_pwd: PathBuf, sort_option: &sort::SortOption) -> std::io::Result<Self> {
        let mut history = JoshutoHistory::new();
        history.populate_to_root(&curr_pwd, sort_option)?;

        Ok(Self { curr_pwd, history })
    }

    pub fn pwd(&self) -> &Path {
        self.curr_pwd.as_path()
    }

    pub fn pwd_mut(&mut self) -> &mut PathBuf {
        &mut self.curr_pwd
    }

    pub fn set_pwd(&mut self, pwd: &Path) {
        self.curr_pwd = pwd.to_path_buf();
    }

    pub fn history_mut(&mut self) -> &mut JoshutoHistory {
        &mut self.history
    }

    pub fn curr_list_ref(&self) -> Option<&JoshutoDirList> {
        self.history.get(self.pwd())
    }

    pub fn parent_list_ref(&self) -> Option<&JoshutoDirList> {
        let parent = self.pwd().parent()?;
        self.history.get(parent)
    }

    pub fn child_list_ref(&self) -> Option<&JoshutoDirList> {
        let curr_list = self.curr_list_ref()?;
        let index = curr_list.index?;
        let path = curr_list.contents[index].file_path();
        self.history.get(path)
    }

    pub fn curr_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        self.history.get_mut(self.curr_pwd.as_path())
    }

    pub fn parent_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        let parent = self.curr_pwd.parent()?;
        self.history.get_mut(parent)
    }

    pub fn child_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        let path = {
            let curr_list = self.curr_list_ref()?;
            let index = curr_list.index?;
            curr_list.contents[index].file_path().to_path_buf()
        };

        self.history.get_mut(path.as_path())
    }
}
