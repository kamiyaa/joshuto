extern crate serde;

use self::serde::de::DeserializeOwned;

pub mod config;
pub mod keymap;
pub mod mimetype;
pub mod preview;
pub mod theme;

pub use self::config::JoshutoConfig;
pub use self::keymap::JoshutoKeymap;
pub use self::mimetype::JoshutoMimetype;
pub use self::preview::JoshutoPreview;
pub use self::theme::JoshutoTheme;
pub use self::theme::JoshutoColorTheme;

pub fn search_config_hierarchy(filename: &str) -> Option<std::path::PathBuf> {
    for path in ::CONFIG_HIERARCHY.iter() {
        let filepath = path.join(filename);
        if filepath.exists() {
            return Some(filepath)
        }
    }
    None
}

fn read_config(filename: &str) -> Option<String> {
    let config_path = search_config_hierarchy(filename)?;
    match std::fs::read_to_string(&config_path) {
        Ok(content) => Some(content),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        },
    }
}

trait Flattenable<T> {
    fn flatten(self) -> T;
}

fn parse_config<T, S>(filename: &str) -> Option<S>
    where T: DeserializeOwned + Flattenable<S>
{
    let config_contents = read_config(filename)?;
    let config = match toml::from_str::<T>(&config_contents) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing {} file: {}", filename, e);
            std::process::exit(1);
        },
    };
    Some(config.flatten())
}
