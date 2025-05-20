use nix::fcntl::AT_FDCWD;
use nix::sys::stat::{fchmodat, FchmodatFlags, Mode};

use crate::error::{AppError, AppErrorKind, AppResult};
use crate::types::state::AppState;
use crate::ui::views::{DummyListener, TuiTextField};
use crate::ui::AppBackend;
use crate::utils::unix::{self, LIBC_PERMISSION_VALS};

use super::cursor_move;

pub fn str_to_mode(s: &str) -> Mode {
    let mut mode = Mode::empty();
    for (i, ch) in s.chars().enumerate().take(LIBC_PERMISSION_VALS.len()) {
        if ch == LIBC_PERMISSION_VALS[i].1 {
            let (val, _) = LIBC_PERMISSION_VALS[i];
            mode = mode.union(val);
        }
    }
    mode
}

pub fn set_mode(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    const PREFIX: &str = "set_mode ";
    let entry = app_state
        .state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|x| x.curr_entry_ref());

    let user_input = match entry {
        Some(entry) => {
            let mode_arr = unix::mode_to_char_array(entry.metadata.mode, entry.metadata.file_type);
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
                        fchmodat(
                            AT_FDCWD,
                            entry.file_path(),
                            mode,
                            FchmodatFlags::NoFollowSymlink,
                        )
                        .map_err(|err| {
                            let error_msg = format!("Failed to set file permissions: {err}");
                            AppError::new(AppErrorKind::Io, error_msg)
                        })?;
                        entry.metadata.mode = mode;
                    }
                } else if let Some(entry) = curr_list.curr_entry_mut() {
                    fchmodat(
                        AT_FDCWD,
                        entry.file_path(),
                        mode,
                        FchmodatFlags::NoFollowSymlink,
                    )
                    .map_err(|err| {
                        let error_msg = format!("Failed to set file permissions: {err}");
                        AppError::new(AppErrorKind::Io, error_msg)
                    })?;
                    entry.metadata.mode = mode;
                    cursor_move::down(app_state, 1)?;
                }
            }
        }
    }
    Ok(())
}
