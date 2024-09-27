use crate::commands::stdout::post_process_std_out;
use crate::error::AppResult;
use crate::traits::app_execute::AppExecute;
use crate::types::keymap::AppKeyMapping;
use crate::types::state::AppState;
use crate::ui::AppBackend;

use crate::commands::*;

use super::Command;

impl AppExecute for Command {
    fn execute(
        &self,
        app_state: &mut AppState,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> AppResult {
        match self {
            Self::Escape => escape::escape(app_state),
            Self::ToggleVisualMode => uimodes::toggle_visual_mode(app_state),

            Self::BulkRename => bulk_rename::bulk_rename(app_state, backend),

            Self::ChangeDirectory { path } => {
                change_directory::change_directory(app_state, path.as_path())?;
                Ok(())
            }
            Self::ParentDirectory => change_directory::parent_directory(app_state),
            Self::PreviousDirectory => change_directory::previous_directory(app_state),

            Self::NewTab { mode, last } => tab_ops::new_tab(app_state, mode, *last),
            Self::CloseTab => tab_ops::close_tab(app_state),
            Self::CommandLine { prefix, suffix } => command_line::read_and_execute(
                app_state,
                backend,
                keymap_t,
                prefix.as_str(),
                suffix.as_str(),
            ),
            Self::CutFiles => file_ops::cut(app_state),
            Self::CopyFiles => file_ops::copy(app_state),
            Self::CopyFileName => file_ops::copy_filename(app_state),
            Self::CopyFileNameWithoutExtension => {
                file_ops::copy_filename_without_extension(app_state)
            }
            Self::CopyFilePath {
                all_selected: false,
            } => file_ops::copy_filepath(app_state, false),
            Self::CopyFilePath { all_selected: true } => file_ops::copy_filepath(app_state, true),
            Self::CopyDirPath => file_ops::copy_dirpath(app_state),
            Self::SymlinkFiles { relative: true } => file_ops::symlink_relative(app_state),
            Self::SymlinkFiles { relative: false } => file_ops::symlink_absolute(app_state),
            Self::PasteFiles { options } => file_ops::paste(app_state, *options),

            Self::DeleteFiles {
                background,
                permanently,
                noconfirm,
            } => delete_files::delete_selected_files(
                app_state,
                backend,
                *background,
                *permanently,
                *noconfirm,
            ),

            Self::CursorMoveUp { offset } => cursor_move::up(app_state, *offset),
            Self::CursorMoveDown { offset } => cursor_move::down(app_state, *offset),
            Self::CursorMoveHome => cursor_move::home(app_state),
            Self::CursorMoveEnd => cursor_move::end(app_state),
            Self::CursorMovePageUp(p) => cursor_move::page_up(app_state, backend, *p),
            Self::CursorMovePageDown(p) => cursor_move::page_down(app_state, backend, *p),

            Self::CursorMovePageHome => cursor_move::page_home(app_state, backend),
            Self::CursorMovePageMiddle => cursor_move::page_middle(app_state, backend),
            Self::CursorMovePageEnd => cursor_move::page_end(app_state, backend),

            Self::ParentCursorMoveUp { offset } => {
                parent_cursor_move::parent_up(app_state, *offset)
            }
            Self::ParentCursorMoveDown { offset } => {
                parent_cursor_move::parent_down(app_state, *offset)
            }

            Self::PreviewCursorMoveUp { offset } => {
                preview_cursor_move::preview_up(app_state, *offset)
            }
            Self::PreviewCursorMoveDown { offset } => {
                preview_cursor_move::preview_down(app_state, *offset)
            }

            Self::NewDirectory { path } => new_directory::new_directory(app_state, path.as_path()),
            Self::OpenFile => open_file::open(app_state, backend),
            Self::OpenFileWith { index: None } => {
                open_file::open_with_interactive(app_state, backend)
            }
            Self::OpenFileWith { index: Some(i) } => {
                open_file::open_with_index(app_state, backend, *i)
            }

            Self::Quit(action) => quit::quit_with_action(app_state, *action),

            Self::ReloadDirList => reload::reload_dirlist(app_state),
            Self::RenameFile { new_name } => {
                rename_file::rename_file(app_state, new_name.as_path())
            }
            Self::RenameFileAppend => rename_file::rename_file_append(app_state, backend, keymap_t),
            Self::RenameFileAppendBase => {
                rename_file::rename_file_append_base(app_state, backend, keymap_t)
            }
            Self::RenameFilePrepend => {
                rename_file::rename_file_prepend(app_state, backend, keymap_t)
            }
            Self::RenameFileKeepExt => {
                rename_file::rename_file_keep_ext(app_state, backend, keymap_t)
            }
            Self::TouchFile { file_name } => touch_file::touch_file(app_state, file_name),
            Self::SearchGlob { pattern } => search_glob::search_glob(app_state, pattern.as_str()),
            Self::SearchRegex { pattern } => {
                search_regex::search_regex(app_state, pattern.as_str())
            }
            Self::SearchString { pattern } => {
                search_string::search_string(app_state, pattern.as_str(), false);
                Ok(())
            }
            // We call `interactive_execute` on each key press, so even before Enter is pressed the
            // cursor will be one the selected word. And as `interactive_execute` for
            // `SearchIncremental` always starts from index 0, this operation will be a no-op
            Self::SearchIncremental { .. } => Ok(()),
            Self::SearchNext => search::search_next(app_state),
            Self::SearchPrev => search::search_prev(app_state),

            Self::SelectGlob { pattern, options } => {
                select_glob::select_glob(app_state, pattern, options)
            }
            Self::SelectRegex { pattern, options } => {
                select_regex::select_regex(app_state, pattern, options)
            }
            Self::SelectString { pattern, options } => {
                select_string::select_string(app_state, pattern, options)
            }
            Self::SetCaseSensitivity {
                case_sensitivity,
                set_type,
            } => case_sensitivity::set_case_sensitivity(app_state, *case_sensitivity, *set_type),
            Self::SetMode => set_mode::set_mode(app_state, backend),
            Self::ShowTasks => show_tasks::show_tasks(app_state, backend, keymap_t),
            Self::Sort {
                sort_method,
                reverse,
            } => sort::set_sort(app_state, *sort_method, *reverse),
            Self::SetLineMode(mode) => linemode::set_linemode(app_state, *mode),
            Self::SortReverse => sort::toggle_reverse(app_state),
            Self::SignalSuspend => signal::signal_suspend(backend),
            Self::SubProcess { words, mode } => {
                sub_process::sub_process(app_state, backend, words.as_slice(), mode.clone())
            }
            Self::StdOutPostProcess { processor } => post_process_std_out(processor, app_state),
            Self::SwitchLineNums(d) => line_nums::switch_line_numbering(app_state, *d),

            Self::Flat { depth } => flat::flatten(app_state, *depth),
            Self::NumberedCommand { initial } => {
                numbered_command::numbered_command(app_state, backend, keymap_t, *initial)
            }

            Self::FilterGlob { pattern } => filter_glob::filter_glob(app_state, pattern.as_str()),
            Self::FilterRegex { pattern } => {
                filter_regex::filter_regex(app_state, pattern.as_str())
            }
            Self::FilterString { pattern } => {
                filter_string::filter_string(app_state, pattern.as_str())
            }

            Self::ToggleHiddenFiles => show_hidden::toggle_hidden(app_state),

            Self::TabSwitch { offset } => {
                tab_ops::tab_switch(app_state, *offset).map_err(|e| e.into())
            }
            Self::TabSwitchIndex { index } => tab_ops::tab_switch_index(app_state, *index),
            Self::Help => show_help::help_loop(app_state, backend, keymap_t),

            Self::SearchFzf => search_fzf::search_fzf(app_state, backend),
            Self::SubdirFzf => subdir_fzf::subdir_fzf(app_state, backend),
            Self::SelectFzf { options } => select_fzf::select_fzf(app_state, backend, options),
            Self::Zoxide(arg) => zoxide::zoxide_query(app_state, arg),
            Self::ZoxideInteractive(args) => {
                zoxide::zoxide_query_interactive(app_state, backend, args)
            }

            Self::BookmarkAdd => bookmark::add_bookmark(app_state, backend),
            Self::BookmarkChangeDirectory => {
                bookmark::change_directory_bookmark(app_state, backend)
            }

            Self::CustomSearch(words) => {
                custom_search::custom_search(app_state, backend, words.as_slice(), false)
            }
            Self::CustomSearchInteractive(words) => {
                custom_search::custom_search(app_state, backend, words.as_slice(), true)
            }
        }
    }
}
