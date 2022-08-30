use crate::context::AppContext;
use crate::error::JoshutoResult;

pub fn toggle_visual_mode(context: &mut AppContext) -> JoshutoResult {
    if let Some(curr_dir_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        curr_dir_list.toggle_visual_mode()
    };
    Ok(())
}
