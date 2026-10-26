use serde::{Deserialize, Serialize};

/// A configured external program (and args) used to generate a preview for a mimetype.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileEntryPreviewEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
}
