use std::path;

use nix::sys::stat::Mode;

use crate::{fs::FileType, HOME_DIR};

/// The 9 owner/group/other read/write/execute mode bits, paired with their `rwx` display char,
/// in the order they appear in a `-rwxrwxrwx` permission string.
pub const UNIX_PERMISSION_VALS: [(Mode, char); 9] = [
    (Mode::S_IRUSR, 'r'),
    (Mode::S_IWUSR, 'w'),
    (Mode::S_IXUSR, 'x'),
    (Mode::S_IRGRP, 'r'),
    (Mode::S_IWGRP, 'w'),
    (Mode::S_IXGRP, 'x'),
    (Mode::S_IROTH, 'r'),
    (Mode::S_IWOTH, 'w'),
    (Mode::S_IXOTH, 'x'),
];

const UNIX_EXECUTE_VALS: [Mode; 3] = [Mode::S_IXUSR, Mode::S_IXGRP, Mode::S_IXOTH];

/// Returns `true` if any of owner/group/other execute bits are set.
pub fn is_executable(mode: Mode) -> bool {
    UNIX_EXECUTE_VALS.iter().any(|val| mode.intersects(*val))
}

/// Formats `mode` and `file_type` as a `-rwxrwxrwx`-style 10-character permission array.
pub fn mode_to_char_array(mode: Mode, file_type: FileType) -> [char; 10] {
    let mut mode_arr = ['-'; 10];

    let ch = match file_type {
        FileType::File => '-',
        FileType::Directory => 'd',
        FileType::Block => 'b',
        FileType::Character => 'c',
        FileType::Link => 'l',
        FileType::Pipe => 'f',
        FileType::Socket => 's',
    };
    mode_arr[0] = ch;

    for (i, (val, ch)) in UNIX_PERMISSION_VALS.iter().enumerate() {
        if mode.intersects(*val) {
            mode_arr[i + 1] = *ch;
        }
    }
    mode_arr
}

/// Expands a leading `~` in `s` to the user's home directory, borrowing when no expansion is
/// needed.
pub fn expand_shell_string_cow(s: &str) -> std::borrow::Cow<'_, str> {
    let os_str = HOME_DIR.clone().map(|s| s.as_os_str().to_owned());
    let app_state_func = || {
        let cow_str = os_str.as_ref().map(|s| s.to_string_lossy());
        cow_str
    };
    shellexpand::tilde_with_context(s, app_state_func)
}

/// Expands a leading `~` in `s` to the user's home directory, returning an owned path.
pub fn expand_shell_string(s: &str) -> path::PathBuf {
    let os_str = HOME_DIR.clone().map(|s| s.as_os_str().to_owned());
    let app_state_func = || {
        let cow_str = os_str.as_ref().map(|s| s.to_string_lossy());
        cow_str
    };
    let tilde_cow = shellexpand::tilde_with_context(s, app_state_func);
    let tilde_path = path::PathBuf::from(tilde_cow.as_ref());
    tilde_path
}

/// Resolves a Unix user id to its user name, if the user exists.
pub fn uid_to_string(uid: u32) -> Option<String> {
    use nix::unistd::{Uid, User};

    match User::from_uid(Uid::from(uid)) {
        Ok(Some(user)) => Some(user.name),
        _ => None,
    }
}

/// Resolves a Unix group id to its group name, if the group exists.
pub fn gid_to_string(gid: u32) -> Option<String> {
    use nix::unistd::{Gid, Group};

    match Group::from_gid(Gid::from(gid)) {
        Ok(Some(group)) => Some(group.name),
        _ => None,
    }
}
