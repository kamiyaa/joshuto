#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate xdg;

mod joshuto;

use std::env;
use std::path::PathBuf;

use joshuto::config::{JoshutoConfig, JoshutoKeymap};

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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args {
        if arg.as_str() == "-v" {
            println!("{}", crate_version!());
            return;
        }
    }

    let config = JoshutoConfig::get_config();
    let keymap = JoshutoKeymap::get_config();

    joshuto::run(config, keymap);
}
