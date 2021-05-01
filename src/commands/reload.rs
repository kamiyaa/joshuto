use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::util::load_child::LoadChild;

pub fn soft_reload(index: usize, context: &mut AppContext) -> std::io::Result<()> {
    let options = context.config_ref().display_options_ref().clone();
    if let Some(curr_tab) = context.tab_context_mut().tab_mut(index) {
        if let Some(curr_list) = curr_tab.curr_list_mut() {
            if curr_list.need_update() {
                curr_list.reload_contents(&options)?;
            }
        }
        if let Some(curr_list) = curr_tab.parent_list_mut() {
            if curr_list.need_update() {
                curr_list.reload_contents(&options)?;
            }
        }
        if let Some(curr_list) = curr_tab.child_list_mut() {
            if curr_list.need_update() {
                curr_list.reload_contents(&options)?;
            }
        }
    }
    Ok(())
}

pub fn reload(context: &mut AppContext, index: usize) -> std::io::Result<()> {
    let options = context.config_ref().display_options_ref().clone();
    if let Some(curr_tab) = context.tab_context_mut().tab_mut(index) {
        if let Some(curr_list) = curr_tab.curr_list_mut() {
            curr_list.reload_contents(&options)?;
        }
        if let Some(curr_list) = curr_tab.parent_list_mut() {
            curr_list.reload_contents(&options)?;
        }
        if let Some(curr_list) = curr_tab.child_list_mut() {
            curr_list.reload_contents(&options)?;
        }
    }
    Ok(())
}

pub fn reload_dirlist(context: &mut AppContext) -> JoshutoResult<()> {
    reload(context, context.tab_context_ref().get_index())?;
    LoadChild::load_child(context)?;
    Ok(())
}
