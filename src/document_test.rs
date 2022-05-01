use crate::{Document, LineNumber, Row, RowIndex};
use std::path::{Path, PathBuf};

#[test]
fn test_document_get_row() {
    let doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        PathBuf::from("test.rs"),
    );
    assert_eq!(
        doc.get_row(RowIndex::new(0)).unwrap().string,
        "Hello".to_string()
    );
    assert_eq!(
        doc.get_row(RowIndex::new(1)).unwrap().string,
        "world!".to_string()
    );
    assert!(doc.get_row(RowIndex::new(2)).is_none());
}

#[test]
fn test_document_is_empty() {
    assert!(Document::new(vec![], PathBuf::from("test.rs")).is_empty());
    assert!(!Document::new(vec![Row::from("Hello")], PathBuf::from("test.rs")).is_empty());
}

#[test]
fn test_document_num_rows() {
    assert_eq!(
        Document::new(vec![], PathBuf::from("test.rs")).num_rows(),
        0
    );
    assert_eq!(
        Document::new(vec![Row::from("")], PathBuf::from("test.rs")).num_rows(),
        1
    );
}

#[test]
fn test_document_num_words() {
    assert_eq!(
        Document::new(
            vec![Row::from("Hello world"), Row::from("dear reviewer!")],
            PathBuf::from("test.rs")
        )
        .num_words(),
        4
    );
}

#[test]
fn test_document_row_for_line_number() {
    let row1 = Row::from("Hello world");
    let row2 = Row::from("dear reviewer!");
    assert_eq!(
        Document::new(vec![row1, row2], PathBuf::from("test.rs"))
            .row_for_line_number(LineNumber::new(1))
            .unwrap()
            .string,
        "Hello world"
    );
    assert!(Document::default()
        .row_for_line_number(LineNumber::new(1))
        .is_some());
    assert!(Document::default()
        .row_for_line_number(LineNumber::new(2))
        .is_none());
}

#[test]
fn test_document_last_line_number() {
    assert_eq!(
        Document::new(
            vec![Row::from("Hello world"), Row::from("dear reviewer!")],
            PathBuf::from("test.rs")
        )
        .last_line_number(),
        LineNumber::new(2)
    );
}

#[test]
fn test_document_insert() {
    let mut doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        PathBuf::from("test.rs"),
    );
    doc.insert(' ', 6, RowIndex::new(1));
    assert_eq!(doc.rows.get(0).unwrap().string, "Hello");
    assert_eq!(doc.rows.get(1).unwrap().string, "world! ");
    doc.insert('W', 0, RowIndex::new(2));
    assert_eq!(doc.rows.get(2).unwrap().string, "W");
}

#[test]
fn test_document_insert_newline_at_the_end() {
    let mut doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        PathBuf::from("test.rs"),
    );
    assert_eq!(doc.num_rows(), 2);
    doc.insert_newline(6, RowIndex::new(1));
    assert_eq!(doc.num_rows(), 3);
}

#[test]
fn test_document_delete() {
    let mut doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        PathBuf::from("test.rs"),
    );
    doc.delete(5, 6, RowIndex::new(1));
    assert_eq!(doc.rows.get(0).unwrap().string, "Hello");
    assert_eq!(doc.rows.get(1).unwrap().string, "world");
    doc.delete(2, 6, RowIndex::new(1));
    assert_eq!(doc.rows.get(1).unwrap().string, "wold");
}

#[test]
fn test_document_delete_at_start_of_line() {
    let mut doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        PathBuf::from("test.rs"),
    );
    doc.delete(0, 0, RowIndex::new(1));
    assert_eq!(doc.rows.get(0).unwrap().string, "Helloworld!");
    assert!(doc.rows.get(1).is_none());
}

#[test]
fn test_document_delete_all_rows() {
    let mut doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        PathBuf::from("test.rs"),
    );
    doc.delete_row(RowIndex::new(1));
    doc.delete_row(RowIndex::new(0));
    assert_eq!(doc.get_row(RowIndex::new(0)).unwrap().string, "");
}

#[test]
fn test_insert_newline() {
    let mut doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        PathBuf::from("test.rs"),
    );
    doc.insert_newline(0, RowIndex::new(0));
    assert_eq!(doc.rows.get(0).unwrap().string, "");
    assert_eq!(doc.rows.get(1).unwrap().string, "Hello");
    assert_eq!(doc.rows.get(2).unwrap().string, "world!");

    doc.insert_newline(0, RowIndex::new(2));
    assert_eq!(doc.rows.get(0).unwrap().string, "");
    assert_eq!(doc.rows.get(1).unwrap().string, "Hello");
    assert_eq!(doc.rows.get(2).unwrap().string, "");
    assert_eq!(doc.rows.get(3).unwrap().string, "world!");
}

#[test]
fn test_insert_newline_row_split() {
    let mut doc = Document::new(vec![Row::from("Hello world!")], PathBuf::from("test.rs"));
    doc.insert_newline(5, RowIndex::new(0));
    assert_eq!(doc.rows.get(0).unwrap().string, "Hello");
    assert_eq!(doc.rows.get(1).unwrap().string, " world!");
}

#[test]
fn test_document_swapfile() {
    assert_eq!(
        Document::swap_filename(Path::new("test.txt")),
        PathBuf::from(".test.txt.swp")
    );
    assert_eq!(
        Document::swap_filename(Path::new("/home/br/code/bo/test.txt")),
        PathBuf::from("/home/br/code/bo/.test.txt.swp")
    );
}

#[test]
fn test_document_trim_trailing_spaces() {
    let mut doc = Document::new(
        vec![Row::from("Hello world!    ")],
        PathBuf::from("test.rs"),
    );
    doc.trim_trailing_spaces();
    assert_eq!(doc.rows.get(0).unwrap().string, "Hello world!");
}

#[test]
fn test_document_join_row_with_previous_one() {
    let mut doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        PathBuf::from("test.rs"),
    );
    doc.join_row_with_previous_one(4, RowIndex::new(1), Some(' '));
    assert_eq!(doc.rows.get(0).unwrap().string, "Hello world!");
    assert_eq!(doc.num_rows(), 1);
}
