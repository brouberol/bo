use crate::Mode;

#[test]
fn test_mode_display() {
    assert_eq!(format!("{}", Mode::Normal), "NORMAL");
    assert_eq!(format!("{}", Mode::Insert), "INSERT");
}
