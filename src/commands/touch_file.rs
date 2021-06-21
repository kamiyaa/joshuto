use std::fs::File;
use std::path;
use std::time::SystemTime;

use filetime::FileTime;

use crate::context::AppContext;
use crate::error::JoshutoResult;

fn _update_actime(file: &path::Path) -> std::io::Result<()> {
    let file_time = FileTime::from_system_time(SystemTime::now());
    filetime::set_file_times(file, file_time, file_time)
}

fn _create_file(file: &path::Path) -> std::io::Result<()> {
    File::create(file)?;
    Ok(())
}

pub fn touch_file(context: &mut AppContext, arg: &str) -> JoshutoResult<()> {
    match arg {
        "" => {
            match context
                .tab_context_ref()
                .curr_tab_ref()
                .curr_list_ref()
                .and_then(|s| s.curr_entry_ref())
                .map(|s| s.file_path().to_path_buf())
            {
                Some(selected_file_path) => _update_actime(&selected_file_path)?,
                None => {}
            }
        }
        file_arg => {
            let file = path::PathBuf::from(file_arg);
            if file.exists() {
                _update_actime(file.as_path())?;
            } else {
                _create_file(file.as_path())?;
            }
        }
    }
    let options = context.config_ref().display_options_ref().clone();
    if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        curr_list.reload_contents(&options)?;
    }
    Ok(())
}
