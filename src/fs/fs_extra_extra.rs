use std::path;

fn rename_filename_conflict(mut path: path::PathBuf) -> path::PathBuf {
    let file_name = path.file_name().unwrap().to_os_string();
    for i in 0.. {
        if !path.exists() {
            break;
        }
        path.pop();

        let mut file_name = file_name.clone();
        file_name.push(&format!("_{}", i));
        path.push(file_name);
    }
    path
}

pub fn fs_copy_with_progress<P, Q, F>(
    paths: &[P],
    to: Q,
    mut options: fs_extra::dir::CopyOptions,
    mut progress_handler: F,
) -> std::io::Result<u64>
where
    P: AsRef<path::Path>,
    Q: AsRef<path::Path>,
    F: FnMut(fs_extra::TransitProcess) -> fs_extra::dir::TransitProcessResult,
{
    let total_size = {
        let mut sum = 0;
        for item in paths {
            sum += match fs_extra::dir::get_size(item) {
                Ok(s) => s,
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{}", e),
                    ));
                }
            }
        }
        sum
    };

    let mut info_process = fs_extra::TransitProcess {
        copied_bytes: 0,
        total_bytes: total_size,
        file_bytes_copied: 0,
        file_total_bytes: 0,
        file_name: String::new(),
        dir_name: String::new(),
        state: fs_extra::dir::TransitState::Normal,
    };

    let mut result: u64 = 0;

    let mut file_options = fs_extra::file::CopyOptions::new();
    file_options.overwrite = options.overwrite;
    file_options.skip_exist = options.skip_exist;
    file_options.buffer_size = options.buffer_size;

    let dir_options = options.clone();

    let mut destination = to.as_ref().clone().to_path_buf();
    for path in paths {
        let file_name = path.as_ref().file_name().unwrap().to_os_string();
        destination.push(file_name.clone());
        if !options.skip_exist {
            destination = rename_filename_conflict(destination);
        }

        if path.as_ref().is_dir() {
            /* create the destination dir */
            std::fs::create_dir(&destination)?;

            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    let dir_handler = |info: fs_extra::dir::TransitProcess| {
                        info_process.copied_bytes = result + info.copied_bytes;
                        info_process.state = info.state;
                        info_process.file_name = info.file_name;
                        let result = progress_handler(info_process.clone());
                        match result {
                            fs_extra::dir::TransitProcessResult::OverwriteAll => {
                                options.overwrite = true
                            }
                            fs_extra::dir::TransitProcessResult::SkipAll => {
                                options.skip_exist = true
                            }
                            _ => {}
                        }
                        result
                    };
                    match fs_extra::dir::copy_with_progress(
                        &entry_path,
                        &destination,
                        &dir_options,
                        dir_handler,
                    ) {
                        Ok(s) => result += s,
                        Err(e) => {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("{}", e),
                            ));
                        }
                    }
                } else {
                    let file_name = entry.file_name();
                    destination.push(file_name.clone());
                    let file_handler = |info: fs_extra::file::TransitProcess| {
                        info_process.copied_bytes = result + info.copied_bytes;
                        info_process.file_bytes_copied = info.copied_bytes;
                        progress_handler(info_process.clone());
                    };
                    match fs_extra::file::copy_with_progress(
                        &entry_path,
                        &destination,
                        &file_options,
                        file_handler,
                    ) {
                        Ok(s) => result += s,
                        Err(e) => {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("{}", e),
                            ));
                        }
                    }
                    destination.pop();
                }
            }
        } else {
            let file_handler = |info: fs_extra::file::TransitProcess| {
                info_process.copied_bytes = result + info.copied_bytes;
                info_process.file_bytes_copied = info.copied_bytes;
                progress_handler(info_process.clone());
            };

            match fs_extra::file::copy_with_progress(
                path,
                &destination,
                &file_options,
                file_handler,
            ) {
                Ok(s) => result += s,
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{}", e),
                    ));
                }
            }
        }
        destination.pop();
    }
    Ok(result)
}

pub fn fs_cut_with_progress<P, Q, F>(
    paths: &[P],
    to: Q,
    mut options: fs_extra::dir::CopyOptions,
    mut progress_handler: F,
) -> std::io::Result<u64>
where
    P: AsRef<path::Path>,
    Q: AsRef<path::Path>,
    F: FnMut(fs_extra::TransitProcess) -> fs_extra::dir::TransitProcessResult,
{
    let total_size = {
        let mut sum = 0;
        for item in paths {
            sum += match fs_extra::dir::get_size(item) {
                Ok(s) => s,
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{}", e),
                    ));
                }
            }
        }
        sum
    };

    let mut info_process = fs_extra::TransitProcess {
        copied_bytes: 0,
        total_bytes: total_size,
        file_bytes_copied: 0,
        file_total_bytes: 0,
        file_name: String::new(),
        dir_name: String::new(),
        state: fs_extra::dir::TransitState::Normal,
    };

    let mut result: u64 = 0;
    #[cfg(target_os = "linux")]
    use std::os::linux::fs::MetadataExt;
    let to_ino = to.as_ref().metadata()?.st_dev();

    let mut destination = to.as_ref().clone().to_path_buf();

    let mut file_options = fs_extra::file::CopyOptions::new();
    file_options.overwrite = options.overwrite;
    file_options.skip_exist = options.skip_exist;
    file_options.buffer_size = options.buffer_size;

    let dir_options = options.clone();

    for path in paths {
        let file_name = path.as_ref().file_name().unwrap().to_os_string();
        destination.push(file_name.clone());
        if !options.skip_exist {
            destination = rename_filename_conflict(destination);
        }
        #[cfg(target_os = "linux")]
        {
            let path_ino = path.as_ref().metadata()?.st_dev();
            /* on the same fs, can do a rename */
            if path_ino == to_ino {
                std::fs::rename(&path, &destination)?;
                result += fs_extra::dir::get_size(&destination).unwrap();
                info_process.copied_bytes = result;

                destination.pop();
                continue;
            }
        }

        if path.as_ref().is_dir() {
            /* create the destination dir */
            std::fs::create_dir(&destination)?;

            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    let dir_handler = |info: fs_extra::dir::TransitProcess| {
                        info_process.copied_bytes = result + info.copied_bytes;
                        info_process.state = info.state;
                        let result = progress_handler(info_process.clone());
                        match result {
                            fs_extra::dir::TransitProcessResult::OverwriteAll => {
                                options.overwrite = true
                            }
                            fs_extra::dir::TransitProcessResult::SkipAll => {
                                options.skip_exist = true
                            }
                            _ => {}
                        }
                        result
                    };
                    match fs_extra::dir::move_dir_with_progress(
                        &entry_path,
                        &destination,
                        &dir_options,
                        dir_handler,
                    ) {
                        Ok(s) => result += s,
                        Err(e) => {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("{}", e),
                            ));
                        }
                    }
                } else {
                    let file_name = entry.file_name();
                    destination.push(file_name.clone());
                    let file_handler = |info: fs_extra::file::TransitProcess| {
                        info_process.copied_bytes = result + info.copied_bytes;
                        info_process.file_bytes_copied = info.copied_bytes;
                        progress_handler(info_process.clone());
                    };
                    match fs_extra::file::move_file_with_progress(
                        &entry_path,
                        &destination,
                        &file_options,
                        file_handler,
                    ) {
                        Ok(s) => result += s,
                        Err(e) => {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("{}", e),
                            ));
                        }
                    }
                    destination.pop();
                }
            }
            /* remove the source dir */
            std::fs::remove_dir_all(path)?;
        } else {
            let handler = |info: fs_extra::file::TransitProcess| {
                info_process.copied_bytes = result + info.copied_bytes;
                info_process.file_bytes_copied = info.copied_bytes;
                progress_handler(info_process.clone());
            };

            match fs_extra::file::move_file_with_progress(
                path,
                &destination,
                &file_options,
                handler,
            ) {
                Ok(s) => result += s,
                Err(e) => {
                    eprintln!("{}", e);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{}", e),
                    ));
                }
            }
        }
        destination.pop();
    }
    Ok(result)
}
