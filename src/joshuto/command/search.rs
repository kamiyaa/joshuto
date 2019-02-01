extern crate ncurses;

use std;

use joshuto::command::CursorMove;
use joshuto::command::JoshutoCommand;
use joshuto::command::JoshutoRunnable;
use joshuto::context::JoshutoContext;
use joshuto::textfield::JoshutoTextField;
use joshuto::ui;

#[derive(Clone, Debug)]
pub struct Search;

impl Search {
    pub fn new() -> Self {
        Search
    }
    pub const fn command() -> &'static str {
        "search"
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

        let mut index: Option<i32> = None;

        if let Some(user_input) = user_input {
            let user_input = user_input.to_lowercase();

            let curr_tab = &context.tabs[context.curr_tab_index];

            if let Some(curr_list) = curr_tab.curr_list.as_ref() {
                let offset = curr_list.index as usize;
                let contents_len = curr_list.contents.len();
                for i in 0..contents_len {
                    let file_name_lower = curr_list.contents[(offset + i) % contents_len]
                        .file_name_as_string
                        .to_lowercase();
                    if file_name_lower.contains(user_input.as_str()) {
                        index = Some(((offset + i) % contents_len) as i32);
                        break;
                    }
                }
            }
        }

        if let Some(index) = index {
            CursorMove::cursor_move(index, context);
        }
        ncurses::doupdate();
    }
}
