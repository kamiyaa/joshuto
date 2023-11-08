use crate::context::AppContext;
use crate::error::AppResult;
use crate::ui::AppBackend;
use crate::{commands::*, config::clean::keymap::AppKeyMapping};

use super::{AppExecute, Command};

impl AppExecute for Command {
    fn execute(
        &self,
        context: &mut AppContext,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> AppResult {
        match self {
            Self::Escape => escape::escape(context),
            Self::ToggleVisualMode => uimodes::toggle_visual_mode(context),

            Self::BulkRename => bulk_rename::bulk_rename(context, backend),

            Self::ChangeDirectory { path } => {
                change_directory::change_directory(context, path.as_path())?;
                Ok(())
            }
            Self::ParentDirectory => change_directory::parent_directory(context),
            Self::PreviousDirectory => change_directory::previous_directory(context),

            Self::NewTab { mode } => tab_ops::new_tab(context, mode),
            Self::CloseTab => tab_ops::close_tab(context),
            Self::CommandLine { prefix, suffix } => command_line::read_and_execute(
                context,
                backend,
                keymap_t,
                prefix.as_str(),
                suffix.as_str(),
            ),
            Self::CutFiles => file_ops::cut(context),
            Self::CopyFiles => file_ops::copy(context),
            Self::CopyFileName => file_ops::copy_filename(context),
            Self::CopyFileNameWithoutExtension => {
                file_ops::copy_filename_without_extension(context)
            }
            Self::CopyFilePath {
                all_selected: false,
            } => file_ops::copy_filepath(context, false),
            Self::CopyFilePath { all_selected: true } => file_ops::copy_filepath(context, true),
            Self::CopyDirPath => file_ops::copy_dirpath(context),
            Self::SymlinkFiles { relative: true } => file_ops::symlink_relative(context),
            Self::SymlinkFiles { relative: false } => file_ops::symlink_absolute(context),
            Self::PasteFiles { options } => file_ops::paste(context, *options),

            Self::DeleteFiles {
                background,
                permanently,
                noconfirm,
            } => delete_files::delete_selected_files(
                context,
                backend,
                *background,
                *permanently,
                *noconfirm,
            ),

            Self::CursorMoveUp { offset } => cursor_move::up(context, *offset),
            Self::CursorMoveDown { offset } => cursor_move::down(context, *offset),
            Self::CursorMoveHome => cursor_move::home(context),
            Self::CursorMoveEnd => cursor_move::end(context),
            Self::CursorMovePageUp(p) => cursor_move::page_up(context, backend, *p),
            Self::CursorMovePageDown(p) => cursor_move::page_down(context, backend, *p),

            Self::CursorMovePageHome => cursor_move::page_home(context, backend),
            Self::CursorMovePageMiddle => cursor_move::page_middle(context, backend),
            Self::CursorMovePageEnd => cursor_move::page_end(context, backend),

            Self::ParentCursorMoveUp { offset } => parent_cursor_move::parent_up(context, *offset),
            Self::ParentCursorMoveDown { offset } => {
                parent_cursor_move::parent_down(context, *offset)
            }

            Self::PreviewCursorMoveUp { offset } => {
                preview_cursor_move::preview_up(context, *offset)
            }
            Self::PreviewCursorMoveDown { offset } => {
                preview_cursor_move::preview_down(context, *offset)
            }

            Self::NewDirectory { path } => new_directory::new_directory(context, path.as_path()),
            Self::OpenFile => open_file::open(context, backend),
            Self::OpenFileWith { index: None } => {
                open_file::open_with_interactive(context, backend)
            }
            Self::OpenFileWith { index: Some(i) } => {
                open_file::open_with_index(context, backend, *i)
            }

            Self::Quit(action) => quit::quit_with_action(context, *action),

            Self::ReloadDirList => reload::reload_dirlist(context),
            Self::RenameFile { new_name } => rename_file::rename_file(context, new_name.as_path()),
            Self::RenameFileAppend => rename_file::rename_file_append(context, backend, keymap_t),
            Self::RenameFileAppendBase => {
                rename_file::rename_file_append_base(context, backend, keymap_t)
            }
            Self::RenameFilePrepend => rename_file::rename_file_prepend(context, backend, keymap_t),
            Self::RenameFileKeepExt => {
                rename_file::rename_file_keep_ext(context, backend, keymap_t)
            }
            Self::TouchFile { file_name } => touch_file::touch_file(context, file_name),
            Self::SearchGlob { pattern } => search_glob::search_glob(context, pattern.as_str()),
            Self::SearchRegex { pattern } => search_regex::search_regex(context, pattern.as_str()),
            Self::SearchString { pattern } => {
                search_string::search_string(context, pattern.as_str(), false);
                Ok(())
            }
            // We call `interactive_execute` on each key press, so even before Enter is pressed the
            // cursor will be one the selected word. And as `interactive_execute` for
            // `SearchIncremental` always starts from index 0, this operation will be a no-op
            Self::SearchIncremental { .. } => Ok(()),
            Self::SearchNext => search::search_next(context),
            Self::SearchPrev => search::search_prev(context),

            Self::SelectGlob { pattern, options } => {
                select_glob::select_glob(context, pattern, options)
            }
            Self::SelectRegex { pattern, options } => {
                select_regex::select_regex(context, pattern, options)
            }
            Self::SelectString { pattern, options } => {
                select_string::select_string(context, pattern, options)
            }
            Self::SetCaseSensitivity {
                case_sensitivity,
                set_type,
            } => case_sensitivity::set_case_sensitivity(context, *case_sensitivity, *set_type),
            Self::SetMode => set_mode::set_mode(context, backend),
            Self::ShowTasks => show_tasks::show_tasks(context, backend, keymap_t),
            Self::Sort(t) => sort::set_sort(context, *t),
            Self::SetLineMode(mode) => linemode::set_linemode(context, *mode),
            Self::SortReverse => sort::toggle_reverse(context),
            Self::SubProcess { words, spawn } => {
                sub_process::sub_process(context, backend, words.as_slice(), *spawn)
            }
            Self::SwitchLineNums(d) => line_nums::switch_line_numbering(context, *d),

            Self::Flat { depth } => flat::flatten(context, *depth),
            Self::NumberedCommand { initial } => {
                numbered_command::numbered_command(context, backend, keymap_t, *initial)
            }

            Self::FilterGlob { pattern } => filter_glob::filter_glob(context, pattern.as_str()),
            Self::FilterRegex { pattern } => filter_regex::filter_regex(context, pattern.as_str()),
            Self::FilterString { pattern } => {
                filter_string::filter_string(context, pattern.as_str())
            }

            Self::ToggleHiddenFiles => show_hidden::toggle_hidden(context),

            Self::TabSwitch { offset } => {
                tab_ops::tab_switch(context, *offset).map_err(|e| e.into())
            }
            Self::TabSwitchIndex { index } => tab_ops::tab_switch_index(context, *index),
            Self::Help => show_help::help_loop(context, backend, keymap_t),

            Self::SearchFzf => search_fzf::search_fzf(context, backend),
            Self::SubdirFzf => subdir_fzf::subdir_fzf(context, backend),
            Self::SelectFzf { options } => select_fzf::select_fzf(context, backend, options),
            Self::Zoxide(arg) => zoxide::zoxide_query(context, arg),
            Self::ZoxideInteractive => zoxide::zoxide_query_interactive(context, backend),

            Self::BookmarkAdd => bookmark::add_bookmark(context, backend),
            Self::BookmarkChangeDirectory => bookmark::change_directory_bookmark(context, backend),

            Self::CustomSearch(words) => {
                custom_search::custom_search(context, backend, words.as_slice(), false)
            }
            Self::CustomSearchInteractive(words) => {
                custom_search::custom_search(context, backend, words.as_slice(), true)
            }
        }
    }
}
