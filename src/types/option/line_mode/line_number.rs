use serde::{Deserialize, Serialize};

/// How line numbers are displayed next to file entries: not at all, relative to the cursor, or
/// as absolute row numbers.
#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
pub enum LineNumberStyle {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "absolute")]
    Absolute,
}
