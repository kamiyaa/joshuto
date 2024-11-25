use std::io::{self, Write};

pub fn set_title(title: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let full_title = format!("{}{}", title, " - joshuto");
    write!(stdout, "\x1b]0;{}\x07", full_title)?;
    stdout.flush()?;
    Ok(())
}