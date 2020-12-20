use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::ui::widgets::TuiTextField;
use crate::ui::TuiBackend;
use crate::util::unix;

use super::cursor_move;

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

pub fn set_mode(context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    use std::os::unix::fs::PermissionsExt;

    const PREFIX: &str = "set_mode ";
    let entry = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|x| x.curr_entry_ref());

    let user_input = match entry {
        Some(entry) => {
            let mode = entry.metadata.permissions_ref().mode();
            let mode_string = unix::mode_to_string(mode);
            TuiTextField::default()
                .prompt(":")
                .prefix(PREFIX)
                .suffix(&mode_string.as_str()[1..])
                .get_input(backend, context)
        }
        None => None,
    };

    if let Some(s) = user_input {
        if let Some(stripped) = s.strip_prefix(PREFIX) {
            let s = stripped;
            let mode = str_to_mode(s);

            let entry = context
                .tab_context_mut()
                .curr_tab_mut()
                .curr_list_mut()
                .and_then(|x| x.curr_entry_mut())
                .unwrap();

            unix::set_mode(entry.file_path(), mode);
            entry.metadata.permissions_mut().set_mode(mode);
            cursor_move::down(context, 1)?;
        }
    }
    Ok(())
}
