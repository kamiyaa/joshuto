use std::collections::hash_map::IterMut;
use std::collections::HashMap;

use uuid::Uuid;

use crate::config::clean::app::tab::{TabBarDisplayMode, TabBarDisplayOption};
use crate::tab::JoshutoTab;

#[derive(Default)]
pub struct TabContext {
    pub index: usize,
    pub tab_order: Vec<Uuid>,
    pub display: TabBarDisplayOption,
    tabs: HashMap<Uuid, JoshutoTab>,
}

impl TabContext {
    pub fn new(display: TabBarDisplayOption) -> Self {
        Self {
            display,
            ..Default::default()
        }
    }
    pub fn len(&self) -> usize {
        self.tab_order.len()
    }

    pub fn tab_ref(&self, id: &Uuid) -> Option<&JoshutoTab> {
        self.tabs.get(id)
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

    pub fn tab_title_width(&self) -> usize {
        self.tabs
            .values()
            .map(|tab| {
                let title_len = tab.tab_title().len();
                (title_len > self.display.max_len)
                    .then(|| self.display.max_len)
                    .unwrap_or(title_len)
            })
            .sum()
    }

    pub fn tab_area_width(&self) -> usize {
        let width_without_divider = match self.display.mode {
            TabBarDisplayMode::Number => (1..=self.len()).map(|n| n.to_string().len() + 2).sum(), // each number has a horizontal padding(1 char width)
            TabBarDisplayMode::Directory => self.tab_title_width(),
            TabBarDisplayMode::All => {
                // [number][: ](width = 2)[title]
                self.tab_title_width()
                    + (1..=self.len())
                        .map(|n| n.to_string().len() + 2)
                        .sum::<usize>()
            }
        };

        width_without_divider + 3 * (self.len() - 1)
    }
}
