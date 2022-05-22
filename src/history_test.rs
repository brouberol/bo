use crate::history::{Operation, OperationType};
use crate::Position;

#[test]
fn test_insert_operation_end_position() {
    let op = Operation {
        op_type: OperationType::Insert,
        content: String::from("Hello"),
        start_position: Position { x: 0, y: 0 },
    };
    assert_eq!(op.end_position(&[5]), Position { x: 4, y: 0 });

    let op_with_newline = Operation {
        op_type: OperationType::Insert,
        content: String::from("Hello\nWorld"),
        start_position: Position { x: 0, y: 0 },
    };
    assert_eq!(
        op_with_newline.end_position(&[5, 5]),
        Position { x: 4, y: 1 }
    );

    let op_starting_with_newline = Operation {
        op_type: OperationType::Insert,
        content: String::from("\nHello\nWorld"),
        start_position: Position { x: 0, y: 0 },
    };
    assert_eq!(
        op_starting_with_newline.end_position(&[0, 5, 5]),
        Position { x: 4, y: 2 }
    );

    let op_starting_with_newline_not_at_start_of_doc = Operation {
        op_type: OperationType::Insert,
        content: String::from("\nplop"),
        start_position: Position { x: 11, y: 0 },
    };
    assert_eq!(
        op_starting_with_newline_not_at_start_of_doc.end_position(&[0, 4]),
        Position { x: 3, y: 1 }
    );
}

#[test]
fn test_insert_operation_with_multiple_adjacent_newlines_end_position() {
    let op_starting_with_adjacent_newlines = Operation {
        op_type: OperationType::Insert,
        content: String::from("hello\n\n\nplop"),
        start_position: Position { x: 0, y: 0 },
    };
    assert_eq!(
        op_starting_with_adjacent_newlines.end_position(&[5, 0, 0, 4]),
        Position { x: 3, y: 3 }
    );
}

#[test]
fn test_delete_operation_end_position() {
    let op = Operation {
        op_type: OperationType::Delete,
        content: String::from("olleH"),
        start_position: Position { x: 4, y: 0 },
    };
    assert_eq!(op.end_position(&[5]), Position { x: 0, y: 0 });

    let op_with_newline = Operation {
        op_type: OperationType::Delete,
        content: String::from("dlrow\nolleH"),
        start_position: Position { x: 4, y: 1 },
    };
    assert_eq!(
        op_with_newline.end_position(&[5, 5]),
        Position { x: 0, y: 0 }
    );
}

#[test]
fn test_operation_reversed() {
    let op = Operation {
        op_type: OperationType::Insert,
        content: String::from("Hello\n"),
        start_position: Position { x: 0, y: 0 },
    };
    let op_rev = op.reversed(&[5]);
    assert_eq!(
        op_rev,
        Operation {
            op_type: OperationType::Delete,
            content: String::from("\nolleH"),
            start_position: Position { x: 0, y: 1 },
        }
    );
    let op_rev_rev = op_rev.reversed(&[5, 0]);
    assert_eq!(op_rev_rev, op);
}

#[test]
fn test_operation_end_position_insert_single_line() {
    let op = Operation {
        op_type: OperationType::Insert,
        content: String::from("rûst"),
        start_position: Position { x: 0, y: 0 },
    };
    assert_eq!(op.end_position(&[4]), Position { x: 3, y: 0 });
}

#[test]
fn test_operation_end_position_insert_multi_line() {
    let op = Operation {
        op_type: OperationType::Insert,
        content: String::from("rûst\nröcks"),
        start_position: Position { x: 0, y: 0 },
    };
    assert_eq!(op.end_position(&[4, 5]), Position { x: 4, y: 1 });
}
#[test]
fn test_operation_end_position_insert_multiple_words() {
    let op = Operation {
        op_type: OperationType::Insert,
        content: String::from("rûst röcks"),
        start_position: Position { x: 0, y: 0 },
    };
    assert_eq!(op.end_position(&[10]), Position { x: 9, y: 0 });
}

#[test]
fn test_operation_end_position_insert_multi_line_starting_with_newline() {
    let op = Operation {
        op_type: OperationType::Insert,
        content: String::from("\nrûst\nröcks"),
        start_position: Position { x: 0, y: 0 },
    };
    assert_eq!(op.end_position(&[0, 4, 5]), Position { x: 4, y: 2 });
}

#[test]
fn test_operation_end_position_delete_single_line() {
    let op = Operation {
        op_type: OperationType::Delete,
        content: String::from("tsûr"),
        start_position: Position { x: 3, y: 0 },
    };
    assert_eq!(op.end_position(&[4]), Position { x: 0, y: 0 });
}

#[test]
fn test_operation_end_position_delete_multiple_words() {
    let op = Operation {
        op_type: OperationType::Delete,
        content: String::from("skcör tsûr"),
        start_position: Position { x: 9, y: 0 },
    };
    assert_eq!(op.end_position(&[10]), Position { x: 0, y: 0 });
}

#[test]
fn test_operation_end_position_delete_multi_line() {
    let op = Operation {
        op_type: OperationType::Delete,
        content: String::from("skcör\ntsur"),
        start_position: Position { x: 4, y: 1 },
    };
    assert_eq!(op.end_position(&[4, 5]), Position { x: 0, y: 0 });
}

#[test]
fn test_operation_end_position_delete_multi_line_starting_with_newline() {
    let op = Operation {
        op_type: OperationType::Delete,
        content: String::from("\nskcör\ntsur"),
        start_position: Position { x: 0, y: 2 },
    };
    assert_eq!(op.end_position(&[4, 5, 0]), Position { x: 0, y: 0 });
}
