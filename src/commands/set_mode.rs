use crate::commands::{CursorMoveDown, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::widgets::TuiTextField;
use crate::ui::TuiBackend;
use crate::util::unix;

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

    pub fn str_to_mode(s: &str) -> u32 {
        let mut mode: u32 = 0;
        for (i, ch) in s.chars().enumerate() {
            if ch == LIBC_PERMISSION_VALS[i].1 {
                let val: u32 = LIBC_PERMISSION_VALS[i].0 as u32;
                mode |= val;
            }
        }
        mode
    }
}

impl JoshutoCommand for SetMode {}

impl std::fmt::Display for SetMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for SetMode {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        use std::os::unix::fs::PermissionsExt;

        const PREFIX: &'static str = "set_mode ";

        let entry = context.tabs[context.curr_tab_index]
            .curr_list_ref()
            .and_then(|x| x.get_curr_ref());

        let user_input = match entry {
            Some(entry) => {
                let mode = entry.metadata.permissions.mode();
                let mode_string = unix::stringify_mode(mode);
                let mut textfield = TuiTextField::default()
                    .prompt(":")
                    .prefix(PREFIX)
                    .suffix(&mode_string.as_str()[1..]);
                textfield.get_input(backend, context)
            }
            None => None,
        };

        if let Some(s) = user_input {
            if s.starts_with(PREFIX) {
                let s = &s[PREFIX.len()..];
                let mode = Self::str_to_mode(s);

                let entry = context.tabs[context.curr_tab_index]
                    .curr_list_mut()
                    .and_then(|x| x.get_curr_mut())
                    .unwrap();

                unix::set_mode(entry.file_path().as_path(), mode);
                entry.metadata.permissions.set_mode(mode);
                CursorMoveDown::new(1).execute(context, backend)?;
            }
        }

        Ok(())
    }
}
