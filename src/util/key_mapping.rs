use termion::event::{Event, Key, MouseButton, MouseEvent};

pub trait ToString {
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
        match *self {
            k => format!("{:?}", k),
        }
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

pub fn str_to_event(s: &str) -> Option<Event> {
    if let Some(k) = str_to_key(s) {
        Some(Event::Key(k))
    } else if let Some(m) = str_to_mouse(s) {
        Some(Event::Mouse(m))
    } else {
        None
    }
}

pub fn str_to_key(s: &str) -> Option<Key> {
    if s.is_empty() {
        return None;
    }

    let key = match s {
        "backspace" => Some(Key::Backspace),
        "backtab" => Some(Key::BackTab),
        "left" => Some(Key::Left),
        "right" => Some(Key::Right),
        "up" => Some(Key::Up),
        "down" => Some(Key::Down),
        "home" => Some(Key::Home),
        "end" => Some(Key::End),
        "page_up" => Some(Key::PageUp),
        "page_down" => Some(Key::PageDown),
        "delete" => Some(Key::Delete),
        "insert" => Some(Key::Insert),
        "escape" => Some(Key::Esc),
        "f1" => Some(Key::F(1)),
        "f2" => Some(Key::F(2)),
        "f3" => Some(Key::F(3)),
        "f4" => Some(Key::F(4)),
        "f5" => Some(Key::F(5)),
        "f6" => Some(Key::F(6)),
        "f7" => Some(Key::F(7)),
        "f8" => Some(Key::F(8)),
        "f9" => Some(Key::F(9)),
        "f10" => Some(Key::F(10)),
        "f11" => Some(Key::F(11)),
        "f12" => Some(Key::F(12)),
        _ => None,
    };

    if key.is_some() {
        return key;
    }

    if s.starts_with("ctrl+") {
        let ch = s.chars().nth("ctrl+".len());
        let key = match ch {
            Some(ch) => Some(Key::Ctrl(ch)),
            None => None,
        };
        return key;
    } else if s.starts_with("alt+") {
        let ch = s.chars().nth("alt+".len());
        let key = match ch {
            Some(ch) => Some(Key::Alt(ch)),
            None => None,
        };
        return key;
    } else if s.len() == 1 {
        let ch = s.chars().next();
        let key = match ch {
            Some(ch) => Some(Key::Char(ch)),
            None => None,
        };
        return key;
    }
    None
}

pub fn str_to_mouse(s: &str) -> Option<MouseEvent> {
    match s {
        "scroll_up" => Some(MouseEvent::Press(MouseButton::WheelUp, 0, 0)),
        "scroll_down" => Some(MouseEvent::Press(MouseButton::WheelDown, 0, 0)),
        s => None,
    }
}
