pub mod process_event;
pub mod process_io;
pub mod process_run_loop;

pub use process_io::*;

use std::fs::File;
use std::io::Write;

use clap::CommandFactory;

use crate::{
    app_command::commands::quit::QuitAction,
    args::{AppArgCommand, AppArgs},
    config::{AppConfig, AppKeybindConfig, AppPersistentConfig},
    error::{AppError, AppErrorCode, AppResult},
    global, print_version,
    state::AppState,
    ui,
    utils::set_current_dir,
};

pub fn run_app(args: AppArgs) -> AppResult<i32> {
    if let Some(command) = args.commands {
        let result = match command {
            AppArgCommand::Completions { shell } => {
                let mut app = AppArgs::command();
                let bin_name = app.get_name().to_string();
                clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
                Ok(0)
            }
            AppArgCommand::Config { config_type } => match config_type.embedded_config() {
                None => {
                    let err = AppError::new(
                        AppErrorCode::NoDefaultConfig,
                        "no default config".to_string(),
                    );
                    Err(err)
                }
                Some(config) => {
                    println!("{config}");
                    Ok(0)
                }
            },
            AppArgCommand::Version => {
                print_version();
                Ok(0)
            }
        };
        return result;
    }

    if let Some(path) = args.rest.first() {
        set_current_dir(path)?;
    }

    // make sure all configs have been loaded before starting
    let config = AppConfig::read_config();
    let keymap = AppKeybindConfig::read_config();

    lazy_static::initialize(&global::APP_THEME);
    lazy_static::initialize(&global::APP_MIMETYPE);
    lazy_static::initialize(&global::APP_PREVIEW);
    lazy_static::initialize(&global::APP_BOOKMARKS);
    lazy_static::initialize(&global::APP_ICONS);

    lazy_static::initialize(&global::HOME_DIR);
    lazy_static::initialize(&global::USERNAME);
    lazy_static::initialize(&global::HOSTNAME);

    let mouse_support = config.mouse_support;
    let mut app_state = AppState::new(config, args.clone());
    {
        let mut backend: ui::AppBackend = ui::AppBackend::new(mouse_support)?;
        process_run_loop::run_loop(&mut backend, &mut app_state, keymap)?;
    }
    run_quit(&args, &app_state)?;
    Ok(app_state.quit.exit_code())
}

/// Writes the output requested by the quit action (current directory or selected files) to
/// `--output-file` if given, otherwise to stderr.
fn run_quit(args: &AppArgs, app_state: &AppState) -> AppResult {
    let curr_tab = app_state.state.tab_state.curr_tab_ref();
    let lines: Vec<String> = match app_state.quit {
        QuitAction::OutputCurrentDirectory => {
            let curr_path = match args.output_file {
                Some(_) => curr_tab.get_cwd().to_path_buf(),
                None => std::env::current_dir()?,
            };
            vec![curr_path.to_string_lossy().to_string()]
        }
        QuitAction::OutputSelectedFiles => curr_tab
            .curr_list_ref()
            .into_iter()
            .flat_map(|s| s.get_selected_paths())
            .map(|p| p.display().to_string())
            .collect(),
        _ => return Ok(()),
    };

    match &args.output_file {
        Some(output_path) => {
            let mut file = File::create(output_path)?;
            for line in lines {
                writeln!(file, "{line}")?;
            }
        }
        None => {
            for line in lines {
                eprintln!("{line}");
            }
        }
    }
    Ok(())
}
