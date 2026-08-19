/// Where a new tab should be opened, as requested by the `new_tab` command's arguments.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum NewTabMode {
    /// Use the configured [`TabHomePage`](super::TabHomePage).
    #[default]
    Default,
    /// Open in the current tab's working directory.
    CurrentTabDir,
    /// Open in the directory of the entry under the cursor.
    CursorDir,
    /// Open in the given directory.
    Directory(String),
}

impl NewTabMode {
    /// Parses a `new_tab` command argument (`--current`, `--cursor`, a path, or empty) into a
    /// `NewTabMode`.
    pub fn from_str(arg: &str) -> NewTabMode {
        match arg.trim() {
            "" => NewTabMode::Default,
            "--current" => NewTabMode::CurrentTabDir,
            "--cursor" => NewTabMode::CursorDir,
            dir => NewTabMode::Directory(String::from(dir)),
        }
    }
}
