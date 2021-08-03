use crate::{Position, Row};
use std::cmp::Ordering;
use std::fmt;
use std::fs;
use std::io::{Error, Write};
use std::slice::Iter;

pub struct Document {
    rows: Vec<Row>,
    pub filename: String,
}

impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(self.filename.as_str()).finish()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            rows: vec![Row::from("")],
            filename: "".to_string(),
        }
    }
}

impl Document {
    #[must_use]
    pub fn new(rows: Vec<Row>, filename: String) -> Self {
        Self { rows, filename }
    }

    #[must_use]
    pub fn new_empty(filename: String) -> Self {
        Self {
            rows: vec![Row::from("")],
            filename,
        }
    }
    /// # Errors
    ///
    /// Returns an error if a file bearing the provided filename
    /// cannot be open.
    pub fn open(filename: &str) -> Result<Self, Error> {
        let file_contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for line in file_contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Self {
            rows,
            filename: filename.to_string().clone(),
        })
    }

    /// # Errors
    ///
    /// Can return an error if the file can't be created or written to.
    pub fn save(&self) -> Result<(), Error> {
        if !self.filename.is_empty() {
            let mut file = fs::File::create(self.filename.as_str())?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.len() == 0
    }

    #[must_use]
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }

    #[must_use]
    pub fn num_words(&self) -> usize {
        self.iter().map(Row::num_words).sum()
    }

    /// Get the document row corresponding to a given line number
    #[must_use]
    pub fn row_for_line_number(&self, line_number: usize) -> Option<&Row> {
        self.get_row(line_number.saturating_sub(1))
    }

    /// Return the line number of the last line in the file
    #[must_use]
    pub fn last_line_number(&self) -> usize {
        self.num_rows()
    }

    #[must_use]
    pub fn iter(&self) -> Iter<Row> {
        self.rows.iter()
    }

    pub fn insert(&mut self, c: char, at: &Position) {
        match at.y.cmp(&self.num_rows()) {
            Ordering::Equal | Ordering::Greater => {
                let mut row = Row::default();
                row.insert(0, c);
                self.rows.push(row);
            }
            Ordering::Less => {
                if let Some(row) = self.rows.get_mut(at.y) {
                    row.insert(at.x, c);
                }
            }
        }
    }

    pub fn delete(&mut self, at: &Position) {
        if at.y >= self.num_rows() {
            return;
        }
        // Deletion at the very start of a line means we append the current line to the previous one
        if at.x == 0 && at.y > 0 {
            let current_row = self.rows.remove(at.y);
            if let Some(previous_row) = self.rows.get_mut(at.y - 1) {
                previous_row.append(&current_row);
            }
        } else if let Some(row) = self.rows.get_mut(at.y) {
            row.delete(at.x);
        }
    }
}

#[cfg(test)]
#[path = "./document_test.rs"]
mod document_test;
