use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::util::load_child::LoadChild;

use super::reload;

pub fn parent_directory_helper(context: &mut AppContext) -> std::io::Result<()> {
    if context.tab_context_mut().curr_tab_mut().pwd_mut().pop() {
        let path = context.tab_context_ref().curr_tab_ref().pwd();
        std::env::set_current_dir(path)?;
    }
    Ok(())
}

pub fn parent_directory(context: &mut AppContext) -> JoshutoResult<()> {
    parent_directory_helper(context)?;
    reload::soft_reload(context.tab_context_ref().get_index(), context)?;
    LoadChild::load_child(context)?;
    Ok(())
}
