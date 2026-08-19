use serde::Deserialize;

/// A single keybinding entry as written in `keymap.toml`: a key sequence bound to one or more
/// commands.
#[derive(Clone, Debug, Deserialize)]
pub struct CommandKeymapRaw {
    pub keys: Vec<String>,

    #[serde(default)]
    pub commands: Vec<String>,
    #[serde(default)]
    pub command: Option<String>,

    pub description: Option<String>,
}

/// All keybindings for a single UI mode (e.g. the default view, task view, or help view).
#[derive(Clone, Debug, Deserialize)]
pub struct AppModeKeyMappingRaw {
    #[serde(default)]
    pub keymap: Vec<CommandKeymapRaw>,
}

/// TOML-deserializable form of `keymap.toml`, grouping keybindings by UI mode.
#[derive(Clone, Debug, Deserialize)]
pub struct AppKeyMappingRaw {
    pub default_view: AppModeKeyMappingRaw,
    pub task_view: AppModeKeyMappingRaw,
    pub help_view: AppModeKeyMappingRaw,
}
