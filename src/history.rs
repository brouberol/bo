use crate::Position;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use unicode_segmentation::UnicodeSegmentation;

const TIME_AFTER_WHICH_OPERATION_COMMITS: u64 = 1; // in seconds
const HISTORY_SIZE: usize = 8;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperationType {
    Insert,
    Delete,
}

impl OperationType {
    fn reversed(self) -> Self {
        match self {
            OperationType::Insert => OperationType::Delete,
            OperationType::Delete => OperationType::Insert,
        }
    }
}
/// An Operation describe a text edition at a specific start position.
///
/// An Operation can be of 2 types: either Insert or Delete, testifying of the fact
/// that we either inserted or deleted the provided text content, starting at a given
/// x/y position.
#[derive(Debug, PartialEq)]
pub struct Operation {
    pub content: String,
    pub start_position: Position,
    pub op_type: OperationType,
}

impl Operation {
    /// Append the argument string to the Operation content
    fn mut_push(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// Return the position the cursor would be at the end of the Operation.
    ///
    /// Examples:
    /// - an insert of "rust" starting at {0, 0} would return an end position of {3, 0}
    /// - an insert of "rust\nrocks" starting at {0, 0} would return an end position of {4, 1}
    /// - a deletion of "tsur" starting at {3, 0} would return an end position of {0, 0}
    /// - a deletion of "skcor\ntsur" starting at {1, 4} would return an end position of {0, 0}
    ///
    /// Note: this one took a long time to get right but seems to work. Check unit tests in doubt!
    #[must_use]
    pub fn end_position(&self, document_rows_length: &[usize]) -> Position {
        let mut x: usize = self.start_position.x;
        let mut y: usize = self.start_position.y;
        match self.op_type {
            OperationType::Insert => {
                for grapheme in self.content.graphemes(true) {
                    if grapheme == "\n" {
                        x = 0;
                        y += 1;
                    } else {
                        x += 1;
                    }
                }
                Position {
                    x: x.saturating_sub(1),
                    y,
                }
            }
            OperationType::Delete => {
                for grapheme in self.content.graphemes(true) {
                    if grapheme == "\n" {
                        y = y.saturating_sub(1);
                        x = *document_rows_length.get(y).unwrap_or(&0);
                    } else {
                        x = x.saturating_sub(1);
                    }
                }
                Position { x, y }
            }
        }
    }

    #[must_use]
    pub fn reversed(&self, document_rows_length: &[usize]) -> Self {
        Self {
            content: self.content.graphemes(true).rev().collect(),
            op_type: self.op_type.reversed(),
            start_position: self.end_position(document_rows_length),
        }
    }
}

/// History is a bounded double-ended ``Vec`` of ``Operations``. Every-time a new change is
/// registered, it is added to the history. If the time elapsed since the last change is
/// greater than ``TIME_AFTER_WHICH_OPERATION_COMMITS``, a new operation is added to the
/// ``operations`` ``VecDeque``. If not, the content of the back (last) operation is
/// mutated in place.
#[derive(Debug)]
pub struct History {
    pub operations: VecDeque<Operation>,
    pub last_edit_time: Instant,
}

impl Default for History {
    fn default() -> Self {
        Self {
            operations: VecDeque::with_capacity(HISTORY_SIZE),
            last_edit_time: Instant::now(),
        }
    }
}

impl History {
    fn set_last_edit_time_to_now(&mut self) {
        self.last_edit_time = Instant::now();
    }

    fn push(&mut self, text: &str, position: Position, operation_type: OperationType) {
        // maintain the history to its max size, to bound memory usage
        if self.operations.len() == HISTORY_SIZE {
            self.operations.pop_front();
        }
        self.operations.push_back(Operation {
            content: text.to_string(),
            start_position: position,
            op_type: operation_type,
        });
        self.set_last_edit_time_to_now();
    }
    /// Push (by either creating a new Operation or mutating the last one) an
    /// Insert operation in history based on the provided inserted text and position.
    fn push_insert(&mut self, text: &str, position: Position) {
        self.push(text, position, OperationType::Insert);
    }

    /// Push (by either creating a new Operation or mutating the last one) a
    /// Delete operation in history based on the provided deleted text and position.
    fn push_delete(&mut self, text: &str, position: Position) {
        self.push(text, position, OperationType::Delete);
    }

    /// Register that an insertion of provided text occured at the provided position.
    ///
    /// Either register that as a whole new ``Operation``, or mutate the back ``Operation``
    /// depending whether the elapsed time since the last operation is greater than
    /// ``TIME_AFTER_WHICH_OPERATION_COMMITS``.
    /// However, if the back operation was a Delee, push a new Insert ``Operation``
    /// in the history.
    pub fn register_insertion(&mut self, text: &str, position: Position) {
        if Instant::elapsed(&self.last_edit_time)
            >= Duration::new(TIME_AFTER_WHICH_OPERATION_COMMITS, 0)
            || self.operations.is_empty()
        {
            self.push_insert(text, position);
        } else if let Some(op) = self.operations.back_mut() {
            match op.op_type {
                OperationType::Insert => {
                    op.mut_push(text);
                    self.set_last_edit_time_to_now();
                }
                OperationType::Delete => self.push_insert(text, position),
            }
        }
    }

    /// Register that a deletion of provided text occured at the provided position.
    ///
    /// Either register that as a whole new ``Operation``, or mutate the back ``Operation``
    /// depending whether the elapsed time since the last operation is greater than
    /// ``TIME_AFTER_WHICH_OPERATION_COMMITS``.
    ///  However, if the back operation was an Insert, push a new Delete ``Operation``
    /// in the history.
    pub fn register_deletion(&mut self, text: &str, position: Position) {
        if Instant::elapsed(&self.last_edit_time)
            >= Duration::new(TIME_AFTER_WHICH_OPERATION_COMMITS, 0)
            || self.operations.is_empty()
        {
            self.push_delete(text, position);
        } else if let Some(op) = self.operations.back_mut() {
            match op.op_type {
                OperationType::Insert => self.push_delete(text, position),
                OperationType::Delete => {
                    op.mut_push(text);
                    self.set_last_edit_time_to_now();
                }
            }
        }
    }

    /// If any Operation is in the history, pop it and returm its reversed Operation.
    #[must_use]
    pub fn last_operation_reversed(&mut self, document_rows_length: &[usize]) -> Option<Operation> {
        self.operations
            .pop_back()
            .map(|op| op.reversed(document_rows_length))
    }
}

#[cfg(test)]
#[path = "./history_test.rs"]
mod history_test;
