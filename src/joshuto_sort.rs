use std;
use std::cmp;
use std::fs;

pub fn alpha_sort(file1 : &fs::DirEntry, file2 : &fs::DirEntry) -> cmp::Ordering
{
    fn res_ordering(file1 : &fs::DirEntry, file2 : &fs::DirEntry) -> Result<cmp::Ordering, std::io::Error> {
        let f1_type = file1.file_type()?;
        let f2_type = file2.file_type()?;

        if !f1_type.is_file() && f2_type.is_file() {
            Ok(cmp::Ordering::Less)
        } else if !f2_type.is_file() && f1_type.is_file() {
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
