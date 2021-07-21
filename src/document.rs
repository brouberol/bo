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
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
