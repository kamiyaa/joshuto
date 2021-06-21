use std::io;
use std::path;
use std::thread;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::history::DirectoryHistory;

pub fn load_preview(context: &mut AppContext, p: path::PathBuf) -> io::Result<path::PathBuf> {
    // get preview
    let options = context.config_ref().display_options_ref().clone();
    context
        .tab_context_mut()
        .curr_tab_mut()
        .history_mut()
        .create_or_soft_update(p.as_path(), &options)?;
    Ok(p)
}

pub struct CursorDir {}

impl CursorDir {
    pub fn load(context: &mut AppContext) -> io::Result<()> {
        let mut p: Option<path::PathBuf> = None;
        if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
            if let Some(index) = curr_list.index {
                let entry = &curr_list.contents[index];
                p = Some(entry.file_path().to_path_buf())
            }
        }

        let res = match p {
            Some(p) if p.is_dir() => load_preview(context, p),
            Some(p) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not a directory".to_string(),
            )),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No such file or directory".to_string(),
            )),
        };
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
