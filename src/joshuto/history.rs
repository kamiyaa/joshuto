
use std;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::env;
use std::fs;
use std::path;
use std::process;

use joshuto::structs;

pub fn get_or_create(map : &mut HashMap<String, structs::JoshutoColumn>,
        path : &path::Path,
        sort_func : fn (&structs::JoshutoDirEntry, &structs::JoshutoDirEntry) -> std::cmp::Ordering,
        show_hidden : bool) -> Result<structs::JoshutoColumn, std::io::Error>
{
    let key = format!("{}", path.to_str().unwrap());
//  eprintln!("Looking for {} in map...", key);
    match map.entry(key) {
        Entry::Occupied(entry) => {
            let tmp = entry.remove_entry();

            let metadata = fs::metadata(&path)?;
            let mut dir_entry = tmp.1;
            let modified = metadata.modified()?;
            if modified > dir_entry.modified {
                dir_entry.modified = modified;
                dir_entry.need_update = true;
            }
            if dir_entry.need_update {
                dir_entry.update(&path, sort_func, show_hidden);
            }
            Ok(dir_entry)
        },
        Entry::Vacant(_entry) => {
//            eprintln!("did not find value, creating new one...");
            structs::JoshutoColumn::new(&path, sort_func, show_hidden)
        }
    }
}

pub fn depecrate_all_entries(map : &mut HashMap<String, structs::JoshutoColumn>)
{
    for (_, direntry) in map.iter_mut() {
        direntry.need_update = true;
    }

}

pub fn init_path_history(
        sort_func : fn (&structs::JoshutoDirEntry, &structs::JoshutoDirEntry) -> std::cmp::Ordering,
        show_hidden : bool) -> HashMap<String, structs::JoshutoColumn>
{
    match env::current_dir() {
        Ok(mut pathbuf) => {
            let mut history : HashMap<String, structs::JoshutoColumn>
                    = HashMap::new();
            while pathbuf.parent() != None {
                match structs::JoshutoColumn::new(pathbuf.parent().unwrap(), sort_func, show_hidden) {
                    Ok(mut s) => {
                        let parent = pathbuf.parent().unwrap();
                        let parent_str = format!("{}", parent.to_str().unwrap());
                        for (i, dirent) in s.contents.as_ref().unwrap().iter().enumerate() {
                            if dirent.entry.path() == pathbuf {
                                s.index = i;
                                break;
                            }
                        }

                        history.insert(parent_str, s);
                    },
                    Err(e) => { eprintln!("{}", e); }
                };
                if pathbuf.pop() == false {
                    break;
                }
            }
            history
        },
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
