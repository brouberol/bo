use crate::utils;
use serde::Serialize;
use std::cmp;
use std::hash::Hash;
use std::str;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default, Hash, Serialize)]
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
        let mut visible = String::new();
        for grapheme in self.graphemes().skip(start).take(end - start) {
            visible.push_str(grapheme);
        }
        let prefix = if x_offset == 0 {
            "".to_string()
        } else {
            format!("{} ", utils::zfill(&line_number.to_string(), " ", x_offset))
        };
        format!("{}{}", prefix, visible)
    }

    pub fn chars(&self) -> std::str::Chars {
        self.string.chars()
    }

    #[must_use]
    pub fn graphemes(&self) -> unicode_segmentation::Graphemes {
        self.string[..].graphemes(true)
    }

    #[must_use]
    pub fn is_whitespace(&self) -> bool {
        !self.string.chars().any(|c| !c.is_whitespace())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.string[..].graphemes(true).count()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn nth_grapheme(&self, index: usize) -> &str {
        self.graphemes().nth(index).unwrap_or_default()
    }

    #[must_use]
    pub fn nth_char(&self, index: usize) -> char {
        self.chars().nth(index).unwrap_or_default()
    }

    #[must_use]
    pub fn num_words(&self) -> usize {
        self.string.unicode_words().count()
    }

    #[must_use]
    pub fn contains(&self, pattern: &str) -> bool {
        self.string.contains(pattern)
    }

    #[must_use]
    pub fn find(&self, pattern: &str) -> Option<usize> {
        self.string.find(pattern)
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn trim_end_inplace(&mut self) {
        self.string = String::from(self.string.trim_end());
    }

    /// Insert a character in the provided x index
    pub fn insert(&mut self, index: usize, c: char) {
        if index >= self.len() {
            self.string.push(c); // Append at the end of the row
        } else {
            // mid row edition
            let mut before: String = self.graphemes().take(index).collect();
            let after: String = self.graphemes().skip(index).collect();
            before.push(c);
            before.push_str(&after);
            self.string = before;
        }
    }

    /// Delete the character located at provided index
    pub fn delete(&mut self, index: usize) {
        if index >= self.len() {
            return;
        }
        let mut before: String = self.graphemes().take(index).collect();
        let after: String = self.graphemes().skip(index.saturating_add(1)).collect();
        before.push_str(&after);
        self.string = before;
    }

    /// Append a string at the end of the current one
    pub fn append(&mut self, other: &Self) {
        self.string = format!("{}{}", self.string, other.string);
    }

    #[must_use]
    pub fn split(&mut self, at: usize) -> Self {
        let before: String = self.graphemes().take(at).collect();
        let after: String = self.graphemes().skip(at).collect();
        self.string = before;
        Self::from(&after[..])
    }
}

#[cfg(test)]
#[path = "./row_test.rs"]
mod row_test;
