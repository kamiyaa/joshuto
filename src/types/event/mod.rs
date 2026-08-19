//! The application event loop: terminal input, background IO/preview results, and OS signals,
//! all funneled into a single [`AppEvent`] channel.

mod app_event_listener;
mod input_listener;
mod signal_listener;

pub use self::app_event_listener::*;
