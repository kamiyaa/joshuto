use std::path::PathBuf;

use crate::fs::JoshutoDirList;
use crate::history::{DirectoryHistory, JoshutoHistory};
use crate::util::sort;

pub struct JoshutoTab {
    pub history: JoshutoHistory,
    pub curr_path: PathBuf,
}

impl JoshutoTab {
    pub fn new(curr_path: PathBuf, sort_option: &sort::SortOption) -> std::io::Result<Self> {
        let mut history = JoshutoHistory::new();
        history.populate_to_root(&curr_path, sort_option)?;

        Ok(Self { curr_path, history })
    }

    pub fn curr_list_ref(&self) -> Option<&JoshutoDirList> {
        self.history.get(self.curr_path.as_path())
    }

    pub fn parent_list_ref(&self) -> Option<&JoshutoDirList> {
        let parent = self.curr_path.parent()?;
        self.history.get(parent)
    }

    pub fn child_list_ref(&self) -> Option<&JoshutoDirList> {
        let curr_list = self.curr_list_ref()?;
        let index = curr_list.index?;
        let path = curr_list.contents[index].file_path();
        self.history.get(path)
    }

    pub fn curr_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        self.history.get_mut(self.curr_path.as_path())
    }

    pub fn parent_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        let parent = self.curr_path.parent()?;
        self.history.get_mut(parent)
    }

    pub fn child_list_mut(&mut self) -> Option<&mut JoshutoDirList> {
        let path = {
            let curr_list = self.curr_list_ref()?;
            let index = curr_list.index?;
            curr_list.contents[index].file_path().clone()
        };

        self.history.get_mut(path.as_path())
    }
}
