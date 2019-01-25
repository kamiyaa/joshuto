extern crate ncurses;

use std;
use std::path;

use joshuto::context::JoshutoContext;
use joshuto::textfield::JoshutoTextField;
use joshuto::ui;


use joshuto::command::ReloadDirList;
use joshuto::command::JoshutoCommand;
use joshuto::command::JoshutoRunnable;

#[derive(Clone, Debug)]
pub struct NewDirectory;

impl NewDirectory {
    pub fn new() -> Self { NewDirectory }
    pub fn command() -> &'static str { "mkdir" }
}

impl JoshutoCommand for NewDirectory {}

impl std::fmt::Display for NewDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for NewDirectory {
    fn execute(&self, context: &mut JoshutoContext)
    {
        let mut term_rows: i32 = 0;
        let mut term_cols: i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

        let textfield = JoshutoTextField::new(1, term_cols, (term_rows as usize - 1, 0), ":mkdir ".to_string());

        if let Some(user_input) = textfield.readline_with_initial("", "") {
            let path = path::PathBuf::from(user_input);

            match std::fs::create_dir_all(&path) {
                Ok(_) => {
                    ReloadDirList::reload(context);
                },
                Err(e) => {
                    ui::wprint_err(&context.views.bot_win, e.to_string().as_str());
                },
            }
        }
        ncurses::doupdate();
    }
}
