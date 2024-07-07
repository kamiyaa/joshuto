use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileEntryPreviewEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
}
