use std::collections::HashMap;

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

        for (keycomb, keycommand) in &map {
            let keybind = Keybind::from_str(&keycommand);
            for ch in keycomb.chars() {
                let ch: i32 = ch as i32;
                if new_map.contains_key(&ch) {
                    match new_map.remove(&ch) {
                        Some(Keybind::CompositeKeybind(..)) => {},
                        Some(_) => {},
                        None => {},
                    }

                }
            }
        }
        new_map
    }
}

#[derive(Debug)]
pub struct JoshutoKeymaps {
    pub keymaps: HashMap<i32, Keybind>,
}
