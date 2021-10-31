use std::fs;

use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::views::TuiTextField;
use crate::ui::TuiBackend;
use crate::util::unix;

use super::cursor_move;

#[derive(Clone, Debug)]
pub struct SetMode;

#[cfg(unix)]
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

pub fn str_to_mode(s: &str) -> libc::mode_t {
    let mut mode: libc::mode_t = 0;
    for (i, ch) in s.chars().enumerate().take(LIBC_PERMISSION_VALS.len()) {
        if ch == LIBC_PERMISSION_VALS[i].1 {
            mode |= LIBC_PERMISSION_VALS[i].0;
        }
    }
    mode
}

pub fn set_mode(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    #[cfg(unix)]
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

            context.flush_event();
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
            let mode = str_to_mode(stripped);
            if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
                if curr_list.any_selected() {
                    for entry in curr_list.iter_selected_mut() {
                        let mut permissions = entry.metadata.permissions_ref().clone();
                        let file_mode = (permissions.mode() >> 12) << 12 | mode;
                        permissions.set_mode(file_mode);

                        fs::set_permissions(entry.file_path(), permissions)?;
                        entry.metadata.permissions_mut().set_mode(file_mode);
                    }
                } else if let Some(entry) = curr_list.curr_entry_mut() {
                    let mut permissions = entry.metadata.permissions_ref().clone();
                    let file_mode = (permissions.mode() >> 12) << 12 | mode;
                    permissions.set_mode(file_mode);

                    fs::set_permissions(entry.file_path(), permissions)?;
                    entry.metadata.permissions_mut().set_mode(file_mode);

                    cursor_move::down(context, 1)?;
                }
            }
        }
    }
    Ok(())
}
