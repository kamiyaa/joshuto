extern crate ncurses;

use std;
use std::fmt;
use std::path;

use joshuto;
use joshuto::input;
use joshuto::ui;
use joshuto::window;

use joshuto::command;

#[derive(Clone, Debug)]
pub struct NewDirectory;

impl NewDirectory {
    pub fn new() -> Self { NewDirectory }
    pub fn command() -> &'static str { "mkdir" }
}

impl command::JoshutoCommand for NewDirectory {}

impl std::fmt::Display for NewDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl command::Runnable for NewDirectory {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let mut term_rows: i32 = 0;
        let mut term_cols: i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

        let win = window::JoshutoPanel::new(1, term_cols, (term_rows as usize - 1, 0));
        ncurses::keypad(win.win, true);

        const PROMPT: &str = ":mkdir ";
        ncurses::waddstr(win.win, PROMPT);

        win.move_to_top();
        ncurses::doupdate();

        if let Some(user_input) = input::get_str(&win, (0, PROMPT.len() as i32)) {
            let path = path::PathBuf::from(user_input);

            match std::fs::create_dir_all(&path) {
                Ok(_) => {
                    context.reload_dirlists();

                    let curr_tab = &context.tabs[context.tab_index];
                    ui::redraw_view(&context.views.left_win, curr_tab.parent_list.as_ref());
                    ui::redraw_view(&context.views.mid_win, curr_tab.curr_list.as_ref());
                    ui::redraw_view(&context.views.right_win, curr_tab.preview_list.as_ref());

                    ui::redraw_status(&context.views, curr_tab.curr_list.as_ref(),
                            &curr_tab.curr_path,
                            &context.username, &context.hostname);
                },
                Err(e) => {
                    ui::wprint_err(&context.views.bot_win, e.to_string().as_str());
                },
            }
        }

        win.destroy();
        ncurses::update_panels();
        ncurses::doupdate();
    }
}
