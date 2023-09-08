use std::collections::HashMap;

use serde::Deserialize;

use crate::config::clean::preview::FileEntryPreviewEntry;

#[derive(Debug, Default, Deserialize)]
pub struct FileEntryPreviewRaw {
    pub extension: Option<HashMap<String, FileEntryPreviewEntry>>,
    pub mimetype: Option<HashMap<String, FileEntryPreviewEntry>>,
}
