#[macro_use]
extern crate serde_derive;

extern crate ncurses;
extern crate toml;
extern crate xdg;

use std::env;
use std::fs;

mod joshuto;

const PROGRAM_NAME : &str = "joshuto";
const CONFIG_FILE : &str = "joshuto.toml";

/*
pub struct joshuto_win {
    win : ncurses::WINDOW,
    offset : usize,
    row_size : i32,
    col_size : i32,
}
*/

#[derive(Debug, Deserialize)]
pub struct JoshutoConfig {
    show_hidden: Option<bool>,
    color_scheme: Option<String>,
    sort_method: Option<String>,
//    keymaps: Option<JoshutoKeymaps>,
    mimetypes: Option<toml::value::Table>,
}

#[derive(Debug, Deserialize)]
pub struct JoshutoKeymaps {
    up : i32,
}

fn generate_default_config() -> JoshutoConfig
{
    JoshutoConfig {
        show_hidden: None,
        color_scheme: None,
        sort_method: None,
//        keymaps: None,
        mimetypes: None,
    }
}

fn read_config() -> Option<JoshutoConfig>
{
    let dirs = xdg::BaseDirectories::with_profile(PROGRAM_NAME, "").unwrap();
    match dirs.find_config_file(CONFIG_FILE) {
        Some(config_path) => {
            println!("config_path: {:?}", config_path);
            let config_contents = fs::read_to_string(&config_path).unwrap();

            match toml::from_str(&config_contents) {
                Ok(config) => {
                    Some(config)
                },
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            }
        },
        None => {
            None
        }
    }
}

fn get_config() -> JoshutoConfig
{
    match read_config() {
        Some(config) => {
            config
        }
        None => {
            generate_default_config()
        }
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    println!("args:\n{:?}", args);

    let mut config = get_config();

    println!("config:\n{:?}", config);

    joshuto::run(&mut config);
}
