// #[macro_use]
// extern crate serde_derive;

extern crate ncurses;
extern crate toml;
extern crate xdg;

// use std::collections::BTreeMap;
use std::env;

mod joshuto;

const PROGRAM_NAME : &str = "joshuto";
const CONFIG_FILE : &str = "joshuto.toml";

fn main()
{
    let args: Vec<String> = env::args().collect();

    println!("args: {:?}", args);

    let config = joshuto::config::JoshutoRawConfig::new();
    let config = config.to_config();

    println!("config:\n{:#?}", config);

    joshuto::run(config);
}
