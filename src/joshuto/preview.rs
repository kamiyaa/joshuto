extern crate ncurses;

use joshuto;
use joshuto::ui;

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
                ncurses::waddstr(context.views.right_win.win, "Not a directory");
                ncurses::wnoutrefresh(context.views.right_win.win);
            }
        }
    }
}
