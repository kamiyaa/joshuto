use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path;
use std::process;

use rand::Rng;

use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::ui::AppBackend;
use crate::util::process::wait_for_enter;

use super::reload;

const ENV_TMP_DIR: &str = "TMP_DIR";
const ENV_EDITOR: &str = "EDITOR";
const FILE_PREFIX: &str = "joshuto-";
const RAND_STR_LEN: usize = 10;

pub fn _bulk_rename(context: &mut AppContext) -> JoshutoResult {
    let tmp_directory = env::var(ENV_TMP_DIR).unwrap_or_else(|_| "/tmp".to_string());

    let editor = std::env::var(ENV_EDITOR)?;

    /* generate a random file name to write to */
    let mut rand_str = String::with_capacity(FILE_PREFIX.len() + RAND_STR_LEN);
    rand_str.push_str(FILE_PREFIX);
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(RAND_STR_LEN)
        .for_each(|ch| rand_str.push(ch as char));

    /* create this file in a temporary folder */
    let mut file_path = path::PathBuf::from(&tmp_directory);
    file_path.push(rand_str);

    let entries = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map_or(vec![], |s| s.selected_or_current());

    /* write file names into temporary file to edit */
    {
        let mut file = fs::File::create(&file_path)?;
        for path in entries.iter() {
            let file_name = path.file_name();
            let file_name_as_bytes = file_name.as_bytes();
            file.write_all(file_name_as_bytes)?;
            file.write_all(&[b'\n'])?;
        }
    }

    /* open file with text editor to edit */
    {
        let initial_modified = fs::metadata(&file_path)?.modified()?;

        process::Command::new(editor)
            .arg(&file_path)
            .spawn()?
            .wait()?;

        // check if the file was modified since it was created
        let last_modified = fs::metadata(&file_path)?.modified()?;
        if last_modified <= initial_modified {
            // remember to remove file
            std::fs::remove_file(&file_path)?;
            return Ok(());
        }
    }

    let mut paths_renamed: Vec<path::PathBuf> = Vec::with_capacity(entries.len());
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
    if paths_renamed.len() < entries.len() {
        return Err(JoshutoError::new(
            JoshutoErrorKind::Io(io::ErrorKind::InvalidInput),
            "Insufficient inputs".to_string(),
        ));
    }

    println!("{}", termion::clear::All);
    for (p, q) in entries.iter().zip(paths_renamed.iter()) {
        println!("{:?} -> {:?}", p.file_name(), q);
    }
    print!("Continue with rename? (Y/n): ");
    std::io::stdout().flush()?;

    let mut user_input = String::with_capacity(4);
    std::io::stdin().read_line(&mut user_input)?;

    let user_input_fmt = user_input.trim().to_lowercase();
    match user_input_fmt.as_str() {
        "" | "y" | "yes" => {
            for (p, q) in entries.iter().zip(paths_renamed.iter()) {
                let mut handle = process::Command::new("mv")
                    .arg("-iv")
                    .arg("--")
                    .arg(p.file_name())
                    .arg(q)
                    .spawn()?;
                handle.wait()?;
            }
        }
        _ => {}
    }
    wait_for_enter()?;

    std::fs::remove_file(file_path)?;
    Ok(())
}

pub fn bulk_rename(context: &mut AppContext, backend: &mut AppBackend) -> JoshutoResult {
    context.remove_external_preview();
    backend.terminal_drop();
    let res = _bulk_rename(context);
    backend.terminal_restore()?;
    reload::soft_reload_curr_tab(context)?;
    res
}
