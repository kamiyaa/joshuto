use std::path::PathBuf;

use crate::context::AppContext;
use crate::history::DirectoryHistory;

pub struct LoadChild {}

impl LoadChild {
    pub fn load_child(context: &mut AppContext) -> std::io::Result<()> {
        let mut path: Option<PathBuf> = None;
        if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
            if let Some(index) = curr_list.index {
                let entry = &curr_list.contents[index];
                path = Some(entry.file_path().to_path_buf())
            }
        }

        // get preview
        if let Some(path) = path {
            if path.is_dir() {
                let options = context.display_options_ref().clone();
                context
                    .tab_context_mut()
                    .curr_tab_mut()
                    .history_mut()
                    .create_or_soft_update(path.as_path(), &options)?;
            }
        }
        Ok(())
    }
}
