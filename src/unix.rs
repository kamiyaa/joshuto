use std::path::{Path, PathBuf};
use std::process;

use crate::config::mimetype;

/*
pub const fn is_reg(mode: u32) -> bool
{
    mode >> 9 & S_IFREG >> 9 == mode >> 9
}

pub fn get_unix_filetype(mode : u32) -> &'static str
{
    match mode & BITMASK {
        S_IFBLK => "inode/blockdevice",
        S_IFCHR => "inode/chardevice",
        S_IFDIR => "inode/directory",
        S_IFIFO => "inode/fifo",
        S_IFLNK => "inode/symlink",
        S_IFSOCK => "inode/socket",
        S_IFREG => "inode/regular",
        _ => "unknown",
    }
}
*/

pub fn is_executable(mode: u32) -> bool {
    const LIBC_PERMISSION_VALS: [libc::mode_t; 3] = [libc::S_IXUSR, libc::S_IXGRP, libc::S_IXOTH];

    for val in LIBC_PERMISSION_VALS.iter() {
        let val: u32 = (*val).into();
        if mode & val != 0 {
            return true;
        }
    }
    false
}

pub fn stringify_mode(mode: u32) -> String {
    let mut mode_str: String = String::with_capacity(10);

    const LIBC_FILE_VALS: [(libc::mode_t, char); 7] = [
        (libc::S_IFREG, '-'),
        (libc::S_IFDIR, 'd'),
        (libc::S_IFLNK, 'l'),
        (libc::S_IFSOCK, 's'),
        (libc::S_IFBLK, 'b'),
        (libc::S_IFCHR, 'c'),
        (libc::S_IFIFO, 'f'),
    ];

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

    let mode_shifted = mode >> 9;

    for (val, ch) in LIBC_FILE_VALS.iter() {
        let val: u32 = (*val >> 9).into();
        if mode_shifted & val == mode_shifted {
            mode_str.push(*ch);
            break;
        }
    }

    for (val, ch) in LIBC_PERMISSION_VALS.iter() {
        let val: u32 = (*val).into();
        if mode & val != 0 {
            mode_str.push(*ch);
        } else {
            mode_str.push('-');
        }
    }
    mode_str
}

pub fn set_mode(path: &Path, mode: u32) {
    let os_path = path.as_os_str();
    if let Some(s) = os_path.to_str() {
        let svec: Vec<i8> = s.bytes().map(|ch| ch as i8).collect();
        unsafe {
            libc::chmod(svec.as_ptr(), mode.into());
        }
    }
}

pub fn open_with_entry(paths: &[PathBuf], entry: &mimetype::JoshutoMimetypeEntry) {
    let program = entry.program.clone();

    let mut command = process::Command::new(program);
    if let Some(true) = entry.silent {
        command.stdout(process::Stdio::null());
        command.stderr(process::Stdio::null());
    }
    if let Some(args) = entry.args.as_ref() {
        for arg in args {
            command.arg(arg.clone());
        }
    }
    for path in paths {
        command.arg(path.as_os_str());
    }

    match command.spawn() {
        Ok(mut handle) => {
            if let Some(true) = entry.fork {
            } else {
                match handle.wait() {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

pub fn open_with_args(paths: &[PathBuf], args: &[String]) {
    let program = args[0].clone();

    let mut command = process::Command::new(program);
    for arg in &args[1..] {
        command.arg(arg.clone());
    }
    for path in paths {
        command.arg(path.as_os_str());
    }

    match command.spawn() {
        Ok(mut handle) => match handle.wait() {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }
}
