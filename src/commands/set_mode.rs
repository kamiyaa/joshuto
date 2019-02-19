extern crate libc;
extern crate ncurses;

use commands::{JoshutoCommand, JoshutoRunnable};
use context::JoshutoContext;
use structs::JoshutoDirEntry;
use textfield::JoshutoTextField;
use ui;
use unix;

#[derive(Clone, Debug)]
pub struct SetMode;

const LIBC_PERMISSION_VALS: [(libc::mode_t, char); 9] = [
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

impl SetMode {
    pub fn new() -> Self {
        SetMode
    }
    pub const fn command() -> &'static str {
        "set_mode"
    }

    pub fn set_mode(&self, entry: &mut JoshutoDirEntry, start_str: String) -> bool {
        use std::os::unix::fs::PermissionsExt;

        const PROMPT: &str = ":set_mode ";

        let (term_rows, term_cols) = ui::getmaxyx();
        let user_input: Option<String>;
        {
            let textfield = JoshutoTextField::new(
                1,
                term_cols,
                (term_rows as usize - 1, 0),
                PROMPT.to_string(),
            );

            user_input = textfield.readline_with_initial(&start_str, "");
        }
        ncurses::doupdate();

        if let Some(s) = user_input {
            let mut mode: u32 = 0;
            for (i, ch) in s.chars().enumerate() {
                if ch == LIBC_PERMISSION_VALS[i].1 {
                    mode |= LIBC_PERMISSION_VALS[i].0;
                }
            }
            unix::set_mode(entry.path.as_path(), mode);
            entry.metadata.permissions.set_mode(mode + (1 << 15));
            return true;
        }
        false
    }
}

impl JoshutoCommand for SetMode {}

impl std::fmt::Display for SetMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for SetMode {
    fn execute(&self, context: &mut JoshutoContext) {
        let mut ok = false;
        {
            use std::os::unix::fs::PermissionsExt;
            let curr_tab = &mut context.tabs[context.curr_tab_index];
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
            let curr_tab = &mut context.tabs[context.curr_tab_index];
            curr_tab.refresh_curr(&context.views.mid_win, context.config_t.scroll_offset);
            curr_tab.refresh_file_status(&context.views.bot_win);
        }
    }
}
