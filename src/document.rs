use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    #[must_use]
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let file_contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for line in file_contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Self { rows })
    }

    #[must_use]
    pub fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.len() == 0
    }
}
