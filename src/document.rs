use crate::Row;
use std::fmt;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: String,
}

impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(self.filename.as_str()).finish()
    }
}
impl Document {
    #[must_use]
    pub fn new(rows: Vec<Row>, filename: String) -> Self {
        Self { rows, filename }
    }
    /// # Errors
    ///
    /// Returns an error if a file bearing the provided filename
    /// cannot be open.
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
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
        self.rows.iter().map(Row::num_words).sum()
    }

    /// Get the document row corresponding to a given line number
    #[must_use]
    pub fn row_for_line_number(&self, line_number: usize) -> &Row {
        self.get_row(line_number.saturating_sub(1)).unwrap() // rows indices are 0 based
    }

    /// Return the line number of the last line in the file
    #[must_use]
    pub fn last_line_number(&self) -> usize {
        self.num_rows()
    }
}
