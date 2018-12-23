
use std::str;
use std::collections::HashMap;


pub enum Keycode {
}

#[derive(Debug)]
pub enum Keybind {
    Quit,

    MoveUp,
    MoveDown,
    MovePageUp,
    MovePageDown,
    MoveHome,
    MoveEnd,

    DeleteFile,
    RenameFile,
    CopyFile,
    OpenFile,
    OpenWith,
    OpenDirectory,
    ToggleHiddenFiles,

    CompositeKeybind(HashMap<i32, Keybind>),
}

impl Keybind {

    pub fn from_str(keybind: &str) -> Keybind
    {
        match keybind {
            "Quit" => Keybind::Quit,
            "MoveUp" => Keybind::MoveUp,
            "MoveDown" => Keybind::MoveDown,
            "MovePageUp" => Keybind::MovePageUp,
            "MovePageDown" => Keybind::MovePageDown,
            "MoveHome" => Keybind::MoveHome,
            "MoveEnd" => Keybind::MoveEnd,
            "DeleteFile" => Keybind::DeleteFile,
            "RenameFile" => Keybind::RenameFile,
            "CopyFile" => Keybind::CopyFile,
            "OpenFile" => Keybind::OpenFile,
            "OpenWith" => Keybind::OpenWith,
            "OpenDirectory" => Keybind::OpenDirectory,
            "ToggleHiddenFiles" => Keybind::ToggleHiddenFiles,
            _ => Keybind::CompositeKeybind(HashMap::new()),
        }

    }
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawKeymaps {
    keymaps: Option<HashMap<String, String>>,
}

impl JoshutoRawKeymaps {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoRawKeymaps {
            keymaps: None,
        }
    }

    pub fn flatten(self) -> JoshutoKeymaps
    {
        let keymaps: HashMap<String, String> = match self.keymaps {
                Some(s) => {
                    s
                }
                None => {
                    HashMap::new()
                },
            };

        JoshutoKeymaps {
            keymaps: JoshutoRawKeymaps::unflatten_hashmap(keymaps)
        }
    }

    fn unflatten_hashmap(map: HashMap<String, String>) -> HashMap<i32, Keybind>
    {
        let mut new_map: HashMap<i32, Keybind> = HashMap::new();

        for (keycommand, keycomb) in &map {
            let keybind = Keybind::from_str(&keycommand);
            let mut chars = keycomb.chars();
            if let Some(ch) = chars.next() {
                JoshutoRawKeymaps::insert_keycommand(&mut new_map, &mut chars,
                    ch as i32, keybind);
            }
        }
        new_map
    }

    fn insert_keycommand(map: &mut HashMap<i32, Keybind>,
            keycomb: &mut str::Chars, curr_char: i32, keybind: Keybind)
    {
        match keycomb.next() {
            Some(s) => {
                print!("{}", curr_char as u8 as char);
                let ch: i32 = s as i32;
                let mut new_map: HashMap<i32, Keybind>;
                match map.remove(&ch) {
                    Some(Keybind::CompositeKeybind(mut m)) => {
                        new_map = m;
                    },
                    Some(_) => {
                        panic!("keybindings ambiguous");
                    },
                    None => {
                        new_map = HashMap::new();
                    }
                }
                JoshutoRawKeymaps::insert_keycommand(&mut new_map, keycomb, ch, keybind);
                map.insert(curr_char, Keybind::CompositeKeybind(new_map));
            }
            None => {
                println!("{} -> {:?}", curr_char as u8 as char, keybind); 
                map.insert(curr_char, keybind);
            }
        }
    }
}

#[derive(Debug)]
pub struct JoshutoKeymaps {
    pub keymaps: HashMap<i32, Keybind>,
}

impl JoshutoKeymaps {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoKeymaps {
            keymaps: HashMap::new(),
        }
    }
}
