use std::io;

use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};

pub fn quit(context: &mut AppContext) -> JoshutoResult<()> {
    if context.worker_is_busy() {
        Err(JoshutoError::new(
            JoshutoErrorKind::Io(io::ErrorKind::Other),
            String::from("operations running in background, use force_quit to quit"),
        ))
    } else {
        context.exit = true;
        Ok(())
    }
}

pub fn force_quit(context: &mut AppContext) -> JoshutoResult<()> {
    context.exit = true;
    Ok(())
}
