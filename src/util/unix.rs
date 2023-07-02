use std::path;

pub fn is_executable(mode: u32) -> bool {
    #[allow(clippy::unnecessary_cast)]
    const LIBC_PERMISSION_VALS: [u32; 3] = [
        libc::S_IXUSR as u32,
        libc::S_IXGRP as u32,
        libc::S_IXOTH as u32,
    ];

    LIBC_PERMISSION_VALS.iter().any(|val| mode & *val != 0)
}

pub fn mode_to_string(mode: u32) -> String {
    #[allow(clippy::unnecessary_cast)]
    const LIBC_FILE_VALS: [(u32, char); 7] = [
        (libc::S_IFREG as u32 >> 9, '-'),
        (libc::S_IFDIR as u32 >> 9, 'd'),
        (libc::S_IFLNK as u32 >> 9, 'l'),
        (libc::S_IFSOCK as u32 >> 9, 's'),
        (libc::S_IFBLK as u32 >> 9, 'b'),
        (libc::S_IFCHR as u32 >> 9, 'c'),
        (libc::S_IFIFO as u32 >> 9, 'f'),
    ];

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

    let mut mode_str: String = String::with_capacity(10);
    let mode_shifted = mode >> 9;

    for (val, ch) in LIBC_FILE_VALS.iter() {
        if mode_shifted == *val {
            mode_str.push(*ch);
            break;
        }
    }

    for (val, ch) in LIBC_PERMISSION_VALS.iter() {
        if mode & *val != 0 {
            mode_str.push(*ch);
        } else {
            mode_str.push('-');
        }
    }
    mode_str
}

pub fn expand_shell_string(s: &str) -> path::PathBuf {
    let dir = dirs_next::home_dir();
    let os_str = dir.map(|s| s.as_os_str().to_owned());
    let context_func = || {
        let cow_str = os_str.as_ref().map(|s| s.to_string_lossy());
        cow_str
    };
    let tilde_cow = shellexpand::tilde_with_context(s, context_func);
    let tilde_path = path::PathBuf::from(tilde_cow.as_ref());
    tilde_path
}
