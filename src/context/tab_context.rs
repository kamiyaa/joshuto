use std::slice::IterMut;

use crate::tab::JoshutoTab;

#[derive(Default)]
pub struct TabContext {
    pub index: usize,
    tabs: Vec<JoshutoTab>,
}

impl TabContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    pub fn tab_ref(&self, i: usize) -> Option<&JoshutoTab> {
        if i >= self.tabs.len() {
            return None;
        }
        Some(&self.tabs[i])
    }
    pub fn tab_mut(&mut self, i: usize) -> Option<&mut JoshutoTab> {
        if i >= self.tabs.len() {
            return None;
        }
        Some(&mut self.tabs[i])
    }

    pub fn curr_tab_ref(&self) -> &JoshutoTab {
        &self.tabs[self.index]
    }
    pub fn curr_tab_mut(&mut self) -> &mut JoshutoTab {
        &mut self.tabs[self.index]
    }
    pub fn push_tab(&mut self, tab: JoshutoTab) {
        self.tabs.push(tab);
        self.index = self.tabs.len() - 1;
    }
    pub fn pop_tab(&mut self, index: usize) -> JoshutoTab {
        self.tabs.remove(index)
    }

    pub fn iter_mut(&mut self) -> IterMut<JoshutoTab> {
        self.tabs.iter_mut()
    }
}
