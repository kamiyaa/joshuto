extern crate libc;
extern crate toml;
extern crate tree_magic;
extern crate ncurses;

use std::fs;
use std::path;
use std::collections::HashMap;

use joshuto::structs;
use joshuto::ui;

pub const BITMASK  : u32 = 0o170000;
pub const S_IFSOCK : u32 = 0o140000;   /* socket */
pub const S_IFLNK  : u32 = 0o120000;   /* symbolic link */
pub const S_IFREG  : u32 = 0o100000;   /* regular file */
pub const S_IFBLK  : u32 = 0o060000;   /* block device */
pub const S_IFDIR  : u32 = 0o040000;   /* directory */
pub const S_IFCHR  : u32 = 0o020000;   /* character device */
pub const S_IFIFO  : u32 = 0o010000;   /* FIFO */

pub fn is_reg(mode : u32) -> bool
{
    mode & BITMASK == S_IFREG
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

pub fn get_mime_type(path: &path::Path) -> String
{
    tree_magic::from_filepath(path)
}

pub fn exec_with(program : String, args : Vec<String>)
{
    use std::process::Command;

    let mut child = Command::new(program);
    child.args(args);

    match child.spawn() {
        Ok(mut ch) => {
            match ch.wait() {
                Ok(exit_code) => println!("program exited with code: {}", exit_code),
                Err(e) => eprintln!("{}", e),
            }
        },
        Err(e) => {
            eprintln!("{:?}", e);
        },
    }
}

pub fn is_executable(mode : u32) -> bool
{
    const LIBC_PERMISSION_VALS : [ u32 ; 3] = [
            libc::S_IXUSR,
            libc::S_IXGRP,
            libc::S_IXOTH,
        ];

    for val in LIBC_PERMISSION_VALS.iter() {
        if mode & val != 0 {
            return true;
        }
    }
    return false;
}

pub fn stringify_mode(mode : u32) -> String
{
    let mut mode_str : String = String::with_capacity(10);

    const LIBC_FILE_VALS : [(u32, char) ; 7] = [
        (S_IFSOCK, 's'),
        (S_IFLNK, 'l'),
        (S_IFREG, '-'),
        (S_IFBLK, 'b'),
        (S_IFDIR, 'd'),
        (S_IFCHR, 'c'),
        (S_IFIFO, 'f'),
    ];

    for val in LIBC_FILE_VALS.iter() {
        if mode & val.0 != 0 {
            mode_str.push(val.1);
            break;
        }
    }

    const LIBC_PERMISSION_VALS : [(u32, char) ; 9] = [
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

    for val in LIBC_PERMISSION_VALS.iter() {
        if mode & val.0 != 0 {
            mode_str.push(val.1);
        } else {
            mode_str.push('-');
        }
    }
    mode_str
}

pub fn open_file(mime_map: &HashMap<String, Vec<Vec<String>>>,
        win: &structs::JoshutoWindow, path: &path::Path) {
    use std::os::unix::fs::PermissionsExt;

    if let Ok(metadata) = fs::metadata(path) {
        let permissions : fs::Permissions = metadata.permissions();
        let mode = permissions.mode();
        if is_reg(mode) {
            let mime_type: String = get_mime_type(path);

            if let Some(mime_args) = mime_map.get(mime_type.as_str()) {
                let mime_args_len = mime_args.len();
                if mime_args_len > 0 {
                    let program_name = mime_args[0][0].clone();

                    let mut args_list : Vec<String> = Vec::with_capacity(mime_args_len);
                    for i in 1..mime_args[0].len() {
                        args_list.push(mime_args[0][i].clone());
                    }
                    args_list.push(format!("{:?}", path));

                    ncurses::savetty();
                    ncurses::endwin();
                    exec_with(program_name, args_list);
                    ncurses::resetty();
                    ncurses::refresh();
                }
            } else {
                ui::wprint_err(win, format!("Don't know how to open: {}", mime_type).as_str());
            }
        } else {
            ui::wprint_err(win, format!("Don't know how to open: {}", get_unix_filetype(mode)).as_str());
        }
    } else {
        ui::wprint_err(win, "Failed to read metadata, unable to determine filetype");
    }
    ncurses::doupdate();
}
