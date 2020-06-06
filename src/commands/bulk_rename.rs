use std::io::{BufRead, Write};
use std::path;
use std::process;

use rand::Rng;

use crate::commands::{JoshutoCommand, JoshutoRunnable, ReloadDirList};
use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::TuiBackend;

const ENV_EDITOR: &str = "EDITOR";

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
        let editor = match std::env::var(ENV_EDITOR) {
            Ok(s) => s,
            Err(_) => {
                return Err(JoshutoError::new(
                    JoshutoErrorKind::EnvVarNotPresent,
                    format!("{} environment variable not set", ENV_EDITOR),
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
        let paths = match curr_tab.curr_list_ref() {
            Some(s) => s.get_selected_paths(),
            None => Vec::new(),
        };
        {
            let mut file = std::fs::File::create(&file_path)?;
            for path in &paths {
                let file_name = path.file_name().unwrap();
                let file_name_as_bytes = file_name.to_str().unwrap().as_bytes();
                file.write_all(file_name_as_bytes)?;
                file.write_all(&[b'\n'])?;
            }
        }

        let mut command = process::Command::new(editor);
        command.arg(&file_path);

        let time = std::time::SystemTime::now();
        {
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
                if line.is_empty() {
                    continue;
                }
                let path = path::PathBuf::from(line);
                paths_renamed.push(path);
            }
            std::fs::remove_file(&file_path)?;
        }
        if paths_renamed.len() < paths.len() {
            return Err(JoshutoError::new(
                JoshutoErrorKind::IOInvalidInput,
                "Insufficient inputs".to_string(),
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
                let mut handle = process::Command::new("mv")
                    .arg("-iv")
                    .arg("--")
                    .arg(p)
                    .arg(q)
                    .spawn()?;
                handle.wait()?;
            }
        }
        print!("Press ENTER to continue...");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut user_input)?;

        std::fs::remove_file(file_path)?;
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
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        backend.terminal_drop();
        let res = Self::bulk_rename(context);
        backend.terminal_restore()?;
        ReloadDirList::reload(context.curr_tab_index, context)?;
        res
    }
}
