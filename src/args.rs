use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::config::ConfigType;

/// joshuto's command-line arguments.
#[derive(Clone, Debug, Parser)]
#[command(author, about)]
pub struct AppArgs {
    #[command(subcommand)]
    pub commands: Option<AppArgCommand>,
    #[arg(short = 'v', long = "version")]
    pub version: bool,
    /// On quit, print the current working directory (for shell `cd` integration).
    #[arg(long = "change-directory")]
    pub change_directory: bool,
    /// On opening a file, print the selected file(s) instead of launching a program.
    #[arg(long = "file-chooser")]
    pub file_chooser: bool,
    /// Write stdout to a file instead
    #[arg(long = "output-file")]
    pub output_file: Option<PathBuf>,
    /// Wildcard for all other arguments, used as a directory argument to start joshuto
    /// in which directory
    #[arg(name = "ARGUMENTS")]
    pub rest: Vec<PathBuf>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum AppArgCommand {
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

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{AppArgCommand, AppArgs};

    #[test]
    fn test_command_new() {
        AppArgs::parse_from(["program_name"]);
    }

    #[test]
    fn test_command_version() {
        match AppArgs::parse_from(["program_name", "version"]).commands {
            Some(AppArgCommand::Version) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test_command_completions() {
        for shell in ["bash", "zsh", "fish", "elvish", "powershell"] {
            match AppArgs::parse_from(["program_name", "completions", shell]).commands {
                Some(AppArgCommand::Completions { shell: _ }) => {}
                _ => panic!(),
            }
        }
    }
}
