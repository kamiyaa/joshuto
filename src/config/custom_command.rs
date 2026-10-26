use serde::Deserialize;

/// A user-defined named alias for a command string, configured in `joshuto.toml`.
#[derive(Debug, Deserialize, Clone)]
pub struct CustomCommand {
    pub name: String,
    pub command: String,
}
