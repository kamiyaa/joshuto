#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum NewTabMode {
    #[default]
    Default,
    CurrentTabDir,
    CursorDir,
    Directory(String),
}

impl NewTabMode {
    pub fn from_str(arg: &str) -> NewTabMode {
        match arg.trim() {
            "" => NewTabMode::Default,
            "--current" => NewTabMode::CurrentTabDir,
            "--cursor" => NewTabMode::CursorDir,
            dir => NewTabMode::Directory(String::from(dir)),
        }
    }
}
