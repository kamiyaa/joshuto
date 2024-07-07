use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CommandKeymapRaw {
    pub keys: Vec<String>,

    #[serde(default)]
    pub commands: Vec<String>,
    #[serde(default)]
    pub command: Option<String>,

    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppModeKeyMappingRaw {
    #[serde(default)]
    pub keymap: Vec<CommandKeymapRaw>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppKeyMappingRaw {
    pub default_view: AppModeKeyMappingRaw,
    pub task_view: AppModeKeyMappingRaw,
    pub help_view: AppModeKeyMappingRaw,
}
