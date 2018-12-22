#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate xdg;

// use std::collections::BTreeMap;
use std::env;
use std::fs;

mod joshuto;

const PROGRAM_NAME : &str = "joshuto";
const CONFIG_FILE : &str = "joshuto.toml";

fn read_config() -> Option<joshuto::config::JoshutoRawConfig>
{
    let dirs = xdg::BaseDirectories::with_profile(PROGRAM_NAME, "").unwrap();

    let config_path = dirs.find_config_file(CONFIG_FILE)?;
    println!("config_path: {:?}", config_path);
    match fs::read_to_string(&config_path) {
        Ok(config_contents) => {
            match toml::from_str(&config_contents) {
                Ok(config) => {
                    println!("rawconfig:\n{:?}", config);
                    Some(config)
                },
                Err(e) => {
                    eprintln!("{}", e);
                    None
                },
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            None
        },
    }
}

fn get_config() -> joshuto::config::JoshutoConfig
{
    match read_config() {
        Some(config) => {
            config.flatten()
        }
        None => {
            joshuto::config::JoshutoConfig::new()
        }
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    println!("args: {:?}", args);

    let config = get_config();

    println!("config:\n{:#?}", config);

    joshuto::run(config);
}
