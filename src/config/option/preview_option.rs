use std::path;

const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}

#[derive(Clone, Debug)]
pub struct PreviewOption {
    pub max_preview_size: u64,
    pub preview_images: bool,
    pub preview_script: Option<path::PathBuf>,
}

impl std::default::Default for PreviewOption {
    fn default() -> Self {
        Self {
            max_preview_size: default_max_preview_size(),
            preview_images: false,
            preview_script: None,
        }
    }
}
