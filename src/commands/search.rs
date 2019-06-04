use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::commands::{cursor_move, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::tab::JoshutoTab;
use crate::window::JoshutoView;

lazy_static! {
    static ref SEARCH_PATTERN: Mutex<Option<String>> = Mutex::new(None);
}

#[derive(Clone, Debug)]
pub struct Search {
    pattern: String,
}

impl Search {
    pub fn new(pattern: &str) -> Self {
        Search {
            pattern: pattern.to_lowercase(),
        }
    }
    pub const fn command() -> &'static str {
        "search"
    }
    pub fn search(curr_tab: &JoshutoTab, pattern: &str) -> Option<usize> {
        let curr_list = &curr_tab.curr_list;
        match curr_list.index {
            Some(index) => {
                let offset = index + 1;
                let contents_len = curr_list.contents.len();
                for i in 0..contents_len {
                    let file_name_lower = curr_list.contents[(offset + i) % contents_len]
                        .file_name
                        .to_lowercase();
                    if file_name_lower.contains(pattern) {
                        return Some((offset + i) % contents_len);
                    }
                }
                None
            }
            None => None,
        }
    }
    pub fn search_rev(curr_tab: &JoshutoTab, pattern: &str) -> Option<usize> {
        let curr_list = &curr_tab.curr_list;
        match curr_list.index {
            Some(offset) => {
                let contents_len = curr_list.contents.len();
                for i in (0..contents_len).rev() {
                    let file_name_lower = curr_list.contents[(offset + i) % contents_len]
                        .file_name
                        .to_lowercase();
                    if file_name_lower.contains(pattern) {
                        return Some((offset + i) % contents_len);
                    }
                }
                None
            }
            None => None,
        }
    }
}

impl JoshutoCommand for Search {}

impl std::fmt::Display for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.pattern)
    }
}

impl JoshutoRunnable for Search {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        let index = Self::search(&context.tabs[context.curr_tab_index], &self.pattern);
        if let Some(index) = index {
            cursor_move::cursor_move(index, context, view);
        }
        let mut data = SEARCH_PATTERN.lock().unwrap();
        match data.as_ref() {
            Some(s) => {
                if *s != self.pattern {
                    *data = Some(self.pattern.clone());
                }
            }
            None => *data = Some(self.pattern.clone()),
        }
        ncurses::doupdate();
        Ok(())
    }
}

fn search_with_func(
    context: &mut JoshutoContext,
    view: &JoshutoView,
    search_func: fn(&JoshutoTab, &str) -> Option<usize>,
) {
    let data = SEARCH_PATTERN.lock().unwrap();
    if let Some(s) = (*data).as_ref() {
        let index = search_func(&context.tabs[context.curr_tab_index], s);
        if let Some(index) = index {
            cursor_move::cursor_move(index, context, view);
        }
        ncurses::doupdate();
    }
}

#[derive(Clone, Debug)]
pub struct SearchNext;

impl SearchNext {
    pub fn new() -> Self {
        SearchNext
    }
    pub const fn command() -> &'static str {
        "search_next"
    }
}

impl JoshutoCommand for SearchNext {}

impl std::fmt::Display for SearchNext {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for SearchNext {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        search_with_func(context, view, Search::search);
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SearchPrev;

impl SearchPrev {
    pub fn new() -> Self {
        SearchPrev
    }
    pub const fn command() -> &'static str {
        "search_prev"
    }
}

impl JoshutoCommand for SearchPrev {}

impl std::fmt::Display for SearchPrev {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for SearchPrev {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        search_with_func(context, view, Search::search_rev);
        Ok(())
    }
}
