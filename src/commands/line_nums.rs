use crate::context::AppContext;
use crate::error::JoshutoResult;

use super::reload;

pub fn switch_line_numbering(context: &mut AppContext, policy: u8) -> JoshutoResult<()> {
    context
        .config_mut()
        .display_options_mut()
        .set_line_nums(policy);
    reload::reload_dirlist(context)
}
