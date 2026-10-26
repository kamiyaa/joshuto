use std::{path::PathBuf, sync::Mutex};

use crate::{
    config::{
        AppIcons, AppPersistentConfig, AppProgramRegistry, AppTheme, Bookmarks, FileEntryPreview,
    },
    utils::log_buffer::LogBuffer,
    CONFIG_HOME, PROGRAM_NAME,
};

lazy_static::lazy_static! {
    // dynamically builds the config hierarchy
    pub static ref CONFIG_HIERARCHY: Vec<PathBuf> = {
        let mut config_dirs = vec![];

        if let Ok(p) = std::env::var(CONFIG_HOME) {
            let p = PathBuf::from(p);
            if p.is_dir() {
                config_dirs.push(p);
            }
        }

        if let Some(mut config_home) = dirs::config_dir() {
            config_home.push(PROGRAM_NAME);
            config_dirs.push(config_home);
        }

        if let Ok(p) = std::env::var("HOME") {
            let mut p = PathBuf::from(p);
            p.push(format!(".config/{}", PROGRAM_NAME));
            if p.is_dir() {
                config_dirs.push(p);
            }
        }

        config_dirs
    };
    pub static ref APP_THEME: AppTheme = AppTheme::read_config();
    pub static ref APP_MIMETYPE: AppProgramRegistry = AppProgramRegistry::read_config();
    pub static ref APP_PREVIEW: FileEntryPreview = FileEntryPreview::read_config_or_default();
    pub static ref APP_BOOKMARKS: Mutex<Bookmarks> = Mutex::new(Bookmarks::read_config());
    pub static ref APP_ICONS: AppIcons = AppIcons::read_config();

    pub static ref HOME_DIR: Option<PathBuf> = dirs::home_dir();

    pub static ref USERNAME: String = whoami::username().unwrap_or("No Username".to_string());
    pub static ref HOSTNAME: String = whoami::hostname().unwrap_or("No Hostname".to_string());

    // where `tracing` output goes, so it never draws over the TUI
    pub static ref APP_LOG: LogBuffer = LogBuffer::default();

    pub static ref TIMEZONE_STR: String = {
        format!(" UTC{:+} ", jiff::Zoned::now().offset().seconds() / 3600)
    };
}
