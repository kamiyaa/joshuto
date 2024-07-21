use std::path;

use crate::fs::FileType;

#[allow(clippy::unnecessary_cast)]
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

pub fn is_executable(mode: u32) -> bool {
    #[allow(clippy::unnecessary_cast)]
    const LIBC_PERMISSION_VALS: [u32; 3] = [
        libc::S_IXUSR as u32,
        libc::S_IXGRP as u32,
        libc::S_IXOTH as u32,
    ];

    LIBC_PERMISSION_VALS.iter().any(|val| mode & *val != 0)
}

pub fn mode_to_char_array(mode: u32) -> [char; 10] {
    let mut mode_arr = ['-'; 10];

    let file_type = FileType::from(mode);
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

    for (i, (val, ch)) in LIBC_PERMISSION_VALS.iter().enumerate() {
        if mode & *val != 0 {
            mode_arr[i + 1] = *ch;
        }
    }
    mode_arr
}

pub fn expand_shell_string_cow(s: &str) -> std::borrow::Cow<'_, str> {
    let dir = dirs_next::home_dir();
    let os_str = dir.map(|s| s.as_os_str().to_owned());
    let app_state_func = || {
        let cow_str = os_str.as_ref().map(|s| s.to_string_lossy());
        cow_str
    };
    shellexpand::tilde_with_context(s, app_state_func)
}

pub fn expand_shell_string(s: &str) -> path::PathBuf {
    let dir = dirs_next::home_dir();
    let os_str = dir.map(|s| s.as_os_str().to_owned());
    let app_state_func = || {
        let cow_str = os_str.as_ref().map(|s| s.to_string_lossy());
        cow_str
    };
    let tilde_cow = shellexpand::tilde_with_context(s, app_state_func);
    let tilde_path = path::PathBuf::from(tilde_cow.as_ref());
    tilde_path
}

pub fn uid_to_string(uid: u32) -> Option<String> {
    use nix::unistd::{Uid, User};

    match User::from_uid(Uid::from(uid)) {
        Ok(Some(user)) => Some(user.name),
        _ => None,
    }
}

pub fn gid_to_string(gid: u32) -> Option<String> {
    use nix::unistd::{Gid, Group};

    match Group::from_gid(Gid::from(gid)) {
        Ok(Some(group)) => Some(group.name),
        _ => None,
    }
}
