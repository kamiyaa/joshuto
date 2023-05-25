use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::AppBackend;
use crate::HOME_DIR;

use std::fs::{File, OpenOptions};

use super::change_directory::change_directory;

pub fn fzf_add_bookmark(
    _context: &mut AppContext,
    _backend: &mut AppBackend,
    bookmark_name: &String,
) -> JoshutoResult {
    if let Some(home_dir) = HOME_DIR.as_ref() {
        let mut markfile_path = PathBuf::from(home_dir);
        markfile_path.push(".fzf-marks");

        if !markfile_path.exists() {
            File::create(&markfile_path)?;
        }

        let bookmark_path = std::env::current_dir()?;

        // let new_mark_str = format!(
        //     "{} : {}",
        //     bookmark_name,
        //     bookmark_path.as_os_str().to_str().unwrap()
        // );

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(markfile_path)
            .unwrap();

        if let Err(e) = writeln!(
            file,
            "{} : {}",
            bookmark_name,
            bookmark_path.as_os_str().to_str().unwrap()
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    } else {
        return Err(JoshutoError::new(
            JoshutoErrorKind::EnvVarNotPresent,
            format!("{}: Cannot find home directory", "fzf_cd_bookmark"),
        ));
    }

    Ok(())
}

pub fn fzf_cd_bookmark(context: &mut AppContext, backend: &mut AppBackend) -> JoshutoResult {
    if let Some(home_dir) = HOME_DIR.as_ref() {
        let mut markfile_path = PathBuf::from(home_dir);
        markfile_path.push(".fzf-marks");

        if !markfile_path.exists() {
            return Err(JoshutoError::new(
                JoshutoErrorKind::Io(io::ErrorKind::NotFound),
                String::from("There's no fzf bookmark! Create one first by fzf_add_bookmark."),
            ));
        }

        backend.terminal_drop();

        let cat_cmd = Command::new("cat")
            .arg(markfile_path.as_os_str().to_str().unwrap())
            .stdout(Stdio::piped())
            .spawn()?;

        let fzf = Command::new("fzf")
            .stdin(Stdio::from(cat_cmd.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;

        let fzf_output = fzf.wait_with_output();

        match fzf_output {
            Ok(output) if output.status.success() => {
                if let Ok(selected) = std::str::from_utf8(&output.stdout) {
                    if let Some((_, dir)) = selected.rsplit_once(':') {
                        let path: PathBuf = PathBuf::from(dir.trim());
                        change_directory(context, path.as_path())?;
                    }
                }
            }
            _ => {}
        }
        backend.terminal_restore()?;
    } else {
        return Err(JoshutoError::new(
            JoshutoErrorKind::EnvVarNotPresent,
            format!("{}: Cannot find home directory", "fzf_cd_bookmark"),
        ));
    }
    Ok(())
}
