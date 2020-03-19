mod commands;
mod config;
mod context;
mod error;
mod fs;
mod history;
mod io;
mod run;
mod sort;
mod tab;
mod ui;
mod util;

use lazy_static::lazy_static;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

use config::{
    ConfigStructure, JoshutoCommandMapping, JoshutoConfig, JoshutoMimetype, JoshutoPreview,
    JoshutoTheme,
};
use run::run;

const PROGRAM_NAME: &str = "joshuto";
const CONFIG_FILE: &str = "joshuto.toml";
const MIMETYPE_FILE: &str = "mimetype.toml";
const KEYMAP_FILE: &str = "keymap.toml";
const THEME_FILE: &str = "theme.toml";
const PREVIEW_FILE: &str = "preview.toml";

lazy_static! {
    // dynamically builds the config hierarchy
    static ref CONFIG_HIERARCHY: Vec<PathBuf> = {
        let mut temp = vec![];
        match xdg::BaseDirectories::with_prefix(PROGRAM_NAME) {
            Ok(dirs) => temp.push(dirs.get_config_home()),
            Err(e) => eprintln!("{}", e),
        };
        // adds the default config files to the config hierarchy if running through cargo
        if cfg!(debug_assertions) {
            temp.push(PathBuf::from("./config"));
        }
        temp
    };
    static ref THEME_T: JoshutoTheme = JoshutoTheme::get_config();
    static ref MIMETYPE_T: JoshutoMimetype = JoshutoMimetype::get_config();
    static ref PREVIEW_T: JoshutoPreview = JoshutoPreview::get_config();

    static ref HOME_DIR: Option<PathBuf> = dirs::home_dir();
    static ref USERNAME: String = whoami::username();
    static ref HOSTNAME: String = whoami::hostname();
}

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(long = "path", parse(from_os_str))]
    path: Option<PathBuf>,
    #[structopt(short = "v", long = "version")]
    version: bool,
}

fn main() {
    let args = Args::from_args();

    if args.version {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}", version);
        return;
    }
    if let Some(p) = args.path {
        match std::env::set_current_dir(p.as_path()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }

    let config = JoshutoConfig::get_config();
    let keymap = JoshutoCommandMapping::get_config();

    match run(config, keymap) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
