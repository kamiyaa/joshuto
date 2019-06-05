use std::path::{Path, PathBuf};
use std::process;

use crate::config::mimetype;

pub fn is_executable(mode: u32) -> bool {
    const LIBC_PERMISSION_VALS: [libc::mode_t; 3] = [libc::S_IXUSR, libc::S_IXGRP, libc::S_IXOTH];

    LIBC_PERMISSION_VALS.iter().any(|val| {
        let val: u32 = (*val) as u32;
        mode & val != 0
    })
}

pub fn stringify_mode(mode: u32) -> String {
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
    let mut mode_str: String = String::with_capacity(10);

    let mode_shifted = mode >> 9;

    for (val, ch) in LIBC_FILE_VALS.iter() {
        let val: u32 = (*val >> 9) as u32;
        if mode_shifted & val == mode_shifted {
            mode_str.push(*ch);
            break;
        }
    }

    for (val, ch) in LIBC_PERMISSION_VALS.iter() {
        let val: u32 = (*val) as u32;
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
        let svec: Vec<libc::c_char> = s.bytes().map(|ch| ch as libc::c_char).collect();
        unsafe {
            libc::chmod(svec.as_ptr(), mode as libc::mode_t);
        }
    }
}

pub fn open_with_entry(paths: &[&PathBuf], entry: &mimetype::JoshutoMimetypeEntry) {
    let program = entry.program.clone();

    let mut command = process::Command::new(program);
    if entry.silent {
        command.stdout(process::Stdio::null());
        command.stderr(process::Stdio::null());
    }

    if let Some(args) = entry.args.as_ref() {
        command.args(args.clone());
    }
    command.args(paths.iter().map(|path| path.as_os_str()));

    match command.spawn() {
        Ok(mut handle) => {
            if !entry.fork {
                match handle.wait() {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
        Err(e) => eprintln!("{}", e),
    };
}

pub fn open_with_args(paths: &[&PathBuf], args: &[String]) {
    let program = args[0].clone();

    let mut command = process::Command::new(program);
    command.args(args[1..].iter().cloned());
    command.args(paths.iter().map(|path| path.as_os_str()));

    match command.spawn() {
        Ok(mut handle) => match handle.wait() {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }
}
