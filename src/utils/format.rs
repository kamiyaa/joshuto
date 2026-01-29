use std::time;

pub fn file_size_to_string(file_size: u64) -> String {
    const FILE_UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "P"];
    const CONV_RATE: f64 = 1024.0;
    let mut file_size: f64 = file_size as f64;

    let mut index = 0;
    while file_size > CONV_RATE {
        file_size /= CONV_RATE;
        index += 1;
    }

    if file_size >= 100.0 {
        format!("{:>4.0} {}", file_size, FILE_UNITS[index])
    } else if file_size >= 10.0 {
        format!("{:>4.1} {}", file_size, FILE_UNITS[index])
    } else {
        format!("{:>4.2} {}", file_size, FILE_UNITS[index])
    }
}

pub fn time_to_string(time: time::SystemTime) -> String {
    const TIME_FORMATTING: &str = "%Y-%m-%d %H:%M";

    let datetime: chrono::DateTime<chrono::offset::Local> = time.into();
    datetime.format(TIME_FORMATTING).to_string()
}

pub fn clear_screen() {
    println!("{}", ratatui::termion::clear::All);
}
