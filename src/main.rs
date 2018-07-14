#[macro_use]
extern crate serde_derive;

extern crate ncurses;
extern crate toml;

use std::env;
use std::fs;
use std::path;

mod joshuto;

#[derive(Debug, Deserialize)]
struct JoshutoConfig {
    show_hidden: Option<bool>,
    color_scheme: Option<String>,
    sort_method: Option<String>,
    keymaps: Option<JoshutoKeymaps>,
}

#[derive(Debug, Deserialize)]
struct JoshutoKeymaps {
    up : i32,
}

/*
pub struct joshuto_win {
    win : ncurses::WINDOW,
    offset : usize,
    row_size : i32,
    col_size : i32,
}
*/

fn get_config_path() -> path::PathBuf
{
    let mut pathbuf : path::PathBuf;
    match env::home_dir() {
        Some(path) => {
            pathbuf = path.to_path_buf();
            pathbuf.push(".config/joshuto/joshuto.toml");
        },
        None => {
            pathbuf = path::PathBuf::new();
            pathbuf.push("/etc/joshuto/joshuto.toml");
        },
    };
    pathbuf
}

fn read_config(config_path : &path::PathBuf) -> JoshutoConfig
{
    // let mut config_file = fs::File::open(config_path).expect("No config found");

    let config_contents = fs::read_to_string(&config_path).unwrap();

/*
    let mut config_contents = String::new();
    config_file.read().read_to_string(&mut config_contents).expect("Error reading config file");
*/

    match toml::from_str(&config_contents) {
        Ok(config) => {
            config
        },
        Err(e) => {
            println!("{}", e);
            JoshutoConfig {
                show_hidden: Some(false),
                color_scheme: None,
                sort_method: Some("Natural".to_string()),
                keymaps: Some(JoshutoKeymaps {
                    up : 3,
                }),
            }
        }
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config_path = get_config_path();
    println!("{:?}", config_path);
    let config = read_config(&config_path);
    println!("{:#?}", config);

    joshuto::run();
}
