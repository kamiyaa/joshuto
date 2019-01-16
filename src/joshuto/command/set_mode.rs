extern crate ncurses;

use std;
use std::fmt;

use joshuto;
use joshuto::command;
use joshuto::input;
use joshuto::structs;
use joshuto::ui;
use joshuto::unix;
use joshuto::window;

#[derive(Clone, Debug)]
pub struct SetMode;

impl SetMode {
    pub fn new() -> Self { SetMode }
    pub fn command() -> &'static str { "set_mode" }

    pub fn set_mode(&self, entry: &mut structs::JoshutoDirEntry, start_str: String)
            -> bool
    {
        use std::os::unix::fs::PermissionsExt;

        let (term_rows, term_cols) = ui::getmaxyx();

        let win = window::JoshutoPanel::new(1, term_cols, (term_rows as usize - 1, 0));
        ncurses::keypad(win.win, true);

        const PROMPT: &str = ":set_mode ";
        ncurses::waddstr(win.win, PROMPT);

        win.move_to_top();
        ncurses::doupdate();

        let user_input = input::get_str_prepend(&win, (0, PROMPT.len() as i32), start_str);

        win.destroy();
        ncurses::update_panels();
        ncurses::doupdate();

        const LIBC_PERMISSION_VALS : [(u32, char) ; 9] = [
                (libc::S_IRUSR, 'r'),
                (libc::S_IWUSR, 'w'),
                (libc::S_IXUSR, 'x'),
                (libc::S_IRGRP, 'r'),
                (libc::S_IWGRP, 'w'),
                (libc::S_IXGRP, 'x'),
                (libc::S_IROTH, 'r'),
                (libc::S_IWOTH, 'w'),
                (libc::S_IXOTH, 'x'),
        ];

        if let Some(s) = user_input {
            let mut mode: libc::mode_t = 0;
            for (i, ch) in s.chars().enumerate() {
                if ch == LIBC_PERMISSION_VALS[i].1 {
                    mode = mode | LIBC_PERMISSION_VALS[i].0;
                }
            }
            unix::set_mode(entry.path.as_path(), mode);
            entry.metadata.permissions.set_mode(mode + (1 << 15));
            return true;
        }
        return false;
    }
}

impl command::JoshutoCommand for SetMode {}

impl std::fmt::Display for SetMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for SetMode {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let mut ok = false;
        {
            use std::os::unix::fs::PermissionsExt;
            let curr_tab = &mut context.tabs[context.tab_index];
            if let Some(s) = curr_tab.curr_list.as_mut() {
                if let Some(file) = s.get_curr_mut() {
                    let mode = file.metadata.permissions.mode();
                    let mut mode_string = unix::stringify_mode(mode);
                    mode_string.remove(0);

                    ok = self.set_mode(file, mode_string);
                }
            }
        }
        if ok {
            let curr_tab = &mut context.tabs[context.tab_index];
            curr_tab.refresh_curr(&context.views.mid_win, &context.theme_t, context.config_t.scroll_offset);
            curr_tab.refresh_file_status(&context.views.bot_win);
        }
    }
}
