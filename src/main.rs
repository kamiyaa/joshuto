#[macro_use]
extern crate serde_derive;

extern crate ncurses;
extern crate toml;
extern crate xdg;

use std::char;
use std::env;
use std::fs;

mod joshuto;
mod joshuto_sort;
mod joshuto_unix;

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
    keymaps: Option<JoshutoKeymaps>,
}

#[derive(Debug, Deserialize)]
pub struct JoshutoKeymaps {
    up : i32,
}

fn generate_default_config() -> JoshutoConfig
{
    JoshutoConfig {
        show_hidden: Some(false),
        color_scheme: None,
        sort_method: Some("Natural".to_string()),
        keymaps: Some(JoshutoKeymaps {
            up : 3,
        }),
    }
}

fn get_config() -> JoshutoConfig
{
    let dirs = xdg::BaseDirectories::with_profile(PROGRAM_NAME, "").unwrap();
    match dirs.find_config_file(CONFIG_FILE) {
        Some(config_path) => {
            let config_contents = fs::read_to_string(&config_path).unwrap();

            match toml::from_str(&config_contents) {
                Ok(config) => {
                    config
                },
                Err(e) => {
                    println!("{}", e);
                    generate_default_config()
                }
            }
        },
        None => {
            generate_default_config()
        }
    }
}

#[cfg(not(feature = "wide"))]
fn main()
{
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = get_config();
    println!("{:#?}", config);

    joshuto::run(&config);
}
