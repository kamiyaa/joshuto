use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::util::load_child::LoadChild;

use super::reload;

pub fn parent_directory_helper(context: &mut JoshutoContext) -> std::io::Result<()> {
    if context.tab_context_mut().curr_tab_mut().pwd_mut().pop() {
        let path = context.tab_context_ref().curr_tab_ref().pwd();
        std::env::set_current_dir(path)?;
    }
    Ok(())
}

pub fn parent_directory(context: &mut JoshutoContext) -> JoshutoResult<()> {
    parent_directory_helper(context)?;
    reload::soft_reload(context.tab_context_ref().get_index(), context)?;
    LoadChild::load_child(context)?;
    Ok(())
}
