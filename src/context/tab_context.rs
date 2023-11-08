use std::collections::hash_map::IterMut;
use std::collections::HashMap;

use uuid::Uuid;

use crate::tab::JoshutoTab;

#[derive(Default)]
pub struct TabContext {
    pub index: usize,
    pub tab_order: Vec<Uuid>,
    tabs: HashMap<Uuid, JoshutoTab>,
}

impl TabContext {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn len(&self) -> usize {
        self.tab_order.len()
    }

    pub fn tab_ref(&self, id: &Uuid) -> Option<&JoshutoTab> {
        self.tabs.get(id)
    }

    pub fn tab_refs_in_order(&self) -> Vec<&JoshutoTab> {
        let mut tab_refs: Vec<&JoshutoTab> = vec![];
        for tab_id in self.tab_order.iter() {
            if let Some(tab_ref) = self.tab_ref(tab_id) {
                tab_refs.push(tab_ref);
            }
        }
        tab_refs
    }

    pub fn tab_mut(&mut self, id: &Uuid) -> Option<&mut JoshutoTab> {
        self.tabs.get_mut(id)
    }

    pub fn curr_tab_id(&self) -> Uuid {
        self.tab_order[self.index]
    }
    pub fn curr_tab_ref(&self) -> &JoshutoTab {
        let id = &self.tab_order[self.index];
        self.tabs.get(id).unwrap()
    }
    pub fn curr_tab_mut(&mut self) -> &mut JoshutoTab {
        let id = &self.tab_order[self.index];
        self.tabs.get_mut(id).unwrap()
    }
    pub fn insert_tab(&mut self, id: Uuid, tab: JoshutoTab) {
        self.tabs.insert(id, tab);
        self.tab_order.push(id);
    }
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

    pub fn iter_mut(&mut self) -> IterMut<Uuid, JoshutoTab> {
        self.tabs.iter_mut()
    }
}
