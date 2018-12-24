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
    println!("args: {:?}", args);

    let config = joshuto::config::JoshutoConfig::get_config();
//    println!("config:\n{:#?}", config);

    let keymap = joshuto::keymap::JoshutoKeymap::get_config();
//    println!("keymap:\n{:#?}", keymap);

    let mimetype = joshuto::mimetype::JoshutoMimetype::get_config();
//    println!("mimetype:\n{:#?}", mimetype);

    joshuto::run(config, keymap, mimetype);
}
