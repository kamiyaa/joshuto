use termion::event::Key;

pub fn str_to_key(s: &str) -> Option<Key> {
    if s.len() == 0 {
        return None;
    }

    let key = match s {
        "backspace" => Some(Key::Backspace),
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

    if let Some(_) = key {
        return key;
    }

    if s.starts_with("ctrl+") {
        let ch = s.chars().skip("ctrl+".len()).next();
        let key = match ch {
            Some(ch) => Some(Key::Ctrl(ch)),
            None => None,
        };
        return key;
    } else if s.starts_with("alt+") {
        let ch = s.chars().skip("alt+".len()).next();
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
    return None;
}
