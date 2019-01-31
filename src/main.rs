#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate xdg;

mod joshuto;

use std::path;

const PROGRAM_NAME: &str = "joshuto";
const CONFIG_FILE: &str = "joshuto.toml";
const MIMETYPE_FILE: &str = "mimetype.toml";
const KEYMAP_FILE: &str = "keymap.toml";
const THEME_FILE: &str = "theme.toml";

lazy_static! {
    static ref CONFIG_HIERARCHY: Vec<path::PathBuf> = {
        let mut temp = vec![];
        match xdg::BaseDirectories::with_prefix(::PROGRAM_NAME) {
            Ok(dirs) => temp.push(dirs.get_config_home()),
            Err(e) => eprintln!("{}", e),
        };
        if cfg!(debug_assertions) {
            temp.push(path::PathBuf::from("./config"));
        }
        temp
    };
}

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    for arg in &args {
        if arg.as_str() == "-v" {
            println!("{}", crate_version!());
            return
        }
    }

    let config = joshuto::config::JoshutoConfig::get_config();
//    println!("{:#?}", config);

    let keymap = joshuto::config::JoshutoKeymap::get_config();
//    println!("{:#?}", keymap);

    joshuto::run(config, keymap);
}
