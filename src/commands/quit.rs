use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};

pub fn quit(context: &mut JoshutoContext) -> JoshutoResult<()> {
    if context.worker_is_busy() {
        Err(JoshutoError::new(
            JoshutoErrorKind::IoOther,
            String::from("operations running in background, use force_quit to quit"),
        ))
    } else {
        context.exit = true;
        Ok(())
    }
}

pub fn force_quit(context: &mut JoshutoContext) -> JoshutoResult<()> {
    context.exit = true;
    Ok(())
}
