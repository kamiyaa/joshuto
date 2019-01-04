extern crate fs_extra;
extern crate ncurses;
extern crate wcwidth;

use std;
use std::fmt;
use std::fs;
use std::path;
use std::sync;
use std::thread;

use joshuto;
use joshuto::command;
use joshuto::structs;
use joshuto::ui;
use joshuto::window;

use joshuto::keymapll::Keycode;

lazy_static! {
    static ref selected_files: sync::Mutex<Vec<path::PathBuf>> = sync::Mutex::new(vec![]);
    static ref fileop: sync::Mutex<FileOp> = sync::Mutex::new(FileOp::Copy);
}

fn set_file_op(operation: FileOp)
{
    let mut data = fileop.lock().unwrap();
    *data = operation;
}

pub fn collect_selected_paths(dirlist: &structs::JoshutoDirList)
        -> Option<Vec<path::PathBuf>>
{
    let selected: Vec<path::PathBuf> = dirlist.contents.iter()
            .filter(|entry| entry.selected)
            .map(|entry| entry.path.clone()).collect();
    if selected.len() > 0 {
        Some(selected)
    } else if dirlist.index >= 0 {
        Some(vec![dirlist.contents[dirlist.index as usize].path.clone()])
    } else {
        None
    }
}

fn repopulated_selected_files(dirlist: &Option<structs::JoshutoDirList>) -> bool
{
    if let Some(s) = dirlist.as_ref() {
        if let Some(contents) = collect_selected_paths(s) {
            let mut data = selected_files.lock().unwrap();
            *data = contents;
            return true;
        }
    }
    return false;
}

enum FileOp {
    Cut,
    Copy,
}

#[derive(Debug)]
pub struct CutFiles;

impl CutFiles {
    pub fn new() -> Self { CutFiles }
    pub fn command() -> &'static str { "cut_files" }
}

impl command::JoshutoCommand for CutFiles {}

impl std::fmt::Display for CutFiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for CutFiles {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        if repopulated_selected_files(&context.curr_list) {
            set_file_op(FileOp::Cut);
        }
    }
}

#[derive(Debug)]
pub struct CopyFiles;

impl CopyFiles {
    pub fn new() -> Self { CopyFiles }
    pub fn command() -> &'static str { "copy_files" }
}

impl command::JoshutoCommand for CopyFiles {}

impl std::fmt::Display for CopyFiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for CopyFiles {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        if repopulated_selected_files(&context.curr_list) {
            set_file_op(FileOp::Copy);
        }
    }
}

#[derive(Debug,Clone)]
pub struct ProgressInfo {
    pub bytes_finished: u64,
    pub total_bytes: u64,
}

pub struct PasteFiles {
    options: fs_extra::dir::CopyOptions,
}

impl PasteFiles {
    pub fn new(options: fs_extra::dir::CopyOptions) -> Self
    {
        PasteFiles {
            options,
        }
    }
    pub fn command() -> &'static str { "paste_files" }

    fn cut(&self, destination: &path::PathBuf, win: &window::JoshutoPanel)
            -> (sync::mpsc::Receiver<command::ProgressInfo>, thread::JoinHandle<i32>)
    {
        let (tx, rx) = sync::mpsc::channel();

        let mut destination = destination.clone();
        let options = self.options.clone();

        let mut move_options = fs_extra::file::CopyOptions::new();
        move_options.overwrite = options.overwrite;
        move_options.skip_exist = options.skip_exist;
        move_options.buffer_size = options.buffer_size;

        let child = thread::spawn(move || {
            let mut paths = selected_files.lock().unwrap();

            let mut progress_info = ProgressInfo {
                    bytes_finished: 0,
                    total_bytes: paths.len() as u64,
                };

            for path in (*paths).iter() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                destination.push(file_name);
                if options.skip_exist && destination.exists() {
                    continue;
                }

                match std::fs::rename(&path, &destination) {
                    Ok(_) => {
                        destination.pop();
                    },
                    Err(_) => {
                        destination.pop();
                        if path.is_dir() {
                            fs_extra::dir::move_dir(&path, &destination, &options);
                        } else {
                            fs_extra::file::move_file(&path, &destination, &move_options);
                        }
                    }
                }

                progress_info.bytes_finished = progress_info.bytes_finished + 1;
                tx.send(progress_info.clone()).unwrap();
            }

            paths.clear();
            0
        });

        (rx, child)
    }

    fn copy(&self, destination: &path::PathBuf, win: &window::JoshutoPanel)
            -> (sync::mpsc::Receiver<command::ProgressInfo>, thread::JoinHandle<i32>)
    {
        let (tx, rx) = sync::mpsc::channel();

        let destination = destination.clone();
        let options = self.options.clone();

        let child = thread::spawn(move || {
            let mut files = selected_files.lock().unwrap();

            let handle = |process_info: fs_extra::TransitProcess| {
                let progress_info = ProgressInfo {
                        bytes_finished: process_info.copied_bytes,
                        total_bytes: process_info.total_bytes,
                    };
                tx.send(progress_info).unwrap();
                fs_extra::dir::TransitProcessResult::ContinueOrAbort
            };

            match fs_extra::copy_items_with_progress(&files, &destination, &options, handle) {
                Ok(_) => {},
                Err(_) => {},
            }
            files.clear();
            0
        });

        (rx, child)
    }
}

impl command::JoshutoCommand for PasteFiles {}

impl std::fmt::Display for PasteFiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} overwrite={}", Self::command(), self.options.overwrite)
    }
}

impl std::fmt::Debug for PasteFiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for PasteFiles {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let file_operation = fileop.lock().unwrap();

        let cprocess = match *file_operation {
                FileOp::Copy => self.copy(&context.curr_path, &context.views.bot_win),
                FileOp::Cut => self.cut(&context.curr_path, &context.views.bot_win),
            };
        context.threads.push(cprocess);

        context.reload_dirlists();

        ncurses::timeout(0);

        ui::redraw_view(&context.views.left_win, context.parent_list.as_ref());
        ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
        ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

        ui::redraw_status(&context.views, context.curr_list.as_ref(),
                &context.curr_path,
                &context.config_t.username, &context.config_t.hostname);

        ncurses::doupdate();
    }
}

#[derive(Debug)]
pub struct DeleteFiles;

impl DeleteFiles {
    pub fn new() -> Self { DeleteFiles }
    pub fn command() -> &'static str { "delete_files" }

    pub fn remove_files(paths: Vec<path::PathBuf>, win: &window::JoshutoPanel)
    {
        let (tx, rx) = sync::mpsc::channel();
        let total = paths.len();

        let child = thread::spawn(move || {
            let mut deleted = 0;
            for path in &paths {
                if let Ok(metadata) = std::fs::symlink_metadata(path) {
                    if metadata.is_dir() {
                        std::fs::remove_dir_all(&path).unwrap();
                    } else {
                        std::fs::remove_file(&path).unwrap();
                    }
                }
                deleted = deleted + 1;
                tx.send(deleted).unwrap();
            }
        });

        while let Ok(deleted) = rx.recv() {
            if deleted == total {
                ncurses::werase(win.win);
                ncurses::wnoutrefresh(win.win);
                ncurses::doupdate();
                break;
            }
            let percent = (deleted as f64 / total as f64) as f32;
            ui::draw_loading_bar(win, percent);
            ncurses::wnoutrefresh(win.win);
            ncurses::doupdate();
        }
    }
}

impl command::JoshutoCommand for DeleteFiles {}

impl std::fmt::Display for DeleteFiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for DeleteFiles {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {

        ui::wprint_msg(&context.views.bot_win,
            format!("Delete selected files? (Y/n)").as_str());
        ncurses::doupdate();

        let ch = ncurses::wgetch(context.views.bot_win.win);
        if ch == Keycode::LOWER_Y as i32 || ch == Keycode::ENTER as i32 {
            if let Some(s) = context.curr_list.as_mut() {
                if let Some(paths) = collect_selected_paths(s) {
                    Self::remove_files(paths, &context.views.bot_win);
                }
            }
            context.reload_dirlists();

            ui::wprint_msg(&context.views.bot_win, "Deleted files");

            ui::redraw_view(&context.views.left_win, context.parent_list.as_ref());
            ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
            ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());
        } else {
            ui::redraw_status(&context.views, context.curr_list.as_ref(),
                    &context.curr_path,
                    &context.config_t.username, &context.config_t.hostname);
        }
        ncurses::doupdate();
    }

}

#[derive(Debug)]
pub enum RenameFileMethod {
    Append,
    Prepend,
    Overwrite
}

#[derive(Debug)]
pub struct RenameFile {
    method: RenameFileMethod,
}

impl RenameFile {
    pub fn new(method: RenameFileMethod) -> Self
    {
        RenameFile {
            method,
        }
    }
    pub fn command() -> &'static str { "rename_file" }

    pub fn rename_file(&self, path: &path::PathBuf, context: &mut joshuto::JoshutoContext, start_str: String)
    {
        let mut term_rows: i32 = 0;
        let mut term_cols: i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

        let mut win = window::JoshutoPanel::new(1, term_cols, (term_rows as usize - 1, 0));
        ncurses::keypad(win.win, true);

        const PROMPT: &str = ":rename_file ";
        ncurses::waddstr(win.win, PROMPT);

        win.move_to_top();
        ncurses::doupdate();

        let user_input: Option<String> = match self.method {
            RenameFileMethod::Append => ui::get_str_append(&win, (0, PROMPT.len() as i32), start_str),
            RenameFileMethod::Prepend => ui::get_str_prepend(&win, (0, PROMPT.len() as i32), start_str),
            RenameFileMethod::Overwrite => ui::get_str(&win, (0, PROMPT.len() as i32)),
            };

        if let Some(s) = user_input {
            let mut new_path = path.parent().unwrap().to_path_buf();
            new_path.push(s);
            match fs::rename(&path, &new_path) {
                Ok(_) => {
                    context.reload_dirlists();

                    ui::redraw_view(&context.views.left_win, context.parent_list.as_ref());
                    ui::redraw_view(&context.views.mid_win, context.curr_list.as_ref());
                    ui::redraw_view(&context.views.right_win, context.preview_list.as_ref());

                    ui::redraw_status(&context.views, context.curr_list.as_ref(),
                            &context.curr_path,
                            &context.config_t.username, &context.config_t.hostname);
                },
                Err(e) => {
                    ui::wprint_err(&context.views.bot_win, e.to_string().as_str());
                },
            }
        }

        win.destroy();
        ncurses::update_panels();
        ncurses::doupdate();
    }
}

impl command::JoshutoCommand for RenameFile {}

impl std::fmt::Display for RenameFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for RenameFile {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let mut path: Option<path::PathBuf> = None;
        let mut file_name: Option<String> = None;

        if let Some(s) = context.curr_list.as_ref() {
            if let Some(s) = s.get_curr_entry() {
                path = Some(s.path.clone());
                file_name = Some(s.file_name_as_string.clone());
            } else {
                return;
            }
        } else {
            return;
        }

        if let Some(file_name) = file_name {
            if let Some(path) = path {
                self.rename_file(&path, context, file_name);
            }
        }
    }
}
