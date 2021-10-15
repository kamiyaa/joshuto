use crate::commands::*;
use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;

use super::{AppExecute, Command};

impl AppExecute for Command {
    fn execute(
        &self,
        context: &mut AppContext,
        backend: &mut TuiBackend,
        keymap_t: &AppKeyMapping,
    ) -> JoshutoResult<()> {
        match &*self {
            Self::BulkRename => bulk_rename::bulk_rename(context, backend),
            Self::ChangeDirectory(p) => {
                change_directory::change_directory(context, p.as_path())?;
                Ok(())
            }
            Self::NewTab => tab_ops::new_tab(context),
            Self::CloseTab => tab_ops::close_tab(context),
            Self::CommandLine(p, s) => {
                command_line::read_and_execute(context, backend, keymap_t, p.as_str(), s.as_str())
            }
            Self::CutFiles => file_ops::cut(context),
            Self::CopyFiles => file_ops::copy(context),
            Self::PasteFiles(options) => file_ops::paste(context, *options),
            Self::CopyFileName => file_ops::copy_filename(context),
            Self::CopyFileNameWithoutExtension => {
                file_ops::copy_filename_without_extension(context)
            }
            Self::CopyFilePath => file_ops::copy_filepath(context),
            Self::CopyDirPath => file_ops::copy_dirpath(context),

            Self::CursorMoveUp(u) => cursor_move::up(context, *u),
            Self::CursorMoveDown(u) => cursor_move::down(context, *u),
            Self::CursorMoveHome => cursor_move::home(context),
            Self::CursorMoveEnd => cursor_move::end(context),
            Self::CursorMovePageUp => cursor_move::page_up(context, backend),
            Self::CursorMovePageDown => cursor_move::page_down(context, backend),

            Self::ParentCursorMoveUp(u) => parent_cursor_move::parent_up(context, *u),
            Self::ParentCursorMoveDown(u) => parent_cursor_move::parent_down(context, *u),

            Self::DeleteFiles => {
                delete_files::delete_selected_files(context, backend)?;
                Ok(())
            }
            Self::NewDirectory(p) => new_directory::new_directory(context, p.as_path()),
            Self::OpenFile => open_file::open(context, backend),
            Self::OpenFileWith(None) => open_file::open_with_interactive(context, backend),
            Self::OpenFileWith(Some(i)) => open_file::open_with_index(context, backend, *i),
            Self::ParentDirectory => parent_directory::parent_directory(context),

            Self::Quit => quit::quit(context),
            Self::QuitToCurrentDirectory => quit::quit_to_current_directory(context),
            Self::ForceQuit => quit::force_quit(context),

            Self::ReloadDirList => reload::reload_dirlist(context),
            Self::RenameFile(p) => rename_file::rename_file(context, p.as_path()),
            Self::RenameFileAppend => rename_file::rename_file_append(context, backend, keymap_t),
            Self::RenameFilePrepend => rename_file::rename_file_prepend(context, backend, keymap_t),
            Self::TouchFile(arg) => touch_file::touch_file(context, arg.as_str()),
            Self::SearchGlob(pattern) => search_glob::search_glob(context, pattern.as_str()),
            Self::SearchString(pattern) => search_string::search_string(context, pattern.as_str()),
            Self::SearchFzf => search_fzf::search_fzf(context, backend),
            Self::SearchNext => search::search_next(context),
            Self::SearchPrev => search::search_prev(context),

            Self::SelectFiles(pattern, options) => {
                selection::select_files(context, pattern.as_str(), options)
            }
            Self::SetMode => set_mode::set_mode(context, backend),
            Self::SubProcess(v, spawn) => {
                sub_process::sub_process(context, backend, v.as_slice(), *spawn)
            }
            Self::ShowWorkers => show_workers::show_workers(context, backend, keymap_t),

            Self::ToggleHiddenFiles => show_hidden::toggle_hidden(context),

            Self::Sort(t) => sort::set_sort(context, *t),
            Self::SortReverse => sort::toggle_reverse(context),

            Self::TabSwitch(i) => {
                tab_ops::tab_switch(*i, context)?;
                Ok(())
            }
            Self::TabSwitchIndex(i) => tab_ops::tab_switch_index(*i as usize, context),
            Self::Help => help::help_loop(context, backend, keymap_t),
        }
    }
}
