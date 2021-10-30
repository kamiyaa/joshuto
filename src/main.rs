mod commands;
mod config;
mod context;
mod error;
mod event;
mod fs;
mod history;
mod io;
mod key_command;
mod preview;
mod run;
mod tab;
mod ui;
mod util;

use lazy_static::lazy_static;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

use crate::config::{
    AppConfig, AppKeyMapping, AppMimetypeRegistry, AppTheme, JoshutoPreview, TomlConfigFile,
};
use crate::context::{AppContext, QuitType};
use crate::error::JoshutoError;
use crate::run::run;

const PROGRAM_NAME: &str = "joshuto";
const CONFIG_HOME: &str = "JOSHUTO_CONFIG_HOME";
const CONFIG_FILE: &str = "joshuto.toml";
const MIMETYPE_FILE: &str = "mimetype.toml";
const KEYMAP_FILE: &str = "keymap.toml";
const THEME_FILE: &str = "theme.toml";
const PREVIEW_FILE: &str = "preview.toml";

lazy_static! {
    // dynamically builds the config hierarchy
    static ref CONFIG_HIERARCHY: Vec<PathBuf> = {
        let mut config_dirs = vec![];

        if let Ok(p) = std::env::var(CONFIG_HOME) {
            let p = PathBuf::from(p);
            if p.is_dir() {
                config_dirs.push(p);
            }
        }

        if let Ok(dirs) = xdg::BaseDirectories::with_prefix(PROGRAM_NAME) {
            config_dirs.push(dirs.get_config_home());
        }

        if let Ok(p) = std::env::var("HOME") {
            let mut p = PathBuf::from(p);
            p.push(".config/joshuto");
            if p.is_dir() {
                config_dirs.push(p);
            }
        }

        // adds the default config files to the config hierarchy if running through cargo
        if cfg!(debug_assertions) {
            config_dirs.push(PathBuf::from("./config"));
        }
        config_dirs
    };
    static ref THEME_T: AppTheme = AppTheme::get_config(THEME_FILE);
    static ref MIMETYPE_T: AppMimetypeRegistry = AppMimetypeRegistry::get_config(MIMETYPE_FILE);
    static ref PREVIEW_T: JoshutoPreview = JoshutoPreview::get_config(PREVIEW_FILE);

    static ref HOME_DIR: Option<PathBuf> = dirs_next::home_dir();
    static ref USERNAME: String = whoami::username();
    static ref HOSTNAME: String = whoami::hostname();
}

#[derive(Clone, Debug, StructOpt)]
pub struct Args {
    #[structopt(long = "path", parse(from_os_str))]
    path: Option<PathBuf>,
    #[structopt(short = "v", long = "version")]
    version: bool,
    #[structopt(long = "last-dir", parse(from_os_str))]
    last_dir: Option<PathBuf>,
}

fn run_joshuto(args: Args) -> Result<(), JoshutoError> {
    if args.version {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}-{}", PROGRAM_NAME, version);
        return Ok(());
    }
    if let Some(p) = args.path.as_ref() {
        if let Err(e) = std::env::set_current_dir(p.as_path()) {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    let config = AppConfig::get_config(CONFIG_FILE);
    let keymap = AppKeyMapping::get_config(KEYMAP_FILE);

    let mut context = AppContext::new(config);
    {
        let mut backend: ui::TuiBackend = ui::TuiBackend::new()?;
        run(&mut backend, &mut context, keymap)?;
    }

    match context.quit {
        QuitType::ToCurrentDirectory => {
            if let Some(p) = args.last_dir {
                let curr_path = std::env::current_dir()?;
                let mut file = File::create(p)?;
                file.write_all(
                    curr_path
                        .into_os_string()
                        .as_os_str()
                        .to_string_lossy()
                        .as_bytes(),
                )?;
                file.write_all("\n".as_bytes())?;
            }
        }
        QuitType::Force => {}
        _ => {}
    }

    Ok(())
}

fn main() {
    let args = Args::from_args();

    if let Err(e) = run_joshuto(args) {
        eprintln!("{}", e.to_string());
        process::exit(1);
    }
}
