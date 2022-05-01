/// This module defines types which role is to abstract the different
/// coordinate and indexing systems the editor has to support.
/// Humans deal with line numbers, starting at 1, which maps to document
/// rows, starting at 0. We define simple types in order to delegate the
/// +1/-1 conversions to them, as it has proven easy to forget these only
/// when dealing with usize values.

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RowIndex {
    pub value: usize,
}

impl From<LineNumber> for RowIndex {
    fn from(ln: LineNumber) -> Self {
        Self::new(ln.value.saturating_sub(1))
    }
}

impl RowIndex {
    #[must_use]
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    #[must_use]
    pub fn next(&self) -> Self {
        Self::new(self.value.saturating_add(1))
    }

    #[must_use]
    pub fn previous(&self) -> Self {
        Self::new(self.value.saturating_sub(1))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct LineNumber {
    pub value: usize,
}

impl From<RowIndex> for LineNumber {
    fn from(ri: RowIndex) -> Self {
        Self::new(ri.value.saturating_add(1))
    }
}

impl LineNumber {
    #[must_use]
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    #[must_use]
    pub fn add(&self, value: usize) -> Self {
        Self::new(self.value.saturating_add(value))
    }

    #[must_use]
    pub fn sub(&self, value: usize) -> Self {
        Self::new(self.value.saturating_sub(value))
    }

    #[must_use]
    pub fn next(&self) -> Self {
        self.add(1)
    }

    #[must_use]
    pub fn previous(&self) -> Self {
        self.sub(1)
    }
}
