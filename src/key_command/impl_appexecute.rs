use crate::commands::*;
use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::AppBackend;

use super::{AppExecute, Command};

impl AppExecute for Command {
    fn execute(
        &self,
        context: &mut AppContext,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> JoshutoResult {
        match &*self {
            Self::BulkRename => bulk_rename::bulk_rename(context, backend),

            Self::ChangeDirectory(p) => {
                change_directory::change_directory(context, p.as_path())?;
                Ok(())
            }
            Self::ParentDirectory => change_directory::parent_directory(context),
            Self::PreviousDirectory => change_directory::previous_directory(context),

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
            Self::CursorMovePageUp(p) => cursor_move::page_up(context, backend, *p),
            Self::CursorMovePageDown(p) => cursor_move::page_down(context, backend, *p),

            Self::CursorMovePageHome => cursor_move::page_home(context, backend),
            Self::CursorMovePageMiddle => cursor_move::page_middle(context, backend),
            Self::CursorMovePageEnd => cursor_move::page_end(context, backend),

            Self::ParentCursorMoveUp(u) => parent_cursor_move::parent_up(context, *u),
            Self::ParentCursorMoveDown(u) => parent_cursor_move::parent_down(context, *u),

            Self::PreviewCursorMoveUp(u) => preview_cursor_move::preview_up(context, *u),
            Self::PreviewCursorMoveDown(u) => preview_cursor_move::preview_down(context, *u),

            Self::DeleteFiles => {
                delete_files::delete_selected_files(context, backend)?;
                Ok(())
            }
            Self::NewDirectory(p) => new_directory::new_directory(context, p.as_path()),
            Self::OpenFile => open_file::open(context, backend),
            Self::OpenFileWith(None) => open_file::open_with_interactive(context, backend),
            Self::OpenFileWith(Some(i)) => open_file::open_with_index(context, backend, *i),

            Self::Quit(action) => quit::quit_with_action(context, *action),

            Self::ReloadDirList => reload::reload_dirlist(context),
            Self::RenameFile(p) => rename_file::rename_file(context, p.as_path()),
            Self::RenameFileAppend => rename_file::rename_file_append(context, backend, keymap_t),
            Self::RenameFilePrepend => rename_file::rename_file_prepend(context, backend, keymap_t),
            Self::TouchFile(arg) => touch_file::touch_file(context, arg.as_str()),
            Self::SearchGlob(pattern) => search_glob::search_glob(context, pattern.as_str()),
            Self::SearchString(pattern) => {
                search_string::search_string(context, pattern.as_str(), false);
                Ok(())
            }
            // We call `interactive_execute` on each key press, so even before Enter is pressed the
            // cursor will be one the selected word. And as `interactive_execute` for
            // `SearchIncremental` always starts from index 0, this operation will be a no-op
            Self::SearchIncremental(_) => Ok(()),
            Self::SearchNext => search::search_next(context),
            Self::SearchPrev => search::search_prev(context),

            Self::SelectFiles(pattern, options) => {
                selection::select_files(context, pattern.as_str(), options)
            }
            Self::SetMode => set_mode::set_mode(context, backend),
            Self::ShowTasks => show_tasks::show_tasks(context, backend, keymap_t),
            Self::Sort(t) => sort::set_sort(context, *t),
            Self::SortReverse => sort::toggle_reverse(context),
            Self::SubProcess(v, spawn) => {
                sub_process::sub_process(context, backend, v.as_slice(), *spawn)
            }
            Self::SwitchLineNums(d) => line_nums::switch_line_numbering(context, *d),

            Self::Flat(depth) => flat::flatten(*depth, context),

            Self::ToggleHiddenFiles => show_hidden::toggle_hidden(context),

            Self::TabSwitch(i) => {
                tab_ops::tab_switch(*i, context)?;
                Ok(())
            }
            Self::TabSwitchIndex(i) => tab_ops::tab_switch_index(*i as usize, context),
            Self::Help => show_help::help_loop(context, backend, keymap_t),

            Self::SearchFzf => search_fzf::search_fzf(context, backend),
            Self::SubdirFzf => subdir_fzf::subdir_fzf(context, backend),
            Self::Zoxide(arg) => zoxide::zoxide_query(context, arg),
            Self::ZoxideInteractive => zoxide::zoxide_query_interactive(context, backend),
        }
    }
}
