use crate::utils;
use std::cmp;
use std::str;

#[derive(Debug)]
pub struct Row {
    pub string: String,
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        Self {
            string: String::from(s),
        }
    }
}

impl Row {
    #[must_use]
    pub fn render(&self, start: usize, end: usize, line_number: usize, x_offset: usize) -> String {
        let end = cmp::min(end, self.string.len()); // either stop at terminal end or string end
        let start = cmp::min(start, end);
        let visible = self.string.get(start..end).unwrap_or_default().to_string();
        let prefix = utils::zfill(line_number.to_string(), " ".to_string(), x_offset);
        format!("{} {}", prefix, visible)
    }

    pub fn chars(&self) -> std::str::Chars {
        self.string.chars()
    }

    #[must_use]
    pub fn is_whitespace(&self) -> bool {
        !self.chars().any(|c| !c.is_whitespace())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.string.len()
    }

    #[must_use]
    pub fn index(&self, index: usize) -> char {
        self.chars().nth(index).unwrap_or_default()
    }
}
