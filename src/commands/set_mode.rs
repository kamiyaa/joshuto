use std::fs;

use crate::context::AppContext;
use crate::error::AppResult;
use crate::ui::views::{DummyListener, TuiTextField};
use crate::ui::AppBackend;
use crate::util::unix;

use super::cursor_move;

#[derive(Clone, Debug)]
pub struct SetMode;

#[allow(clippy::unnecessary_cast)]
#[cfg(unix)]
const LIBC_PERMISSION_VALS: [(u32, char); 9] = [
    (libc::S_IRUSR as u32, 'r'),
    (libc::S_IWUSR as u32, 'w'),
    (libc::S_IXUSR as u32, 'x'),
    (libc::S_IRGRP as u32, 'r'),
    (libc::S_IWGRP as u32, 'w'),
    (libc::S_IXGRP as u32, 'x'),
    (libc::S_IROTH as u32, 'r'),
    (libc::S_IWOTH as u32, 'w'),
    (libc::S_IXOTH as u32, 'x'),
];

pub fn str_to_mode(s: &str) -> u32 {
    let mut mode: u32 = 0;
    for (i, ch) in s.chars().enumerate().take(LIBC_PERMISSION_VALS.len()) {
        if ch == LIBC_PERMISSION_VALS[i].1 {
            let (val, _) = LIBC_PERMISSION_VALS[i];
            mode |= val;
        }
    }
    mode
}

pub fn set_mode(context: &mut AppContext, backend: &mut AppBackend) -> AppResult {
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
            let mut listener = DummyListener {};

            context.flush_event();
            TuiTextField::default()
                .prompt(":")
                .prefix(PREFIX)
                .suffix(&mode_string.as_str()[1..])
                .get_input(backend, context, &mut listener)
        }
        None => None,
    };

    if let Some(s) = user_input {
        if let Some(stripped) = s.strip_prefix(PREFIX) {
            let mode = str_to_mode(stripped);
            if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
                if curr_list.selected_count() > 0 {
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
