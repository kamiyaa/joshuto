use crate::context::AppContext;
use crate::error::JoshutoResult;

use super::reload;

pub fn escape(context: &mut AppContext) -> JoshutoResult {
    if let Some(curr_dir_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        if curr_dir_list.get_visual_mode_anchor_index().is_some() {
            curr_dir_list.visual_mode_cancel();
        } else {
            // reload to clear current filter
            reload::reload_dirlist(context)?;
        }
    };
    Ok(())
}
