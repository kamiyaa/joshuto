use std::collections::HashMap;

use serde::Deserialize;

use super::preview::FileEntryPreviewEntry;

#[derive(Debug, Default, Deserialize)]
pub struct FileEntryPreviewRaw {
    pub extension: Option<HashMap<String, FileEntryPreviewEntry>>,
    pub mimetype: Option<HashMap<String, FileEntryPreviewEntry>>,
}
