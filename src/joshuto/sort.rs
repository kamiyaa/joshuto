use std;
use std::cmp;
use std::fs;

pub fn filter_func_hidden_files(result : Result<fs::DirEntry, std::io::Error>) -> Option<fs::DirEntry>
{
    match result {
        Ok(direntry) => {
            match direntry.file_name().into_string() {
                Ok(file_name) => {
                    if file_name.starts_with(".") {
                        None
                    } else {
                        Some(direntry)
                    }
                },
                Err(_e) => {
                    None
                },
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}


/* sort by directory first, incase-sensitive */
pub fn sort_dir_first(file1 : &fs::DirEntry, file2 : &fs::DirEntry) -> cmp::Ordering
{
    fn res_ordering(file1 : &fs::DirEntry, file2 : &fs::DirEntry) -> Result<cmp::Ordering, std::io::Error> {

        let f1_meta = file1.metadata()?;
        let f2_meta = file2.metadata()?;

        if f1_meta.is_dir() && f2_meta.is_dir() {
            let f1_name : std::string::String =
                file1.file_name().as_os_str().to_str().unwrap().to_lowercase();
            let f2_name : std::string::String =
                file2.file_name().as_os_str().to_str().unwrap().to_lowercase();
            if f1_name <= f2_name {
                Ok(cmp::Ordering::Less)
            } else {
                Ok(cmp::Ordering::Greater)
            }
        } else if f1_meta.is_dir() {
            Ok(cmp::Ordering::Less)
        } else if f2_meta.is_dir() {
            Ok(cmp::Ordering::Greater)
        } else {
            let f1_name : std::string::String =
                file1.file_name().as_os_str().to_str().unwrap().to_lowercase();
            let f2_name : std::string::String =
                file2.file_name().as_os_str().to_str().unwrap().to_lowercase();
            if f1_name <= f2_name {
                Ok(cmp::Ordering::Less)
            } else {
                Ok(cmp::Ordering::Greater)
            }
        }
    }
    res_ordering(file1, file2).unwrap_or(cmp::Ordering::Less)
}
