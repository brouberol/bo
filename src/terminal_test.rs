use crate::editor::Position;
use crate::terminal::AnsiPosition;

#[test]
fn test_position_to_ansi_position_conversion() {
    let pos = Position { x: 10, y: 5 };
    assert_eq!(AnsiPosition::from(pos), AnsiPosition { x: 11, y: 6 });
}
