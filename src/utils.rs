use std::fs;
use std::io::Write;
use std::result::Result::Err;
use termion::color;

/// # Panics
///
/// Can panic if the file can't be written to
pub fn log(s: &str) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("bo.log")
        .unwrap();
    if let Err(e) = writeln!(file, "{}", s) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn zfill(s: &str, fill_by: &str, size: usize) -> String {
    if size == 0 {
        return "".to_string();
    }
    format!("{}{}", fill_by.repeat(size - s.len()), s)
}

pub fn red(s: &str) -> String {
    format!("{}{}{}", color::Fg(color::Red), s, color::Fg(color::Reset))
}
