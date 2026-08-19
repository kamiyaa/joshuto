use std::fmt::Debug;
use std::{process::Output, time};

/// Status of a file preview: still being generated, failed, or ready to display.
#[derive(Clone)]
pub enum PreviewFileState {
    Loading,
    Error(String),
    Success(FilePreview),
}

impl Debug for PreviewFileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Loading => f.debug_tuple("Loading").finish(),
            Self::Error(message) => f.debug_tuple("Error").field(message).finish(),
            Self::Success(_) => f.debug_tuple("Success").finish(),
        }
    }
}

/// The rendered output of a successfully-generated file preview.
#[derive(Clone, Debug)]
pub struct FilePreview {
    pub status: std::process::ExitStatus,
    pub output: String,
    pub index: usize,
    pub _modified: time::SystemTime,
}

impl std::convert::From<Output> for FilePreview {
    fn from(output: Output) -> Self {
        let s = String::from_utf8_lossy(&output.stdout).to_string();
        let s2 = s.replace('\t', "        ");
        let status = output.status;
        let modified = time::SystemTime::now();
        Self {
            status,
            output: s2,
            _modified: modified,
            index: 0,
        }
    }
}
