use std::time;

use tui::buffer::Buffer;
use tui::style::Style;

use super::unix;

pub fn file_size_to_string(file_size: u64) -> String {
    const FILE_UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "E"];
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

pub fn mode_to_string(mode: u32) -> String {
    unix::stringify_mode(mode)
}

pub fn mtime_to_string(mtime: time::SystemTime) -> String {
    const MTIME_FORMATTING: &str = "%Y-%m-%d %H:%M";

    let datetime: chrono::DateTime<chrono::offset::Utc> = mtime.into();
    datetime.format(MTIME_FORMATTING).to_string()
}

pub fn write_str_to_buf(buf: &mut Buffer, s: &str, width: usize, x: u16, y: u16, style: Style) {}
