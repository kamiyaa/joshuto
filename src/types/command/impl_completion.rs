use crate::constants::command_name::*;
use crate::traits::app_execute::CommandCompletion;
use crate::types::completion_kind::CompletionKind;

use super::Command;

impl CommandCompletion for Command {
    fn completion_kind<'a>(cmd: &'a str) -> Option<CompletionKind<'a>> {
        Some(match cmd {
            CMD_CHANGE_DIRECTORY => CompletionKind::Dir(None),
            CMD_DELETE_FILES => CompletionKind::Custom(vec![
                "--background=false",
                "--background=true",
                "--noconfirm",
                "--permanently",
            ]),
            CMD_NEW_TAB => CompletionKind::Dir(Some(vec!["--current", "--cursor", "--last"])),
            CMD_OPEN_FILE_WITH
            | CMD_SUBPROCESS_CAPTURE
            | CMD_SUBPROCESS_INTERACTIVE
            | CMD_SUBPROCESS_SPAWN => CompletionKind::Bin,
            CMD_QUIT => CompletionKind::Custom(vec![
                "--force",
                "--output-current-directory",
                "--output-file",
                "--output-selected-files",
            ]),
            CMD_SEARCH_INCREMENTAL | CMD_SEARCH_STRING => CompletionKind::File,
            CMD_SELECT_FZF | CMD_SELECT_GLOB | CMD_SELECT_REGEX | CMD_SELECT_STRING => {
                CompletionKind::Custom(vec![
                    "--all=false",
                    "--all=true",
                    "--deselect=false",
                    "--deselect=true",
                    "--toggle=false",
                    "--toggle=true",
                ])
            }
            CMD_SET_CASE_SENSITIVITY => CompletionKind::Custom(vec![
                "--type=fzf",
                "--type=glob",
                "--type=regex",
                "--type=string",
                "auto",
                "insensitive",
                "sensitive",
            ]),
            CMD_SET_DISPLAY_MODE => CompletionKind::Custom(vec!["default", "minimal", "hsplit"]),
            CMD_SET_LINEMODE => CompletionKind::Custom(vec![
                "all", "group", "mtime", "none", "perm", "size", "user",
            ]),
            CMD_SORT => CompletionKind::Custom(vec![
                "--reverse=false",
                "--reverse=true",
                "ext",
                "lexical",
                "mtime",
                "natural",
                "reverse",
                "size",
            ]),
            CMD_SWITCH_LINE_NUMBERS => CompletionKind::Custom(vec!["absolute", "none", "relative"]),
            CMD_SYMLINK_FILES => {
                CompletionKind::Custom(vec!["--relative=false", "--relative=true"])
            }
            _ => CompletionKind::File,
        })
    }
}
