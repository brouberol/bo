use crate::{Document, Row};
use std::cmp;
use std::collections::HashMap;

fn matching_closing_symbols() -> HashMap<&'static str, &'static str> {
    [
        ("'", "'"),
        ("\"", "\""),
        ("{", "}"),
        ("<", ">"),
        ("(", ")"),
        ("[", "]"),
    ]
    .iter()
    .copied()
    .collect()
}

fn matching_opening_symbols() -> HashMap<&'static str, &'static str> {
    [
        ("'", "'"),
        ("\"", "\""),
        ("}", "{"),
        (">", "<"),
        (")", "("),
        ("]", "["),
    ]
    .iter()
    .copied()
    .collect()
}
#[derive(PartialEq)]
pub enum Boundary {
    Start,
    End,
}

#[derive(Debug)]
pub struct Navigator {}

impl Navigator {
    #[must_use]
    pub fn find_index_of_first_non_whitespace(row: &Row) -> Option<usize> {
        for (x, character) in row.string.chars().enumerate() {
            if !character.is_whitespace() {
                return Some(x);
            }
        }
        None
    }

    /// Return the index of the matching closing symbol (eg } for {, etc)
    /// # Panics
    /// TODO
    #[must_use]
    pub fn find_x_index_of_matching_closing_symbol(
        current_row: &Row,
        current_x_position: usize,
    ) -> Option<usize> {
        let symbol = current_row.index(current_x_position);
        if matching_closing_symbols().get(&symbol).is_some() {
            let mut stack = vec![symbol];
            let mut current_opening_symbol = symbol;

            for index in current_x_position.saturating_add(1)..current_row.len() {
                let c = current_row.index(index);
                if c == *matching_closing_symbols()
                    .get(&current_opening_symbol)
                    .unwrap()
                {
                    stack.pop();
                    if stack.is_empty() {
                        return Some(index);
                    }
                    current_opening_symbol = *stack.first().unwrap();
                } else if matching_closing_symbols().contains_key(&c) {
                    stack.push(c);
                    current_opening_symbol = c;
                }
            }
            None
        } else {
            None
        }
    }

    /// Return the index of the matching opening symbol (eg } for {, etc)
    /// # Panics
    /// TODO
    #[must_use]
    pub fn find_x_index_of_matching_opening_symbol(
        current_row: &Row,
        current_x_position: usize,
    ) -> Option<usize> {
        let symbol = current_row.index(current_x_position);
        if matching_opening_symbols().get(&symbol).is_some() {
            let mut stack = vec![symbol];
            let mut current_closing_symbol = symbol;

            for index in (0..current_x_position).rev() {
                let c = current_row.index(index);
                if c == *matching_opening_symbols()
                    .get(&current_closing_symbol)
                    .unwrap()
                {
                    stack.pop();
                    if stack.is_empty() {
                        return Some(index);
                    }
                    current_closing_symbol = *stack.first().unwrap();
                } else if matching_opening_symbols().contains_key(&c) {
                    stack.push(c);
                    current_closing_symbol = c;
                }
            }
            None
        } else {
            None
        }
    }

    #[must_use]
    pub fn find_line_number_of_start_or_end_of_paragraph(
        document: &Document,
        current_line_number: usize,
        boundary: &Boundary,
    ) -> usize {
        let mut current_line_number = current_line_number;
        loop {
            current_line_number = match boundary {
                Boundary::Start => cmp::max(1, current_line_number.saturating_sub(1)),
                Boundary::End => cmp::min(
                    document.last_line_number(),
                    current_line_number.saturating_add(1),
                ),
            };
            let current_line_followed_by_empty_line =
                document // whether both the current and next lines are empty
                    .row_for_line_number(current_line_number)
                    .is_whitespace()
                    && !document
                        .row_for_line_number(current_line_number - 1)
                        .is_whitespace();
            if current_line_number == document.last_line_number()
                || current_line_number == 1
                || current_line_followed_by_empty_line
            {
                return current_line_number;
            }
        }
    }

    #[allow(clippy::suspicious_operation_groupings)]
    #[must_use]
    // mirrorred over the look and feel of vim
    // Note: this assumes working on char, and I _think_ is is shaky at best
    // as we start supporting unicde, as an unicode is made of code points, each
    // of which is internally represented by a char, so this has no change of _really_ working well.
    // we should drop that function and try to rely on the string.split_word_bounds
    // method implemented in the unicode-segmentation crate. However, that crate seems
    // to drop all characters (eg: heart) that isn't alphabetic.
    pub fn is_word_delimiter(char1: char, char2: char) -> bool {
        if char2.is_whitespace() || char1 == '_' || char2 == '_' {
            return false;
        }
        (char1.is_alphabetic() && !char2.is_alphabetic())
            || (!char1.is_alphabetic() && char2.is_alphabetic())
            || (char1.is_alphanumeric() && char2.is_ascii_punctuation())
            || (char1.is_whitespace() && char2.is_alphanumeric())
    }

    #[must_use]
    pub fn find_index_of_next_or_previous_word(
        current_row: &Row,
        current_x_position: usize,
        boundary: &Boundary,
    ) -> usize {
        let current_x_index = current_x_position.saturating_add(1);
        match boundary {
            Boundary::End => {
                let mut current_char = current_row.nth_char(current_x_position);
                for (i, next_char) in current_row.chars().skip(current_x_index).enumerate() {
                    if Self::is_word_delimiter(current_char, next_char) {
                        return current_x_index.saturating_add(i);
                    }
                    current_char = next_char;
                }
                current_row.len().saturating_sub(1)
            }
            Boundary::Start => {
                for i in (1..current_x_index.saturating_sub(1)).rev() {
                    let current_char = current_row.nth_char(i);
                    let prev_char = current_row.nth_char(i.saturating_sub(1));
                    if Self::is_word_delimiter(prev_char, current_char) {
                        return i;
                    }
                }
                0
            }
        }
    }
}

#[cfg(test)]
#[path = "./navigation_test.rs"]
mod navigation_test;
