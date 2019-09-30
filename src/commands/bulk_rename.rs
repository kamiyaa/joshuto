use std::io::{BufRead, Write};
use std::path;
use std::process;

use rand::Rng;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::window::JoshutoView;

#[derive(Clone, Debug)]
pub struct BulkRename;

impl BulkRename {
    pub fn new() -> Self {
        BulkRename {}
    }
    pub const fn command() -> &'static str {
        "bulk_rename"
    }

    pub fn bulk_rename(context: &mut JoshutoContext) -> JoshutoResult<()> {
        const PREFIX: &str = "joshuto-";
        let editor = match std::env::var("EDITOR") {
            Ok(s) => s,
            Err(_) => {
                return Err(JoshutoError::new(
                    JoshutoErrorKind::EnvVarNotPresent,
                    String::from("EDITOR environment variable not set"),
                ));
            }
        };

        /* generate a random file name to write to */
        let mut rand_str = String::with_capacity(PREFIX.len() + 10);
        rand_str.push_str(PREFIX);
        rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(10)
            .for_each(|ch| rand_str.push(ch));

        /* create this file in a temporary folder */
        let mut file_path = path::PathBuf::from("/tmp");
        file_path.push(rand_str);

        let curr_tab = &context.tabs[context.curr_tab_index];
        let paths = curr_tab.curr_list.get_selected_paths();
        {
            let mut file = std::fs::File::create(&file_path)?;
            for path in &paths {
                let file_name = path.file_name().unwrap();
                let file_name_as_bytes = file_name.to_str().unwrap().as_bytes();
                file.write(file_name_as_bytes)?;
                file.write(&['\n' as u8])?;
            }
        }

        let mut command = process::Command::new(editor);
        command.arg(&file_path);

        let time = std::time::SystemTime::now();
        /* exit curses and launch program */
        {
            ncurses::savetty();
            ncurses::endwin();
            let mut handle = command.spawn()?;
            handle.wait()?;
        }
        let metadata = std::fs::metadata(&file_path)?;
        if time >= metadata.modified()? {
            return Ok(());
        }

        let mut paths_renamed: Vec<path::PathBuf> = Vec::with_capacity(paths.len());
        {
            let file = std::fs::File::open(&file_path)?;

            let reader = std::io::BufReader::new(file);
            for line in reader.lines() {
                let line2 = line?;
                let line = line2.trim();
                if line.len() == 0 {
                    continue;
                }
                let path = path::PathBuf::from(line);
                paths_renamed.push(path);
            }
        }
        if paths_renamed.len() < paths.len() {
            return Err(JoshutoError::new(
                JoshutoErrorKind::IOInvalidInput,
                String::from("Not enough input given"),
            ));
        }

        for (p, q) in paths.iter().zip(paths_renamed.iter()) {
            println!("{:?} -> {:?}", p, q);
        }
        print!("Continue with rename? (Y/n): ");
        std::io::stdout().flush()?;

        let mut user_input = String::with_capacity(4);
        std::io::stdin().read_line(&mut user_input)?;
        user_input = user_input.to_lowercase();

        let user_input_trimmed = user_input.trim();
        if user_input_trimmed != "n" || user_input_trimmed != "no" {
            for (p, q) in paths.iter().zip(paths_renamed.iter()) {
                let mut command = process::Command::new("mv");
                command.arg("-iv");
                command.arg("--");
                command.arg(p);
                command.arg(q);
                let mut handle = command.spawn()?;
                handle.wait()?;
            }
        }
        print!("Press ENTER to continue");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut user_input)?;

        std::fs::remove_file(file_path)?;

        /* restore ncurses */
        ncurses::resetty();
        ncurses::refresh();
        ncurses::doupdate();
        Ok(())
    }
}

impl JoshutoCommand for BulkRename {}

impl std::fmt::Display for BulkRename {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl JoshutoRunnable for BulkRename {
    fn execute(&self, context: &mut JoshutoContext, view: &JoshutoView) -> JoshutoResult<()> {
        Self::bulk_rename(context)?;
        ReloadDirList::reload(context.curr_tab_index, context)?;
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        curr_tab.refresh(view, &context.config_t);
        ncurses::doupdate();
        Ok(())
    }
}
