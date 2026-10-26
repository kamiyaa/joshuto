use nix::libc::raise;
use nix::sys::signal::Signal;

use crate::error::AppResult;
use crate::ui::AppBackend;

/// Implements `suspend`: releases the terminal, raises `SIGTSTP` to suspend joshuto, and
/// restores the terminal once resumed.
pub fn signal_suspend(backend: &mut AppBackend) -> AppResult {
    backend.terminal_drop();
    unsafe {
        let signal: i32 = Signal::SIGTSTP as i32;
        raise(signal);
    }
    backend.terminal_restore()?;
    Ok(())
}
