use std::process::Command;
use crate::ui::TuiBackend;
use crate::ui::views::TuiTextField;
use crate::ui::views::TuiBookmarkMenu;
use termion::event::Event;
use termion::event::Key;
use crate::context::{AppContext};
use crate::config;
use crate::config::{AppBookmarkMapping,};
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};


fn notify<T: std::fmt::Debug>(x: T){
    let log = format!("{:?}", x);
    let _  = std::process::Command::new("notify-send").arg(log).status();
}
pub fn oldadd_bookmark(context: &mut AppContext, c: char) -> JoshutoResult<()> {
    notify(c);


    let opt_entry = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|dirlist| dirlist.file_path());

    if let Some(pathbuf) = opt_entry {
        if let Some(dir) = pathbuf.to_str().map(|s| String::from(s)) {
        let path = std::path::PathBuf::from(dir);
        let event = Event::Key(Key::Char(c));
        let bookmarks = &mut context.bookmarks;
        config::bookmarks::insert_bookmark(bookmarks, path, event); 


        }

    };
    Ok(())

}



pub fn add_bookmark(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {


    const PROMPT: &str = "add bookmark";

    // let user_input: Option<String> = {
    //     context.flush_event();
    //     let menu_lines = vec!["".to_string()];

    //     TuiTextField::default()
    //         .prompt(format!("-->{}-->", PROMPT).as_str())
    //         .menu_items(menu_lines.iter().map(|s| s.as_str()));
    //         // .get_input(backend, context)
    //     Some("dummy".to_string())
    // };
    // notify(&user_input);
    //


    let mut tbm = TuiBookmarkMenu::new(context);
    match tbm.get_any_char(backend, context){

        Some(Event::Key(Key::Char(c))) => {


            let opt_entry = context
                .tab_context_ref()
                .curr_tab_ref()
                .curr_list_ref()
                .map(|dirlist| dirlist.file_path());

            if let Some(pathbuf) = opt_entry {
                if let Some(dir) = pathbuf.to_str().map(|s| String::from(s)) {
                let path = std::path::PathBuf::from(dir);
                let event = Event::Key(Key::Char(c));
                let bookmarks = &mut context.bookmarks;
                config::bookmarks::insert_bookmark(bookmarks, path, event); 



                notify(c);
                return Ok(())
                }
            }
        },
        // _ => return Err("asdfasdfsaf".to_string())
        _ => return Ok(()) 
    }



/*
    if let Some(line) = user_input{ 
        let c = line.chars().next().unwrap();


        let opt_entry = context
            .tab_context_ref()
            .curr_tab_ref()
            .curr_list_ref()
            .map(|dirlist| dirlist.file_path());

        if let Some(pathbuf) = opt_entry {
            if let Some(dir) = pathbuf.to_str().map(|s| String::from(s)) {
            let path = std::path::PathBuf::from(dir);
            let event = Event::Key(Key::Char(c));
            let bookmarks = &mut context.bookmarks;
            config::bookmarks::insert_bookmark(bookmarks, path, event); 


            }

        };
    };
*/
    Ok(())

}




