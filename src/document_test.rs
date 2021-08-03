use crate::{Document, Position, Row};

#[test]
fn test_document_get_row() {
    let doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        "test.rs".to_string(),
    );
    assert_eq!(doc.get_row(0).unwrap().string, "Hello".to_string());
    assert_eq!(doc.get_row(1).unwrap().string, "world!".to_string());
    assert!(doc.get_row(2).is_none());
}

#[test]
fn test_document_is_empty() {
    assert!(Document::new(vec![], "test.rs".to_string(),).is_empty());
    assert!(!Document::new(vec![Row::from("Hello")], "test.rs".to_string()).is_empty());
}

#[test]
fn test_document_num_rows() {
    assert_eq!(Document::new(vec![], "test.rs".to_string()).num_rows(), 0);
    assert_eq!(
        Document::new(vec![Row::from("")], "test.rs".to_string()).num_rows(),
        1
    );
}

#[test]
fn test_document_num_words() {
    assert_eq!(
        Document::new(
            vec![Row::from("Hello world"), Row::from("dear reviewer!")],
            "test.rs".to_string()
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
        Document::new(vec![row1, row2], "test.rs".to_string())
            .row_for_line_number(1)
            .unwrap()
            .string,
        "Hello world"
    );
    assert!(Document::default().row_for_line_number(1).is_some());
    assert!(Document::default().row_for_line_number(2).is_none());
}

#[test]
fn test_document_last_line_number() {
    assert_eq!(
        Document::new(
            vec![Row::from("Hello world"), Row::from("dear reviewer!")],
            "test.rs".to_string()
        )
        .last_line_number(),
        2
    );
}

#[test]
fn test_document_insert() {
    let mut doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        "test.rs".to_string(),
    );
    doc.insert(
        ' ',
        &Position {
            x: 6,
            y: 1,
            x_offset: 0,
        },
    );
    assert_eq!(doc.rows.get(0).unwrap().string, "Hello");
    assert_eq!(doc.rows.get(1).unwrap().string, "world! ");
    doc.insert(
        'W',
        &Position {
            x: 0,
            y: 2,
            x_offset: 0,
        },
    );
    assert_eq!(doc.rows.get(2).unwrap().string, "W");
}

#[test]
fn test_document_delete() {
    let mut doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        "test.rs".to_string(),
    );
    doc.delete(&Position {
        x: 5,
        y: 1,
        x_offset: 0,
    });
    assert_eq!(doc.rows.get(0).unwrap().string, "Hello");
    assert_eq!(doc.rows.get(1).unwrap().string, "world");
    doc.delete(&Position {
        x: 2,
        y: 1,
        x_offset: 0,
    });
    assert_eq!(doc.rows.get(1).unwrap().string, "wold");
}

#[test]
fn test_document_delete_at_start_of_line() {
    let mut doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        "test.rs".to_string(),
    );
    doc.delete(&Position {
        x: 0,
        y: 1,
        x_offset: 0,
    });
    assert_eq!(doc.rows.get(0).unwrap().string, "Helloworld!");
    assert!(doc.rows.get(1).is_none());
}

#[test]
fn test_insert_newline() {
    let mut doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        "test.rs".to_string(),
    );
    doc.insert_newline(&Position {
        x: 0,
        y: 0,
        x_offset: 0,
    });
    assert_eq!(doc.rows.get(0).unwrap().string, "");
    assert_eq!(doc.rows.get(1).unwrap().string, "Hello");
    assert_eq!(doc.rows.get(2).unwrap().string, "world!");

    doc.insert_newline(&Position {
        x: 0,
        y: 2,
        x_offset: 0,
    });
    assert_eq!(doc.rows.get(0).unwrap().string, "");
    assert_eq!(doc.rows.get(1).unwrap().string, "Hello");
    assert_eq!(doc.rows.get(2).unwrap().string, "");
    assert_eq!(doc.rows.get(3).unwrap().string, "world!");
}

#[test]
fn test_insert_newline_row_split() {
    let mut doc = Document::new(vec![Row::from("Hello world!")], "test.rs".to_string());
    doc.insert_newline(&Position {
        x: 5,
        y: 0,
        x_offset: 0,
    });
    assert_eq!(doc.rows.get(0).unwrap().string, "Hello");
    assert_eq!(doc.rows.get(1).unwrap().string, " world!");
}
