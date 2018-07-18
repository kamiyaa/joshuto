#[macro_use]
extern crate serde_derive;

extern crate ncurses;
extern crate toml;
extern crate xdg;

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
pub struct IntermediateConfig {
    show_hidden: Option<bool>,
    color_scheme: Option<String>,
    sort_method: Option<String>,
    keymaps: Option<JoshutoKeymaps>,
}

#[derive(Debug, Deserialize)]
pub struct JoshutoKeymaps {
    up : i32,
}

pub struct JoshutoConfig {
    show_hidden: bool,
    color_scheme: String,
    sort_method: String,
    keymaps: JoshutoKeymaps,
}

fn generate_default_config() -> JoshutoConfig
{
    JoshutoConfig {
        show_hidden: false,
        color_scheme: "default".to_string(),
        sort_method: "Natural".to_string(),
        keymaps: JoshutoKeymaps {
            up : 3,
        },
    }
}

fn read_config() -> Option<IntermediateConfig>
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
                    None
                }
            }
        },
        None => {
            None
        }
    }
}

fn convert_config(config : IntermediateConfig) -> JoshutoConfig
{
    let show_hidden : bool = 
        match config.show_hidden {
            Some(s) => s,
            None => false,
        };
    let color_scheme : String =
        match config.color_scheme {
            Some(s) => s,
            None => "default".to_string(),
        };
    let sort_method : String =
        match config.sort_method {
            Some(s) => s,
            None => "natural".to_string(),
        };
    let keymaps : JoshutoKeymaps =
        match config.keymaps {
            Some(s) => s,
            None => JoshutoKeymaps {
                        up : 3,
                    },
        };

    JoshutoConfig {
        show_hidden: show_hidden,
        color_scheme: color_scheme,
        sort_method: sort_method,
        keymaps: keymaps,
    }

}

fn get_config() -> JoshutoConfig
{
    match read_config() {
        Some(inter_config) => {
            convert_config(inter_config)
        }
        None => {
            generate_default_config()
        }
    }

}

fn main()
{
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = get_config();

    joshuto::run(&config);
}
