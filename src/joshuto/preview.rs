extern crate ncurses;
extern crate mime_detective;
extern crate mime;

use std::path;
use std::process;

use joshuto;
use joshuto::ui;
use joshuto::window;

pub fn preview_file(context: &mut joshuto::JoshutoContext)
{
    let curr_tab = &mut context.tabs[context.tab_index];

    if let Some(ref curr_list) = curr_tab.curr_list {
        if let Some(entry) = curr_list.get_curr_entry() {
            if entry.path.is_dir() {
                if let Some(dirlist) = curr_tab.history.get_mut_or_create(&entry.path, &context.config_t.sort_type) {
                    ui::display_contents(&context.config_t, &context.theme_t, &context.views.right_win, dirlist);
                } else {
                    ncurses::werase(context.views.right_win.win);
                    ncurses::waddstr(context.views.right_win.win, "Can't find direntry");
                    ncurses::wnoutrefresh(context.views.right_win.win);
                }
            } else {
                ncurses::werase(context.views.right_win.win);

                if let Some(file_ext) = entry.path.extension() {
                    if let Some(file_ext) = file_ext.to_str() {
                        match file_ext {
                            "o" | "a" | "avi" | "mp3" | "mp4" | "wmv" | "wma" |
                            "mkv" | "flv" | "vob" | "wav" | "mpc" | "flac" |
                            "divx" | "xcf" | "pdf" | "torrent" | "class" | "so" |
                            "img" | "pyc" | "dmg" | "png" | "jpg" | "jpeg" | "out" | "svg" => {
                                ui::wprint_err(&context.views.right_win, "Binary File");
                            },
                            _ => {
                                let detective = mime_detective::MimeDetective::new().unwrap();
                                match detective.detect_filepath(&entry.path) {
                                    Ok(mime_type) => {
                                        match mime_type.type_() {
                                            mime::TEXT => {
                                                text_preview(&context.views.right_win, &entry.path);
                                            },
                                            _ => {
                                                ui::wprint_err(&context.views.right_win, mime_type.type_().as_str());
                                            },
                                        }
                                    },
                                    Err(e) => {
                                        ui::wprint_err(&context.views.right_win, e.to_string().as_str());
                                    },
                                }
                            }
                        }
                    }
                }

                ncurses::wnoutrefresh(context.views.right_win.win);
            }
        } else {
            ncurses::werase(context.views.right_win.win);
            ncurses::wnoutrefresh(context.views.right_win.win);
        }
    }
}

pub fn text_preview(win: &window::JoshutoPanel, path: &path::PathBuf)
{
/*
    let mut command = process::Command::new("bat");
    command.arg("--terminal-width");
    command.arg(win.cols.to_string());
//    command.arg("--wrap=never");
    command.arg("line-range");
    command.arg(format!("{}:{}", 0, win.rows));
    command.arg("--style=numbers");
    command.arg("--tabs");
    command.arg("4");
//    command.arg("--color");
//    command.arg("always");
    command.arg(path.as_os_str());
    command.stdout(process::Stdio::piped());
    // eprintln!("{:?}", command);

*/
    let mut command = process::Command::new("head");
    command.arg("-n");
    command.arg(win.cols.to_string());
    command.arg(path.as_os_str());
    command.stdout(process::Stdio::piped());

    match command.output() {
        Ok(s) => {
            match std::str::from_utf8(&s.stdout) {
                Ok(s) => {
                    let lines = s.split('\n');
                    for (i, line) in lines.enumerate() {
                        ncurses::wmove(win.win, i as i32, 0);
                        ncurses::waddnstr(win.win, line, win.cols);
                    }
                },
                Err(e) => {
                    ncurses::waddstr(win.win, e.to_string().as_str());
                },
            }
        },
        Err(e) => {
            ncurses::waddstr(win.win, e.to_string().as_str());
        }
    }
    // bat joshuto.rs --terminal-width 20 --wrap=never --line-range 0:26 --style='numbers'
}
