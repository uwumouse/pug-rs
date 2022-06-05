use std::{ffi::OsStr, path::PathBuf};
use colored::{ColoredString, Colorize};

pub fn is_pug_file(path: &PathBuf) -> bool {
    path.extension().and_then(OsStr::to_str).unwrap_or("") == "pug"
}

pub fn gray_text(t: &str) -> ColoredString {
    t.truecolor(187, 196, 189)
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
