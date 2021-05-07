use std::io;

use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};

pub fn quit(context: &mut AppContext) -> JoshutoResult<()> {
    let worker_context = context.worker_context_ref();
    if worker_context.is_busy() || !worker_context.is_empty() {
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
