mod commands;
mod config;
mod context;
mod error;
mod history;
mod preview;
mod run;
mod sort;
mod structs;
mod tab;
mod textfield;
mod ui;
mod unix;
mod window;

use lazy_static::lazy_static;
use std::path::PathBuf;
use structopt::StructOpt;

use config::{
    ConfigStructure, JoshutoConfig, JoshutoKeymap, JoshutoMimetype, JoshutoPreview, JoshutoTheme,
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
    #[structopt(short = "d", long = "debug")]
    debug: bool,
}

fn main() {
    let args = Args::from_args();

    let config = JoshutoConfig::get_config();
    let keymap = JoshutoKeymap::get_config();

    if args.debug {
        eprintln!("config: {:#?}", config);
        eprintln!("theme config: {:#?}", *THEME_T);
        eprintln!("mimetype config: {:#?}", *MIMETYPE_T);
    }

    run(config, keymap);
}
