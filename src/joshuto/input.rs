extern crate ncurses;
extern crate wcwidth;

use std::collections::HashMap;

use joshuto::command;
use joshuto::config::keymap;
use joshuto::window;
use joshuto::config::keymap::*;

pub fn get_str(win: &window::JoshutoPanel,
        coord: (i32, i32)) -> Option<String>
{
    let user_input: Vec<(u8, char)> = Vec::new();
    get_str_prefill(win, coord, user_input, coord.1, 0)
}

pub fn get_str_append(win: &window::JoshutoPanel,
        coord: (i32, i32), start_str: String) -> Option<String>
{
    let mut user_input: Vec<(u8, char)> = Vec::new();
    for (_, ch) in start_str.char_indices() {
        let char_len = wcwidth::char_width(ch).unwrap_or(1);
        user_input.push((char_len, ch));
    }
    let mut curs_x = coord.1;
    for (size, _) in &user_input {
        curs_x = curs_x + (*size) as i32;
    }
    let curr_index = user_input.len();
    get_str_prefill(win, coord, user_input, curs_x, curr_index)
}

pub fn get_str_prepend(win: &window::JoshutoPanel,
        coord: (i32, i32), start_str: String) -> Option<String>
{
    let mut user_input: Vec<(u8, char)> = Vec::new();
    for (_, ch) in start_str.char_indices() {
        let char_len = wcwidth::char_width(ch).unwrap_or(1);
        user_input.push((char_len, ch));
    }
    get_str_prefill(win, coord, user_input, coord.1, 0)
}

pub fn get_str_prefill(win: &window::JoshutoPanel,
        coord: (i32, i32), mut user_input: Vec<(u8, char)>,
        mut curs_x: i32, mut curr_index: usize) -> Option<String>
{
    loop {
        ncurses::wmove(win.win, coord.0, coord.1);
        for (_, ch) in &user_input {
            ncurses::waddstr(win.win, ch.to_string().as_str());
        }
        ncurses::waddstr(win.win, "    ");

        ncurses::mvwchgat(win.win, coord.0, curs_x, 1,
                ncurses::A_STANDOUT(), 0);
        ncurses::wrefresh(win.win);

        let ch: i32 = ncurses::wgetch(win.win);

        if ch == keymap::ESCAPE {
            return None;
        } else if ch == keymap::ENTER {
            break;
        } else if ch == ncurses::KEY_HOME {
            if curr_index != 0 {
                curs_x = coord.1;
                curr_index = 0;
            }
        } else if ch == ncurses::KEY_END {
            let user_input_len = user_input.len();
            if curr_index != user_input_len {
                for i in curr_index..user_input_len {
                    curs_x = curs_x + user_input[i].0 as i32;
                }
                curr_index = user_input_len;
            }
        } else if ch == ncurses::KEY_LEFT {
            if curr_index > 0 {
                curr_index = curr_index - 1;
                curs_x = curs_x - user_input[curr_index].0 as i32;
            }
        } else if ch == ncurses::KEY_RIGHT {
            let user_input_len = user_input.len();
            if curr_index < user_input_len {
                curs_x = curs_x + user_input[curr_index].0 as i32;
                curr_index = curr_index + 1;
            }
        } else if ch == keymap::BACKSPACE {
            let user_input_len = user_input.len();

            if user_input_len == 0 {
                continue;
            }

            if curr_index == user_input_len {
                curr_index = curr_index - 1;
                if let Some((size, _)) = user_input.pop() {
                    curs_x = curs_x - size as i32;
                }
            } else if curr_index > 0 {
                curr_index = curr_index - 1;
                let (size, _) = user_input.remove(curr_index);
                curs_x = curs_x - size as i32;
            }
        } else if ch == ncurses::KEY_DC {
            let user_input_len = user_input.len();

            if user_input_len == 0 || curr_index == user_input_len {
                continue;
            }

            if curr_index > 0 {
                let (size, _) = user_input.remove(curr_index);
                if curr_index > user_input_len {
                    curr_index = curr_index - 1;
                    curs_x = curs_x - size as i32;
                }
            } else if curr_index == 0 {
                user_input.remove(curr_index);
            }
        } else {
            let user_input_len = user_input.len();

            let ch = ch as u8 as char;
            let char_len = wcwidth::char_width(ch).unwrap_or(1);
            let size_ch = (char_len, ch);

            if curr_index == user_input_len {
                user_input.push(size_ch);
            } else {
                user_input.insert(curr_index, size_ch);
            }
            curs_x = curs_x + user_input[curr_index].0 as i32;
            curr_index = curr_index + 1;
        }
    }
    let user_str: String = user_input.iter().map(|(_, ch)| ch).collect();

    return Some(user_str);
}


pub fn initialize_default_keymap() -> HashMap<i32, command::CommandKeybind>
{
    let mut keymaps: HashMap<i32, command::CommandKeybind> = HashMap::new();

    // quit
    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::Quit::new()));
    keymaps.insert('q' as i32, command);

    // up
    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::CursorMove::new(-1)));
    keymaps.insert(ncurses::KEY_UP, command);
    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::CursorMove::new(-1)));
    keymaps.insert('k' as i32, command);

    // down
    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::CursorMove::new(1)));
    keymaps.insert(ncurses::KEY_DOWN, command);
    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::CursorMove::new(1)));
    keymaps.insert('j' as i32, command);

    // left
    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::ParentDirectory::new()));
    keymaps.insert(ncurses::KEY_LEFT, command);
    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::ParentDirectory::new()));
    keymaps.insert('h' as i32, command);

    // right
    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::OpenFile::new()));
    keymaps.insert(ncurses::KEY_RIGHT, command);
    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::OpenFile::new()));
    keymaps.insert('l' as i32, command);
    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::OpenFile::new()));
    keymaps.insert(ENTER, command);

    let command = command::CommandKeybind::SimpleKeybind(
        Box::new(command::OpenFileWith::new()));
    keymaps.insert('r' as i32, command);

    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::CursorMovePageUp::new()));
    keymaps.insert(ncurses::KEY_PPAGE, command);

    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::CursorMovePageDown::new()));
    keymaps.insert(ncurses::KEY_NPAGE, command);

    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::CursorMoveHome::new()));
    keymaps.insert(ncurses::KEY_HOME, command);

    let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::CursorMoveEnd::new()));
    keymaps.insert(ncurses::KEY_END, command);

    let command = command::CommandKeybind::SimpleKeybind(
        Box::new(command::DeleteFiles::new()));
    keymaps.insert(ncurses::KEY_DC, command);

    let command = command::CommandKeybind::SimpleKeybind(
        Box::new(command::RenameFile::new(command::RenameFileMethod::Append)));
    keymaps.insert('a' as i32, command);

    {
        let mut subkeymap: HashMap<i32, command::CommandKeybind> = HashMap::new();
        let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::ToggleHiddenFiles::new()));
        subkeymap.insert('h' as i32, command);

        let command = command::CommandKeybind::CompositeKeybind(subkeymap);
        keymaps.insert('z' as i32, command);
    }

    {
        let mut subkeymap: HashMap<i32, command::CommandKeybind> = HashMap::new();
        let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::CutFiles::new()));
        subkeymap.insert('d' as i32, command);

        let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::DeleteFiles::new()));
        subkeymap.insert('D' as i32, command);

        let command = command::CommandKeybind::CompositeKeybind(subkeymap);
        keymaps.insert('d' as i32, command);
    }

    {
        let mut subkeymap: HashMap<i32, command::CommandKeybind> = HashMap::new();

        let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::CopyFiles::new()));
        subkeymap.insert('y' as i32, command);

        let command = command::CommandKeybind::CompositeKeybind(subkeymap);
        keymaps.insert('y' as i32, command);
    }

    {
        let mut subkeymap: HashMap<i32, command::CommandKeybind> = HashMap::new();

        let options = fs_extra::dir::CopyOptions::new();
        let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::PasteFiles::new(options)));
        subkeymap.insert('p' as i32, command);

        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::PasteFiles::new(options)));
        subkeymap.insert('o' as i32, command);

        let command = command::CommandKeybind::CompositeKeybind(subkeymap);
        keymaps.insert('p' as i32, command);
    }

    {
        let mut subkeymap: HashMap<i32, command::CommandKeybind> = HashMap::new();
        let command = command::CommandKeybind::SimpleKeybind(
            Box::new(command::NewDirectory::new()));
        subkeymap.insert('k' as i32, command);

        let command = command::CommandKeybind::CompositeKeybind(subkeymap);
        keymaps.insert('m' as i32, command);
    }

    keymaps
}
