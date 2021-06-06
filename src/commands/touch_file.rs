use std::path;

use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::util::load_child::LoadChild;
use filetime::FileTime;
use std::fs::File;
use std::fs::OpenOptions;
use std::time::SystemTime;

fn _touch_file(context: &mut AppContext, file: &path::Path) -> std::io::Result<()> {
    match file.parent() {
        Some(base_path) => {
            if !base_path.exists() {
                let err = std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "Filename already exists",
                );
                return Err(err);
            }
        }
        None => {
            let err =
                std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Filename already exists");
            return Err(err);
        }
    }
    match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&file)
    {
        Ok(_) => {}
        Err(_) => {
            let err =
                std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Filename already exists");
            return Err(err);
        }
    }
    //   .map_err(|_| format!("could not open {}", file.to_string_lossy()))?;

    let file_time = FileTime::from_system_time(SystemTime::now());

    match filetime::set_file_times(file, file_time, file_time) {
        Ok(_) => {}
        Err(_) => {
            let err =
                std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Filename already exists");
            return Err(err);
        }
    }
    //   .map_err(|_| String::from("could not update file times"))?;

    let options = context.config_ref().display_options_ref().clone();
    if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        curr_list.reload_contents(&options)?;
    }
    Ok(())
}

fn _update_actime(file: &path::Path) -> std::io::Result<()> {
    let file_time = FileTime::from_system_time(SystemTime::now());
    filetime::set_file_times(file, file_time, file_time)
}

fn _create_file(file: &path::Path) -> std::io::Result<()> {
    File::create(file)?;
    Ok(())
}

pub fn touch_file(context: &mut AppContext, arg: &str) -> JoshutoResult<()> {
    let selected_file_path: Option<path::PathBuf> = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|s| s.curr_entry_ref())
        .map(|s| s.file_path().to_path_buf());

    if let Some(selected_file_path) = selected_file_path {
        match arg {
            "" => _update_actime(&selected_file_path)?,
            file_arg => {
                let mut file = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
                file.push(file_arg);
                if file.exists() {
                    _update_actime(file.as_path())?;
                } else {
                    _create_file(file.as_path())?;
                }
            }
        }
    }
    let options = context.config_ref().display_options_ref().clone();
    if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        curr_list.reload_contents(&options)?;
    }
    LoadChild::load_child(context)?;
    Ok(())
}
