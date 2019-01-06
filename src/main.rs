#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate xdg;

// use std::collections::BTreeMap;
use std::env;

mod joshuto;

const PROGRAM_NAME: &str = "joshuto";
const CONFIG_FILE: &str = "joshuto.toml";
const MIMETYPE_FILE: &str = "mimetype.toml";
const KEYMAP_FILE: &str = "keymap.toml";
const THEME_FILE: &str = "theme.toml";

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
    let mimetype = joshuto::mimetype::JoshutoMimetype::get_config();
    let keymap = joshuto::keymap::JoshutoKeymap::get_config();
//    println!("{:#?}", keymap);
//    println!("{:#?}", keymap.keymaps);

    let theme = joshuto::theme::JoshutoTheme::get_config();
//    println!("{:#?}", theme);

    joshuto::run(config, keymap, mimetype);
}
