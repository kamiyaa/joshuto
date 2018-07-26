
use std;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs;
use std::path;

use joshuto::structs;

pub fn get_or_create(map : &mut HashMap<String, structs::JoshutoDirEntry>,
        path : &path::Path,
        sort_func : fn (&fs::DirEntry, &fs::DirEntry) -> std::cmp::Ordering,
        show_hidden : bool) -> Result<structs::JoshutoDirEntry, std::io::Error>
{
    let key = format!("{}", path.to_str().unwrap());
//  eprintln!("Looking for {} in map...", key);
    match map.entry(key) {
        Entry::Occupied(entry) => {
            Ok(entry.remove_entry().1)
        },
        Entry::Vacant(_entry) => {
//            eprintln!("did not find value, creating new one...");
            structs::JoshutoDirEntry::new(&path, sort_func, show_hidden)
        }
    }
}
