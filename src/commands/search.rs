use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::commands::{CursorMove, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::textfield::JoshutoTextField;
use crate::tab::JoshutoTab;
use crate::ui;

lazy_static! {
    static ref search_pattern: Mutex<Option<String>> = Mutex::new(None);
}

#[derive(Clone, Debug)]
pub struct Search;

impl Search {
    pub fn new() -> Self {
        Search
    }
    pub const fn command() -> &'static str {
        "search"
    }
    pub fn search(curr_tab: &JoshutoTab, pattern: &str) -> Option<i32> {
        if let Some(curr_list) = curr_tab.curr_list.as_ref() {
            let offset = curr_list.index as usize + 1;
            let contents_len = curr_list.contents.len();
            for i in 0..contents_len {
                let file_name_lower = curr_list.contents[(offset + i) % contents_len]
                    .file_name_as_string
                    .to_lowercase();
                if file_name_lower.contains(pattern) {
                    return Some(((offset + i) % contents_len) as i32);
                }
            }
        }
        return None;
    }
    pub fn search_rev(curr_tab: &JoshutoTab, pattern: &str) -> Option<i32> {
        if let Some(curr_list) = curr_tab.curr_list.as_ref() {
            let offset = curr_list.index as usize;
            let contents_len = curr_list.contents.len();
            for i in (0..contents_len).rev() {
                let file_name_lower = curr_list.contents[(offset + i) % contents_len]
                    .file_name_as_string
                    .to_lowercase();
                if file_name_lower.contains(pattern) {
                    return Some(((offset + i) % contents_len) as i32);
                }
            }
        }
        return None;
    }
}

impl JoshutoCommand for Search {}

impl std::fmt::Display for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for Search {
    fn execute(&self, context: &mut JoshutoContext) {
        const PROMPT: &str = ":search ";
        let (term_rows, term_cols) = ui::getmaxyx();
        let user_input: Option<String>;
        {
            let textfield = JoshutoTextField::new(
                1,
                term_cols,
                (term_rows as usize - 1, 0),
                PROMPT.to_string(),
            );

            user_input = textfield.readline_with_initial("", "");
        }
        ncurses::doupdate();

        if let Some(user_input) = user_input {
            let user_input = user_input.to_lowercase();

            let index = Self::search(&context.tabs[context.curr_tab_index], &user_input);
            if let Some(index) = index {
                CursorMove::cursor_move(index, context);
            }
            let mut data = search_pattern.lock().unwrap();
	        *data = Some(user_input);
            ncurses::doupdate();
        }
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
    fn execute(&self, context: &mut JoshutoContext) {
        let data = search_pattern.lock().unwrap();
        if let Some(s) = (*data).as_ref() {
            let index = Search::search(&context.tabs[context.curr_tab_index], s);
            if let Some(index) = index {
                CursorMove::cursor_move(index, context);
            }
            ncurses::doupdate();
        }
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
    fn execute(&self, context: &mut JoshutoContext) {
        let data = search_pattern.lock().unwrap();
        if let Some(s) = (*data).as_ref() {
            let index = Search::search_rev(&context.tabs[context.curr_tab_index], s);
            if let Some(index) = index {
                CursorMove::cursor_move(index, context);
            }
            ncurses::doupdate();
        }
    }
}
