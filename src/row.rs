use crate::utils;
use std::cmp;

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

    #[must_use]
    pub fn is_whitespace(&self) -> bool {
        !self.string.chars().any(|c| !c.is_whitespace())
    }
}
