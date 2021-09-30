use crate::context::AppContext;
use crate::error::JoshutoResult;

pub fn parent_directory_helper(context: &mut AppContext) -> std::io::Result<()> {
    if let Some(parent) = context
        .tab_context_ref()
        .curr_tab_ref()
        .cwd()
        .parent()
        .map(|p| p.to_path_buf())
    {
        std::env::set_current_dir(&parent)?;
        context
            .tab_context_mut()
            .curr_tab_mut()
            .set_cwd(parent.as_path());
    }
    Ok(())
}

pub fn parent_directory(context: &mut AppContext) -> JoshutoResult<()> {
    parent_directory_helper(context)?;
    reload::soft_reload(context.tab_context_ref().index, context)?;
    Ok(())
}
