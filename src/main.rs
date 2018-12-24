#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate xdg;

// use std::collections::BTreeMap;
use std::env;

mod joshuto;

pub const PROGRAM_NAME: &str = "joshuto";
pub const CONFIG_FILE: &str = "joshuto.toml";
pub const KEYMAP_FILE: &str = "keymap.toml";
pub const MIMETYPE_FILE: &str = "mimetype.toml";

fn main()
{
    let args: Vec<String> = env::args().collect();
    for arg in &args {
        if arg.as_str() == "-v" {
            println!("{}", crate_version!());
            return
        }
    }

    let config = joshuto::config::JoshutoConfig::get_config();
    let keymap = joshuto::keymap::JoshutoKeymap::get_config();
    let mimetype = joshuto::mimetype::JoshutoMimetype::get_config();

    joshuto::run(config, keymap, mimetype);
}
