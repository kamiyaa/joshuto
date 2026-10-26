use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;

use uuid::Uuid;

use crate::tab::JoshutoTab;

/// The set of open tabs, their display order, and which one is active.
#[derive(Default)]
pub struct TabState {
    pub index: usize,
    pub tab_order: Vec<Uuid>,
    pub tabs: HashMap<Uuid, JoshutoTab>,
}

impl TabState {
    /// Creates an empty tab state with no tabs.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    /// Returns the number of open tabs.
    pub fn len(&self) -> usize {
        self.tab_order.len()
    }

    /// Returns the tab with the given id, if open.
    pub fn tab_ref(&self, id: &Uuid) -> Option<&JoshutoTab> {
        self.tabs.get(id)
    }
    /// Returns all open tabs in display order.
    pub fn tab_refs_in_order(&self) -> Vec<&JoshutoTab> {
        let mut tab_refs: Vec<&JoshutoTab> = vec![];
        for tab_id in self.tab_order.iter() {
            if let Some(tab_ref) = self.tab_ref(tab_id) {
                tab_refs.push(tab_ref);
            }
        }
        tab_refs
    }
    /// Returns a mutable reference to the tab with the given id, if open.
    pub fn tab_mut(&mut self, id: &Uuid) -> Option<&mut JoshutoTab> {
        self.tabs.get_mut(id)
    }

    /// Returns the id of the currently active tab.
    pub fn curr_tab_id(&self) -> Uuid {
        self.tab_order[self.index]
    }
    /// Returns the currently active tab.
    pub fn curr_tab_ref(&self) -> &JoshutoTab {
        let id = &self.tab_order[self.index];
        self.tabs.get(id).unwrap()
    }
    /// Returns a mutable reference to the currently active tab.
    pub fn curr_tab_mut(&mut self) -> &mut JoshutoTab {
        let id = &self.tab_order[self.index];
        self.tabs.get_mut(id).unwrap()
    }

    /// Inserts `tab` under `id`, placing it last or just after the active tab.
    pub fn insert_tab(&mut self, id: Uuid, tab: JoshutoTab, last: bool) {
        self.tabs.insert(id, tab);
        if last {
            self.tab_order.push(id);
        } else {
            self.tab_order.insert(self.index + 1, id);
        }
    }
    /// Removes and returns the tab with the given id, if open.
    pub fn remove_tab(&mut self, id: &Uuid) -> Option<JoshutoTab> {
        let tab = self.tabs.remove(id);
        for i in 0..self.tab_order.len() {
            if self.tab_order[i] == *id {
                self.tab_order.remove(i);
                break;
            }
        }
        tab
    }

    /// Returns an iterator over all tabs, in arbitrary (hash map) order.
    pub fn iter<'a>(&'a self) -> Iter<'a, Uuid, JoshutoTab> {
        self.tabs.iter()
    }
    /// Returns a mutable iterator over all tabs, in arbitrary (hash map) order.
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, Uuid, JoshutoTab> {
        self.tabs.iter_mut()
    }
}
