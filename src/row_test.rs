use crate::Row;

#[test]
fn test_row_render() {
    // fn render(&self, start: usize, end: usize, line_number: usize, x_offset: usize)
    assert_eq!(Row::from("Test").render(0, 50, 1, 0), "Test");
    assert_eq!(Row::from("Test").render(0, 50, 1, 4), "   1 Test");
    assert_eq!(Row::from("Test").render(0, 50, 11, 4), "  11 Test");
    assert_eq!(Row::from("Test").render(10, 60, 11, 4), "  11 ");
    assert_eq!(Row::from("\u{2764}").render(0, 50, 11, 4), "  11 \u{2764}");
}

#[test]
fn test_row_graphemes_index() {
    let row = Row::from("I \u{2764} unicode!");
    let mut graphemes = row.graphemes();
    assert_eq!(graphemes.next(), Some("I"));
    assert_eq!(graphemes.next(), Some(" "));
    assert_eq!(graphemes.next(), Some("\u{2764}"));
    assert_eq!(graphemes.next(), Some(" "));
    assert_eq!(graphemes.next(), Some("u"));
}

#[test]
fn test_row_len() {
    assert_eq!(Row::from("Hello World!").len(), 12);
    assert_eq!(Row::from("\u{2764}\u{2764}\u{2764}!").len(), 4); // 3 unicode hearts
    assert_eq!(Row::from("").len(), 0);
}

#[test]
fn test_row_is_empty() {
    assert!(Row::from("").is_empty());
}

#[test]
fn test_row_index() {
    assert_eq!(Row::from("I \u{2764} unicode!").index(2), "\u{2764}")
}

#[test]
fn test_row_num_words() {
    assert_eq!(Row::from("I l\u{f8}ve unicode!").num_words(), 3);
    assert_eq!(Row::from("I \u{9ec} unicode!").num_words(), 3);

    // "weird cases": turns out a heart isn't alphabetic, so it's not considered
    // a word.
    assert_eq!(Row::from("I \u{2764} unicode!").num_words(), 2);
    assert_eq!(Row::from("I \u{2764}\u{2764} unicode!").num_words(), 2);
}

#[test]
fn test_row_contains() {
    assert!(Row::from("I \u{2764} unicode!").contains("\u{2764}"));

    // check that the match is done on the unicode char and not the raw text
    assert!(!Row::from("I \u{2764} unicode!").contains("2764"));
    assert!(!Row::from("Hello").contains("Plop"));
    assert!(Row::from("Hello").contains("lo"));
    assert!(!Row::from("Hello").contains("LO"));
}

#[test]
fn test_row_find() {
    assert_eq!(Row::from("Hello world!").find("world"), Some(6));
    assert_eq!(Row::from("Hello world!").find("\u{2764}"), None);
    assert_eq!(Row::from("Hello \u{2764} world!").find("\u{2764}"), Some(6));
}

#[test]
fn test_row_is_whitespace() {
    // whitespaces
    assert!(Row::from(" ").is_whitespace());
    assert!(Row::from("\t").is_whitespace());
    assert!(Row::from("\t ").is_whitespace());
    assert!(!Row::from("a").is_whitespace());
    assert!(!Row::from("aa").is_whitespace());
    assert!(!Row::from(" \u{2764}").is_whitespace());
}

#[test]
fn test_row_string_chars() {
    assert_eq!(
        Row::from(" \u{2764}").string.chars().collect::<Vec<char>>(),
        [' ', '\u{2764}']
    );
}

#[test]
fn test_row_insert() {
    let mut row = Row::from("Hell");
    row.insert(4, 'o');
    assert_eq!(row.string, "Hello");
    row.insert(8, 'o');
    assert_eq!(row.string, "Helloo");
    row.insert(0, '.');
    assert_eq!(row.string, ".Helloo");
}

#[test]
fn test_row_delete() {
    let mut row = Row::from("Hello!");
    row.delete(8); // outside the string's boundaries
    assert_eq!(row.string, "Hello!");
    row.delete(5);
    assert_eq!(row.string, "Hello");
    row.delete(2);
    assert_eq!(row.string, "Helo");
}
