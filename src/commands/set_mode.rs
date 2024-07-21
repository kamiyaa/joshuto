use std::fs;

use crate::error::AppResult;
use crate::types::state::AppState;
use crate::ui::views::{DummyListener, TuiTextField};
use crate::ui::AppBackend;
use crate::utils::unix;

use super::cursor_move;

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

pub fn set_mode(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    const PREFIX: &str = "set_mode ";
    let entry = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|x| x.curr_entry_ref());

    let user_input = match entry {
        Some(entry) => {
            let mode = entry.metadata.permissions_ref().mode();
            let mode_arr = unix::mode_to_char_array(mode);
            let mut listener = DummyListener {};

            let mode_str: String = mode_arr[1..].iter().collect();
            app_state.flush_event();
            TuiTextField::default()
                .prompt(":")
                .prefix(PREFIX)
                .suffix(&mode_str)
                .get_input(app_state, backend, &mut listener)
        }
        None => None,
    };

    if let Some(s) = user_input {
        if let Some(stripped) = s.strip_prefix(PREFIX) {
            let mode = str_to_mode(stripped);
            if let Some(curr_list) = app_state
                .state
                .tab_state_mut()
                .curr_tab_mut()
                .curr_list_mut()
            {
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

                    cursor_move::down(app_state, 1)?;
                }
            }
        }
    }
    Ok(())
}
