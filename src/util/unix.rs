pub fn is_executable(mode: u32) -> bool {
    const LIBC_PERMISSION_VALS: [u32; 3] = [
        libc::S_IXUSR as u32,
        libc::S_IXGRP as u32,
        libc::S_IXOTH as u32,
    ];

    LIBC_PERMISSION_VALS.iter().any(|val| mode & *val != 0)
}

pub fn mode_to_string(mode: u32) -> String {
    const LIBC_FILE_VALS: [(u32, char); 7] = [
        (libc::S_IFREG as u32 >> 9, '-'),
        (libc::S_IFDIR as u32 >> 9, 'd'),
        (libc::S_IFLNK as u32 >> 9, 'l'),
        (libc::S_IFSOCK as u32 >> 9, 's'),
        (libc::S_IFBLK as u32 >> 9, 'b'),
        (libc::S_IFCHR as u32 >> 9, 'c'),
        (libc::S_IFIFO as u32 >> 9, 'f'),
    ];

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
        if mode_shifted == (*val).into() {
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
