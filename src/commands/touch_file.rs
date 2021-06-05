use std::path;

use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;
use crate::util::load_child::LoadChild;
use std::fs::OpenOptions;
use super::command_line;
use filetime::FileTime;
use std::time::SystemTime;

pub fn _touch_file(
    context: &mut AppContext,
    file: &path::Path
) -> std::io::Result<()> {
    match file.parent() {
        Some(base_path)  => {
            if !base_path.exists() {
                let err = std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Filename already exists");
                return Err(err);
            }
        },
        None => {
            let err = std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Filename already exists");
            return Err(err);
        }
    }
    match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&file) {
            Ok(_) => {},
            Err(_) => {
                let err = std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Filename already exists");
                return Err(err);
            }
        }
     //   .map_err(|_| format!("could not open {}", file.to_string_lossy()))?;

    let file_time = FileTime::from_system_time(SystemTime::now());

    match filetime::set_file_times(file, file_time, file_time) {
        Ok(_) => {},
        Err(_) => {
                let err = std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Filename already exists");
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

pub fn touch_file(context: &mut AppContext) -> JoshutoResult<()> {
    let path: Option<path::PathBuf> = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|s| s.curr_entry_ref())
        .map(|s| s.file_path().to_path_buf());

    if let Some(path) = path {
        _touch_file(context, &path)?;
    }
    LoadChild::load_child(context)?;
    Ok(())
}
