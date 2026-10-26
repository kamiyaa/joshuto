use clap::Parser;

use crate::args::AppArgs;

mod app_command;
mod args;
mod config;
mod constants;
mod error;
mod fs;
mod global;
mod run;
mod shadow;
mod state;
mod tab;
mod tasks;
mod ui;
mod utils;

const PROGRAM_NAME: &str = "joshuto";
const CONFIG_HOME: &str = "JOSHUTO_CONFIG_HOME";

lazy_static::lazy_static! {
    static ref USERNAME: String = whoami::username().unwrap_or("No Username".to_string());
    static ref HOSTNAME: String = whoami::hostname().unwrap_or("No Hostname".to_string());
}

fn print_version() {
    println!("{PROGRAM_NAME}-{}", shadow::CLAP_LONG_VERSION);
}

fn main() {
    let args = AppArgs::parse();

    let log = global::APP_LOG.clone();
    tracing_subscriber::fmt()
        .with_writer(move || log.clone())
        .with_ansi(false)
        .init();

    match run::run_app(args) {
        Ok(ret) => {
            std::process::exit(ret);
        }
        Err(err) => {
            eprintln!("{:?}", err);
            std::process::exit(1);
        }
    }
}
