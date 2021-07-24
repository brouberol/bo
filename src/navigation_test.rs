use crate::{Boundary, Document, Navigator, Row};

fn test_document() -> Document {
    Document::new(
        vec![
            Row::from("Test line 2"),
            Row::from(""),
            Row::from("Test line 2"),
        ],
        "test.txt".to_string(),
    )
}

fn test_row_word_nav() -> Row {
    Row::from("const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);")
}

#[test]
fn test_find_index_of_first_non_whitespace() {
    assert_eq!(
        Navigator::find_index_of_first_non_whitespace(&Row::from("  test")),
        Some(2)
    );
}

#[test]
fn test_find_x_index_of_matching_closing_symbol() {
    let row = Row::from("Hello(world);");
    assert_eq!(
        Navigator::find_x_index_of_matching_closing_symbol(&row, 5),
        Some(11)
    );
}

#[test]
fn test_find_x_index_of_matching_closing_symbol_no_match() {
    let row = Row::from("Hello(world;");
    assert_eq!(
        Navigator::find_x_index_of_matching_closing_symbol(&row, 5),
        None
    );
}

#[test]
fn test_find_x_index_of_matching_opening_symbol() {
    let row = Row::from("Hello(world);");
    assert_eq!(
        Navigator::find_x_index_of_matching_opening_symbol(&row, 11),
        Some(5)
    );
}

#[test]
fn test_find_x_index_of_matching_opening_symbol_no_match() {
    let row = Row::from("Helloworld);");
    assert_eq!(
        Navigator::find_x_index_of_matching_opening_symbol(&row, 11),
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
        )
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
        )
    }
}
