use ratatui::termion::event::{Event, Key, MouseEvent};

/// Converts a value into its config-file string representation (e.g. `ctrl+c`, `arrow_left`),
/// distinct from `std::string::ToString` which most termion types don't implement usefully.
pub trait ToString {
    /// Returns the config-file string representation of this value.
    fn to_string(&self) -> String;
}

impl ToString for Key {
    fn to_string(&self) -> String {
        match *self {
            Key::Char(c) => format!("{}", c),
            Key::Ctrl(c) => format!("ctrl+{}", c),
            Key::Left => "arrow_left".to_string(),
            Key::Right => "arrow_right".to_string(),
            Key::Up => "arrow_up".to_string(),
            Key::Down => "arrow_down".to_string(),
            Key::Backspace => "backspace".to_string(),
            Key::Home => "home".to_string(),
            Key::End => "end".to_string(),
            Key::PageUp => "page_up".to_string(),
            Key::PageDown => "page_down".to_string(),
            Key::BackTab => "backtab".to_string(),
            Key::Insert => "insert".to_string(),
            Key::Delete => "delete".to_string(),
            Key::Esc => "escape".to_string(),
            Key::F(i) => format!("f{}", i),
            k => format!("{:?}", k),
        }
    }
}

impl ToString for MouseEvent {
    fn to_string(&self) -> String {
        let k = *self;
        format!("{:?}", k)
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        match self {
            Event::Key(key) => key.to_string(),
            Event::Mouse(mouse) => mouse.to_string(),
            Event::Unsupported(v) => format!("{:?}", v),
        }
    }
}
