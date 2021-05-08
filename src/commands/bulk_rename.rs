use std::io::{self, BufRead, Write};
use std::path;
use std::process;

use rand::Rng;

use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::TuiBackend;

use super::reload;

const ENV_EDITOR: &str = "EDITOR";

pub fn _bulk_rename(context: &mut AppContext) -> JoshutoResult<()> {
    const PREFIX: &str = "joshuto-";
    let editor = std::env::var(ENV_EDITOR)?;

    /* generate a random file name to write to */
    let mut rand_str = String::with_capacity(PREFIX.len() + 10);
    rand_str.push_str(PREFIX);
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(10)
        .for_each(|ch| rand_str.push(ch as char));

    /* create this file in a temporary folder */
    let mut file_path = path::PathBuf::from("/tmp");
    file_path.push(rand_str);

    let paths = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map_or(vec![], |s| s.get_selected_paths());

    {
        let mut file = std::fs::File::create(&file_path)?;
        for path in paths.iter() {
            let file_name = path.file_name().unwrap();
            let file_name_as_bytes = file_name.to_str().unwrap().as_bytes();
            file.write_all(file_name_as_bytes)?;
            file.write_all(&[b'\n'])?;
        }
    }

    let mut command = process::Command::new(editor);
    command.arg(&file_path);

    let last_modified = std::fs::metadata(&file_path)?.modified()?;
    {
        let mut handle = command.spawn()?;
        handle.wait()?;
    }
    // check if the file was modified since it was created
    let metadata = std::fs::metadata(&file_path)?;
    if metadata.modified()? <= last_modified {
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
            JoshutoErrorKind::Io(io::ErrorKind::InvalidInput),
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

pub fn bulk_rename(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    backend.terminal_drop();
    let res = _bulk_rename(context);
    backend.terminal_restore()?;
    reload::reload(context, context.tab_context_ref().index)?;
    res
}
