use crate::{Boundary, Document, Navigator, Position, Row, ViewportOffset};
use std::path::PathBuf;

fn test_document() -> Document {
    Document::new(
        vec![
            Row::from("Test line 2"),
            Row::from(""),
            Row::from("Test line 2"),
        ],
        PathBuf::from("test.txt"),
    )
}

fn test_row_word_nav() -> Row {
    Row::from("const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);")
}

fn test_row_word_nav_unicode() -> Row {
    Row::from("I \u{9ec} unicode!")
}

#[test]
fn test_find_index_of_first_non_whitespace() {
    assert_eq!(
        Navigator::find_index_of_first_non_whitespace(&Row::from("  test")),
        Some(2)
    );
}

#[test]
fn test_find_matching_closing_symbol() {
    let doc = Document::new(vec![Row::from("fn test() {}")], PathBuf::from("test.txt"));
    assert_eq!(
        Navigator::find_matching_closing_symbol(
            &doc,
            &Position { x: 7, y: 0 },
            &ViewportOffset {
                columns: 0,
                rows: 0
            },
        ),
        Some(Position { x: 8, y: 0 })
    );
}
#[test]
fn test_find_matching_closing_symbol_multiline() {
    let doc = Document::new(
        vec![
            Row::from("fn test() {"),
            Row::from("  return 2;"),
            Row::from("};"),
        ],
        PathBuf::from("test.txt"),
    );
    assert_eq!(
        Navigator::find_matching_closing_symbol(
            &doc,
            &Position { x: 10, y: 0 },
            &ViewportOffset {
                columns: 0,
                rows: 0
            },
        ),
        Some(Position { x: 0, y: 2 })
    );
}

#[test]
fn test_find_matching_closing_symbol_no_match() {
    let doc = Document::new(vec![Row::from("fn test( {}")], PathBuf::from("test.txt"));
    assert_eq!(
        Navigator::find_matching_closing_symbol(
            &doc,
            &Position { x: 7, y: 0 },
            &ViewportOffset {
                columns: 0,
                rows: 0
            }
        ),
        None
    );
}

#[test]
fn test_find_matching_opening_symbol() {
    let doc = Document::new(vec![Row::from("fn test() {}")], PathBuf::from("test.txt"));
    assert_eq!(
        Navigator::find_matching_opening_symbol(
            &doc,
            &Position { x: 11, y: 0 },
            &ViewportOffset {
                columns: 0,
                rows: 0
            }
        ),
        Some(Position { x: 10, y: 0 })
    );
}

#[test]
fn test_find_matching_opening_symbol_multiline() {
    let doc = Document::new(
        vec![
            Row::from("fn test() {"),
            Row::from("  return 2;"),
            Row::from("};"),
        ],
        PathBuf::from("test.txt"),
    );
    assert_eq!(
        Navigator::find_matching_opening_symbol(
            &doc,
            &Position { x: 0, y: 2 },
            &ViewportOffset {
                columns: 0,
                rows: 0
            }
        ),
        Some(Position { x: 10, y: 0 })
    );
}

#[test]
fn test_find_matching_opening_symbol_no_match() {
    let doc = Document::new(vec![Row::from("fn test) {}")], PathBuf::from("test.txt"));
    assert_eq!(
        Navigator::find_matching_opening_symbol(
            &doc,
            &Position { x: 7, y: 0 },
            &ViewportOffset {
                columns: 0,
                rows: 0
            }
        ),
        None
    );
}

#[test]
/// Make sure that the end of the current paragraph is on the
/// next all-whitespace line.
fn test_find_line_number_of_end_of_paragraph() {
    let current_line_number = 1;
    assert_eq!(
        Navigator::find_line_number_of_start_or_end_of_paragraph(
            &test_document(),
            current_line_number,
            &Boundary::End
        ),
        2
    );
}

#[test]
/// Make sure that the end of the current paragraph when the cursor
/// is located on the last line of the doc is the current line
fn test_find_line_number_of_end_of_paragraph_when_at_end_of_document() {
    assert_eq!(
        Navigator::find_line_number_of_start_or_end_of_paragraph(
            &test_document(),
            test_document().last_line_number(),
            &Boundary::End
        ),
        test_document().last_line_number()
    );
}

#[test]
/// Make sure that the start of the current paragraph is on the
/// previous all-whitespace line.
fn test_find_line_number_of_start_of_paragraph() {
    assert_eq!(
        Navigator::find_line_number_of_start_or_end_of_paragraph(
            &test_document(),
            test_document().last_line_number(),
            &Boundary::Start
        ),
        2
    );
}

#[test]
/// Make sure that the start of the current paragraph when the cursor
/// is located on the first line of the doc is the current line
fn test_find_line_number_of_start_of_paragraph_when_at_first_line() {
    assert_eq!(
        Navigator::find_line_number_of_start_or_end_of_paragraph(
            &test_document(),
            1,
            &Boundary::Start
        ),
        1
    );
}

#[test]
fn test_is_word_delimiter_false() {
    assert!(!Navigator::is_word_delimiter('a', 'a'));
    assert!(!Navigator::is_word_delimiter('a', ' '));
    assert!(!Navigator::is_word_delimiter('a', '_'));
    assert!(!Navigator::is_word_delimiter('_', 'a'));
    assert!(!Navigator::is_word_delimiter(':', ':'));
}

#[test]
fn test_is_word_delimiter_true() {
    assert!(Navigator::is_word_delimiter('a', ':'));
    assert!(Navigator::is_word_delimiter(':', 'a'));
    assert!(Navigator::is_word_delimiter(' ', 'a'));
    assert!(Navigator::is_word_delimiter('"', 'a'));
    assert!(Navigator::is_word_delimiter('a', '"'));
}

#[test]
fn test_is_word_delimited_unicode() {
    assert!(Navigator::is_word_delimiter(' ', '\u{9ec}'));
}

#[test]
fn test_find_index_of_next_word() {
    let test_cases: Vec<(usize, usize)> = vec![
        // const STATUS_FG_COLOR
        // 0.....6
        (0, 6),
        // const STATUS_FG_COLOR: color::Rgb
        //       6..............^21
        (6, 21),
        // const STATUS_FG_COLOR: color::Rgb
        //                    21^.^23
        (21, 23),
        // const STATUS_FG_COLOR: color::Rgb
        //                      23^....^26
        (23, 28),
        (58, 58), // EOL
    ];
    for (start_index, expected_next_word_start_index) in test_cases {
        assert_eq!(
            Navigator::find_index_of_next_or_previous_word(
                &test_row_word_nav(),
                start_index,
                &Boundary::End
            ),
            expected_next_word_start_index
        );
    }
}

#[test]
fn test_find_index_of_next_word_with_unicode_chars() {
    let test_cases: Vec<(usize, usize)> = vec![
        // I * unicode!
        // 0.2.4......^11
        (0, 2),
        // I * unicode!
        // 0.2.4......^11
        (2, 4),
        (4, 11),
    ];
    for (start_index, expected_next_word_start_index) in test_cases {
        assert_eq!(
            Navigator::find_index_of_next_or_previous_word(
                &test_row_word_nav_unicode(),
                start_index,
                &Boundary::End
            ),
            expected_next_word_start_index
        );
    }
}

#[test]
fn test_find_index_of_previous_word() {
    let test_cases: Vec<(usize, usize)> = vec![
        // const STATUS_FG_COLOR
        // 0.....6
        (6, 0),
        // const STATUS_FG_COLOR: color::Rgb
        //       6..............^21
        (21, 6),
        // const STATUS_FG_COLOR: color::Rgb
        //                    21^.^23
        (23, 21),
        // const STATUS_FG_COLOR: color::Rgb
        //                      23^....^26
        (28, 23),
        (1, 0),
        (0, 0),
    ];
    for (start_index, expected_next_word_start_index) in test_cases {
        assert_eq!(
            Navigator::find_index_of_next_or_previous_word(
                &test_row_word_nav(),
                start_index,
                &Boundary::Start
            ),
            expected_next_word_start_index
        );
    }
}

#[test]
fn test_find_index_of_previous_word_with_unicode() {
    let test_cases: Vec<(usize, usize)> = vec![
        // I * unicode!
        // 0.2.4......^11
        (11, 4),
        (4, 2),
        (2, 0),
    ];
    for (start_index, expected_next_word_start_index) in test_cases {
        assert_eq!(
            Navigator::find_index_of_next_or_previous_word(
                &test_row_word_nav_unicode(),
                start_index,
                &Boundary::Start
            ),
            expected_next_word_start_index
        );
    }
}
