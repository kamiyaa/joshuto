use std::path;

use crate::HOSTNAME;

pub fn set_current_dir(path: &path::Path) -> std::io::Result<()> {
    std::env::set_current_dir(path)?;
    // OSC 7:
    // Escape sequences to advise the terminal of the working directory
    print!(
        "\x1b]7;file://{}{}\x1b\\",
        HOSTNAME.as_str(),
        path.display()
    );
    Ok(())
}
