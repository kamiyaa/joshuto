mod commands;
mod config;
mod context;
mod error;
mod event;
mod fs;
mod history;
mod io;
mod key_command;
mod preview;
mod run;
mod shadow;
mod tab;
mod traits;
mod ui;
mod util;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;
use std::sync::Mutex;

use clap::{CommandFactory, Parser, Subcommand};
use config::clean::app::AppConfig;
use config::clean::icon::Icons;
use config::clean::keymap::AppKeyMapping;
use config::clean::preview::FileEntryPreview;
use lazy_static::lazy_static;

use config::clean::bookmarks::Bookmarks;
use config::clean::mimetype::AppProgramRegistry;
use config::clean::theme::AppTheme;
use config::{ConfigType, TomlConfigFile};
use util::cwd;

use crate::commands::quit::QuitAction;

use crate::context::AppContext;
use crate::error::AppError;

const PROGRAM_NAME: &str = "joshuto";
const CONFIG_HOME: &str = "JOSHUTO_CONFIG_HOME";

lazy_static! {
    // dynamically builds the config hierarchy
    static ref CONFIG_HIERARCHY: Vec<PathBuf> = {
        let mut config_dirs = vec![];

        if let Ok(p) = std::env::var(CONFIG_HOME) {
            let p = PathBuf::from(p);
            if p.is_dir() {
                config_dirs.push(p);
            }
        }

        if let Ok(dirs) = xdg::BaseDirectories::with_prefix(PROGRAM_NAME) {
            config_dirs.push(dirs.get_config_home());
        }

        if let Ok(p) = std::env::var("HOME") {
            let mut p = PathBuf::from(p);
            p.push(format!(".config/{}", PROGRAM_NAME));
            if p.is_dir() {
                config_dirs.push(p);
            }
        }

        config_dirs
    };
    static ref THEME_T: AppTheme = AppTheme::get_config();
    static ref MIMETYPE_T: AppProgramRegistry = AppProgramRegistry::get_config();
    static ref PREVIEW_T: FileEntryPreview = FileEntryPreview::get_config();
    static ref BOOKMARKS_T: Mutex<Bookmarks> = Mutex::new(Bookmarks::get_config());
    static ref ICONS_T: Icons = Icons::get_config();

    static ref HOME_DIR: Option<PathBuf> = dirs_next::home_dir();

    static ref USERNAME: String = whoami::fallible::username().unwrap_or("No Username".to_string());
    static ref HOSTNAME: String = whoami::fallible::hostname().unwrap_or("No Hostname".to_string());

    static ref TIMEZONE_STR: String = {
        let offset = chrono::Local::now().offset().local_minus_utc() / 3600;
        if offset.is_positive() {
            format!(" UTC+{} ", offset.abs())
        } else {
            format!(" UTC-{} ", offset.abs())
        }
    };
}

#[derive(Clone, Debug, Parser)]
#[command(author, about)]
pub struct Args {
    #[command(subcommand)]
    commands: Option<Commands>,

    #[arg(short = 'v', long = "version")]
    version: bool,

    #[arg(long = "change-directory")]
    change_directory: bool,

    #[arg(long = "file-chooser")]
    file_chooser: bool,

    #[arg(long = "output-file")]
    output_file: Option<PathBuf>,

    #[arg(name = "ARGUMENTS")]
    rest: Vec<PathBuf>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Print completions for a given shell.
    Completions { shell: clap_complete::Shell },

    /// Print embedded toml configuration for a given config type.
    Config {
        /// Filename of the given config without '.toml' extension.
        config_type: ConfigType,
    },

    /// Print 'joshuto' build version.
    Version,
}

fn run_main(args: Args) -> Result<i32, AppError> {
    if let Some(command) = args.commands {
        let result = match command {
            Commands::Completions { shell } => {
                let mut app = Args::command();
                let bin_name = app.get_name().to_string();
                clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
                Ok(0)
            }
            Commands::Config { config_type } => match config_type.embedded_config() {
                None => AppError::fail("no default config"),
                Some(config) => {
                    println!("{config}");
                    Ok(0)
                }
            },
            Commands::Version => print_version(),
        };
        return result;
    }

    if args.version {
        return print_version();
    }

    if let Some(path) = args.rest.first() {
        cwd::set_current_dir(path)?;
    }

    // make sure all configs have been loaded before starting
    let config = AppConfig::get_config();
    let keymap = AppKeyMapping::get_config();

    lazy_static::initialize(&THEME_T);
    lazy_static::initialize(&MIMETYPE_T);
    lazy_static::initialize(&PREVIEW_T);
    lazy_static::initialize(&BOOKMARKS_T);
    lazy_static::initialize(&ICONS_T);

    lazy_static::initialize(&HOME_DIR);
    lazy_static::initialize(&USERNAME);
    lazy_static::initialize(&HOSTNAME);

    let mouse_support = config.mouse_support;
    let mut context = AppContext::new(config, args.clone());
    {
        let mut backend: ui::AppBackend = ui::AppBackend::new(mouse_support)?;
        run::run_loop(&mut backend, &mut context, keymap)?;
    }
    run_quit(&args, &context)?;
    Ok(context.quit.exit_code())
}

fn run_quit(args: &Args, context: &AppContext) -> Result<(), AppError> {
    match &args.output_file {
        Some(output_path) => match context.quit {
            QuitAction::OutputCurrentDirectory => {
                let curr_path = context.tab_context_ref().curr_tab_ref().cwd();
                let mut file = File::create(output_path)?;
                file.write_all(curr_path.as_os_str().to_string_lossy().as_bytes())?;
                file.write_all("\n".as_bytes())?;
            }
            QuitAction::OutputSelectedFiles => {
                let curr_tab = context.tab_context_ref().curr_tab_ref();
                let selected_files = curr_tab
                    .curr_list_ref()
                    .into_iter()
                    .flat_map(|s| s.get_selected_paths());
                let mut f = File::create(output_path)?;
                for file in selected_files {
                    writeln!(f, "{}", file.display())?;
                }
            }
            _ => {}
        },
        None => match context.quit {
            QuitAction::OutputCurrentDirectory => {
                let curr_path = std::env::current_dir()?;
                eprintln!(
                    "{}",
                    curr_path.into_os_string().as_os_str().to_string_lossy()
                );
            }
            QuitAction::OutputSelectedFiles => {
                let curr_tab = context.tab_context_ref().curr_tab_ref();
                let selected_files = curr_tab
                    .curr_list_ref()
                    .into_iter()
                    .flat_map(|s| s.get_selected_paths());
                for file in selected_files {
                    eprintln!("{}", file.display());
                }
            }
            _ => {}
        },
    }
    Ok(())
}

fn print_version() -> Result<i32, AppError> {
    writeln!(
        &mut std::io::stdout(),
        "{PROGRAM_NAME}-{}",
        shadow::CLAP_LONG_VERSION
    )?;
    Ok(0)
}

fn main() {
    let args = Args::parse();

    match run_main(args) {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::{Args, Commands};

    #[test]
    fn test_command_new() {
        Args::parse_from(["program_name"]);
    }

    #[test]
    fn test_command_version() {
        match Args::parse_from(["program_name", "version"]).commands {
            Some(Commands::Version) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test_command_completions() {
        for shell in ["bash", "zsh", "fish", "elvish", "powershell"] {
            match Args::parse_from(["program_name", "completions", shell]).commands {
                Some(Commands::Completions { shell: _ }) => {}
                _ => panic!(),
            }
        }
    }
}
