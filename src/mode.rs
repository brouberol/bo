use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Insert,
    Normal,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mode::Insert => write!(f, "INSERT"),
            Mode::Normal => write!(f, "NORMAL"),
        }
    }
}

#[cfg(test)]
#[path = "./mode_test.rs"]
mod mode_test;
