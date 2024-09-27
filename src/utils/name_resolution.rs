use std::path;

pub fn rename_filename_conflict(path: &mut path::PathBuf) {
    let file_name = path.file_name().unwrap().to_os_string();
    for i in 0.. {
        if !path.exists() {
            break;
        }
        path.pop();

        let mut file_name = file_name.clone();
        file_name.push(format!("_{i}"));
        path.push(file_name);
    }
}
