use crate::Position;

use std::io::{self, Write, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
pub height: u16,
pub width: u16,
}

pub struct Terminal {
size: Size,
_stdout: RawTerminal<std::io::Stdout>
}


impl Terminal {
/// # Errors
///
/// will return an error if the terminal size can't be acquired
/// or if the stdout cannot be put into raw mode. 
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        println!("{:?}", size);
        Ok(
            Self {
                size: Size {
                    height: size.1,
                    width: size.0,
                },
                _stdout: stdout().into_raw_mode()?,
            }
        )
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    /// # Errors
    /// 
    /// Returns an error if stdout can't be flushed
    pub fn flush() -> Result<(), std::io::Error> {
        std::io::stdout().flush()
    }

    /// # Errors
    /// 
    /// Returns an error if a key can't be read
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            let opt_key = io::stdin().lock().keys().next();
            // at that point, key is a Result<Key, Error>, as the Option was unwrapped
            if let Some(key) = opt_key {
                return key
            }
        }
    }

    pub fn set_cursor_position(position: &Position) {
        let Position{mut x, mut y} = position;
        // hiding the fact that the terminal position is 1-based, while preventing an overflow
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x as u16, y as u16));
    }

    pub fn hide_cursor() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn show_cursor() {
        print!("{}", termion::cursor::Show);
    }
}
