extern crate mime;
extern crate mime_detective;
extern crate ncurses;

use std::path::PathBuf;
use std::process;

use joshuto::context::JoshutoContext;
use joshuto::ui;
use joshuto::window;

pub fn preview_file(context: &mut JoshutoContext) {
    /*
        let curr_tab = &mut context.tabs[context.tab_index];

        if let Some(ref curr_list) = curr_tab.curr_list {
            if let Some(entry) = curr_list.get_curr_ref() {
                if entry.path.is_dir() {
                    if let Some(dirlist) = curr_tab.history.get_mut_or_create(&entry.path, &context.config_t.sort_type) {
                        context.views.right_win.display_contents(&context.theme_t, dirlist, context.config_t.scroll_offset);
                        context.views.right_win.queue_for_refresh();
                    } else {
                        ncurses::werase(context.views.right_win.win);
                        ncurses::waddstr(context.views.right_win.win, "Can't find direntry");
                        context.views.right_win.queue_for_refresh();
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
    */
}

pub fn text_preview(win: &window::JoshutoPanel, path: &PathBuf) {
    let mut command = process::Command::new("head");
    command.arg("-n");
    command.arg(win.cols.to_string());
    command.arg(path.as_os_str());
    command.stdin(std::process::Stdio::piped());
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

    match command.spawn() {
        Ok(child) => {
            if let Some(output) = child.stdout {
                let mut reader = std::io::BufReader::new(output);
                let mut buffer = String::new();

                // reader.read_line(&mut buffer);
            }
        }
        Err(e) => {
            ncurses::waddstr(win.win, e.to_string().as_str());
        }
    }
    // bat joshuto.rs --terminal-width 20 --wrap=never --line-range 0:26 --style='numbers'
}
