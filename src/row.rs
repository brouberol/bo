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
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len()); // either stop at terminal end or string end
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}
