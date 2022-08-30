use crate::context::AppContext;
use crate::error::JoshutoResult;

pub fn escape(context: &mut AppContext) -> JoshutoResult {
    if let Some(curr_dir_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        curr_dir_list.visual_mode_cancel();
    };
    Ok(())
}
