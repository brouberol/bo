use crate::{Console, Document, Editor, Mode, Position, Row, Size};
use std::io::Error;
use termion::color;
use termion::event::{Event, Key, MouseEvent};

#[derive(Default)]
struct MockConsole {}

impl Console for MockConsole {
    fn read_event(&self) -> Result<Event, Error> {
        Ok(Event::Key(Key::Char('r')))
    }

    fn clear_screen(&self) {}

    fn clear_current_line(&self) {}

    /// # Errors
    ///
    /// Returns an error if stdout can't be flushed
    fn flush(&self) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn hide_cursor(&self) {}

    fn show_cursor(&self) {}

    fn set_bg_color(&self, _color: color::Rgb) {}

    fn reset_bg_color(&self) {}

    fn set_fg_color(&self, _color: color::Rgb) {}

    fn reset_fg_color(&self) {}

    fn to_alternate_screen(&self) {}

    fn to_main_screen(&self) {}

    fn clear_all(&self) {}

    fn set_cursor_as_steady_bar(&self) {}

    fn set_cursor_as_steady_block(&self) {}

    fn size(&self) -> &Size {
        &Size {
            height: 80,
            width: 120,
        }
    }

    fn middle_of_screen_line_number(&self) -> usize {
        self.size().height as usize / 2
    }

    fn set_cursor_position(&self, _position: &Position) {}

    #[must_use]
    fn get_cursor_index_from_mouse_event(
        &self,
        _mouse_event: MouseEvent,
        _x_offset: u8,
    ) -> Position {
        Position::default()
    }
}

fn get_short_document() -> Document {
    let lines: Vec<&str> = vec!["Hello world", "Hello world!", "Hello world!!"];
    let mut rows: Vec<Row> = vec![];
    for line in lines {
        rows.push(Row::from(line));
    }
    Document::new(rows, "test".to_string())
}

fn get_long_document() -> Document {
    let mut rows: Vec<Row> = vec![];
    for _ in 0..200 {
        rows.push(Row::from("Some line"));
    }
    Document::new(rows, "test".to_string())
}

fn get_test_editor() -> Editor {
    let mut editor = Editor::default(None);
    editor.document = get_short_document();
    editor
}

fn get_test_editor_with_long_document() -> Editor {
    let mut editor = Editor::default(None);
    editor.document = get_long_document();
    editor
}

fn assert_position_is(editor: &Editor, x: usize, y: usize) {
    assert_eq!(editor.cursor_position, Position { x, x_offset: 0, y });
}

fn assert_nth_row_is(editor: &Editor, n: usize, s: &str) {
    assert_eq!(editor.document.get_row(n).unwrap().string, String::from(s));
}

#[test]
fn test_editor_enter_mode() {
    let mut editor = get_test_editor();
    let console = MockConsole::default();
    assert_eq!(editor.mode, Mode::Normal); // default mode
    editor.enter_insert_mode(&console);
    assert_eq!(editor.mode, Mode::Insert);
    editor.enter_normal_mode(&console);
    assert_eq!(editor.mode, Mode::Normal);
}

#[test]
fn test_editor_command_buffer() {
    let mut editor = get_test_editor();
    assert!(!editor.is_receiving_command());
    editor.start_receiving_command();
    editor.command_buffer.push_str("help");
    assert!(editor.is_receiving_command());
    editor.stop_receiving_command();
    assert!(!editor.is_receiving_command());
}

#[test]
fn test_editor_pop_normal_command_repetitions() {
    let mut editor = get_test_editor();
    editor.normal_command_buffer.push("123".to_string());
    let times = editor.pop_normal_command_repetitions();
    assert_eq!(times, 123);
    assert!(editor.normal_command_buffer.is_empty());
}

#[test]
fn test_editor_process_keystroke_command() {
    let mut editor = get_test_editor();
    let console = MockConsole::default();
    assert!(!editor.is_receiving_command());
    editor.process_keystroke(Key::Char(':'), &console);
    assert!(editor.is_receiving_command());
}

#[test]
fn test_editor_process_keystroke_navigation() {
    let mut editor = get_test_editor();
    let console = MockConsole::default();
    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('j'), &console);
    assert_position_is(&editor, 0, 1);

    editor.process_keystroke(Key::Char('k'), &console);
    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('l'), &console);
    assert_position_is(&editor, 1, 0);

    editor.process_keystroke(Key::Char('h'), &console);
    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('2'), &console);
    editor.process_keystroke(Key::Char('j'), &console);
    assert_position_is(&editor, 0, 2);
}

#[test]
fn test_editor_help_command() {
    let mut editor = get_test_editor();
    let console = MockConsole::default();
    let input: Vec<char> = vec![':', 'h', 'e', 'l', 'p', '\n'];
    assert!(!editor.alternate_screen);
    for c in input {
        editor.process_keystroke(Key::Char(c), &console);
    }
    assert!(editor.alternate_screen);
    editor.process_keystroke(Key::Char('q'), &console);
    assert!(!editor.alternate_screen);
}

#[test]
fn test_editor_search() {
    let mut editor = get_test_editor();
    let console = MockConsole::default();
    let input: Vec<char> = vec!['/', 'w', 'o', 'r', 'l', 'd', '\n'];
    assert!(editor.search_matches.is_empty());
    for c in input {
        editor.process_keystroke(Key::Char(c), &console);
    }
    assert_eq!(editor.search_matches.len(), 3);
    assert_eq!(
        editor.search_matches,
        vec![
            (
                Position {
                    x: 6,
                    x_offset: 0,
                    y: 1
                },
                Position {
                    x: 12,
                    x_offset: 0,
                    y: 1
                }
            ),
            (
                Position {
                    x: 6,
                    x_offset: 0,
                    y: 2
                },
                Position {
                    x: 12,
                    x_offset: 0,
                    y: 2
                }
            ),
            (
                Position {
                    x: 6,
                    x_offset: 0,
                    y: 3
                },
                Position {
                    x: 12,
                    x_offset: 0,
                    y: 3
                }
            )
        ]
    );
    assert_eq!(editor.message, "Match 1/3");
    assert_eq!(editor.current_search_match_index, 0);

    editor.process_keystroke(Key::Char('n'), &console);
    assert_eq!(editor.current_search_match_index, 1);
    assert_position_is(&editor, 6, 1);

    editor.process_keystroke(Key::Char('n'), &console);
    assert_eq!(editor.current_search_match_index, 2);
    assert_position_is(&editor, 6, 2);

    editor.process_keystroke(Key::Char('n'), &console);
    assert_eq!(editor.current_search_match_index, 0);
    assert_position_is(&editor, 6, 0);

    editor.process_keystroke(Key::Esc, &console);
    assert!(editor.search_matches.is_empty());
    assert_eq!(editor.current_search_match_index, 0);
}

#[test]
fn test_editor_unknown_command() {
    let mut editor = get_test_editor();
    let console = MockConsole::default();
    let input: Vec<char> = vec![':', 'd', 'e', 'r', 'p', '\n'];
    for c in input {
        editor.process_keystroke(Key::Char(c), &console);
    }
    assert_eq!(
        editor.message,
        "\u{1b}[38;5;1mUnknown command 'derp'\u{1b}[39m"
    );
}

#[test]
fn test_editor_navigation() {
    let mut editor = get_test_editor();
    let console = MockConsole::default();
    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('G'), &console);
    assert_position_is(&editor, 0, 2);

    editor.process_keystroke(Key::Char('g'), &console);
    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('$'), &console);
    assert_position_is(&editor, 10, 0);

    editor.process_keystroke(Key::Char('^'), &console);
    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('w'), &console);
    assert_position_is(&editor, 6, 0);

    editor.process_keystroke(Key::Char('b'), &console);
    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('2'), &console);
    editor.process_keystroke(Key::Char('w'), &console);
    assert_position_is(&editor, 10, 0);

    editor.process_keystroke(Key::Char('2'), &console);
    editor.process_keystroke(Key::Char('b'), &console);
    assert_position_is(&editor, 0, 0);
}

#[test]
fn test_editor_edition() {
    let mut editor = get_test_editor();
    let console = MockConsole::default();

    assert_eq!(editor.document.num_rows(), 3);
    editor.process_keystroke(Key::Char('o'), &console);
    assert_position_is(&editor, 0, 1);
    assert_eq!(editor.document.num_rows(), 4);
    assert_nth_row_is(&editor, 1, "");

    editor.process_keystroke(Key::Esc, &console);
    editor.process_keystroke(Key::Char('O'), &console);
    assert_position_is(&editor, 0, 1);
    assert_eq!(editor.document.num_rows(), 5);
    assert_nth_row_is(&editor, 1, "");
    assert_nth_row_is(&editor, 2, "");

    editor.process_keystroke(Key::Esc, &console);
    assert_eq!(editor.document.num_rows(), 5);
    editor.process_keystroke(Key::Char('d'), &console);
    assert_eq!(editor.document.num_rows(), 4);

    editor.goto_x_y(0, 1, &console);
    editor.process_keystroke(Key::Char('i'), &console);
    assert_eq!(editor.mode, Mode::Insert);
    editor.process_keystroke(Key::Char('b'), &console);
    editor.process_keystroke(Key::Char('o'), &console);
    editor.process_keystroke(Key::Char('o'), &console);
    editor.process_keystroke(Key::Char('p'), &console);
    assert_nth_row_is(&editor, 1, "boop");
    editor.process_keystroke(Key::Backspace, &console);
    assert_nth_row_is(&editor, 1, "boo");

    editor.process_keystroke(Key::Esc, &console);
    assert_eq!(editor.mode, Mode::Normal);
    editor.process_keystroke(Key::Char('^'), &console);
    editor.process_keystroke(Key::Char('i'), &console);
    assert_eq!(editor.mode, Mode::Insert);
    assert_eq!(editor.document.num_rows(), 4);
    editor.process_keystroke(Key::Backspace, &console);
    assert_eq!(editor.document.num_rows(), 3);
    assert_nth_row_is(&editor, 0, "Hello worldboo");

    editor.goto_x_y(11, 0, &console);
    assert_position_is(&editor, 11, 0);
    assert_eq!(editor.document.num_rows(), 3);
    editor.process_keystroke(Key::Char('\n'), &console);
    assert_eq!(editor.document.num_rows(), 4);
    assert_nth_row_is(&editor, 0, "Hello world");
    assert_nth_row_is(&editor, 1, "boo");
    assert_position_is(&editor, 0, 1);

    editor.goto_x_y(0, 0, &console);
    editor.process_keystroke(Key::Esc, &console);
    editor.process_keystroke(Key::Char('x'), &console);
    assert_nth_row_is(&editor, 0, "ello world");

    editor.process_keystroke(Key::Char('A'), &console);
    assert_eq!(editor.mode, Mode::Insert);
    assert_position_is(&editor, 10, 0);
}

#[test]
fn test_editor_move_cursor_to_position_x() {
    let mut editor = get_test_editor();
    let console = MockConsole::default();

    assert_position_is(&editor, 0, 0);
    editor.move_cursor_to_position_x(1, &console);
    assert_position_is(&editor, 1, 0);
    assert_eq!(editor.offset.x, 0);

    editor.move_cursor_to_position_x(140, &console);
    assert_position_is(&editor, 119, 0);
    assert_eq!(editor.offset.x, 21);
}

#[test]
fn test_editor_move_cursor_to_position_y() {
    let mut editor = get_test_editor_with_long_document();
    let console = MockConsole::default();
    assert_position_is(&editor, 0, 0);
    assert_eq!(editor.offset.y, 0);

    editor.move_cursor_to_position_y(10, &console);
    assert_position_is(&editor, 0, 10);
    assert_eq!(editor.offset.y, 0);

    editor.move_cursor_to_position_y(200, &console);
    assert_position_is(&editor, 0, 80);
    assert_eq!(editor.offset.y, 120);

    editor.move_cursor_to_position_y(110, &console);
    assert_position_is(&editor, 0, 40);
    assert_eq!(editor.offset.y, 70);

    editor.move_cursor_to_position_y(112, &console);
    assert_position_is(&editor, 0, 42);
    assert_eq!(editor.offset.y, 70);

    editor.move_cursor_to_position_y(180, &console);
    assert_position_is(&editor, 0, 60);
    assert_eq!(editor.offset.y, 120);
}

#[test]
fn test_editor_goto_percentage_in_document() {
    let mut editor = get_test_editor_with_long_document();
    let console = MockConsole::default();
    editor.process_keystroke(Key::Char('1'), &console);
    editor.process_keystroke(Key::Char('0'), &console);
    editor.process_keystroke(Key::Char('%'), &console);
    assert_position_is(&editor, 0, 19); // line 20
}

#[test]
fn test_editor_navigate_long_document() {
    let mut editor = get_test_editor_with_long_document();
    let console = MockConsole::default();

    editor.move_cursor_to_position_y(110, &console);
    assert_position_is(&editor, 0, 40);
    assert_eq!(editor.offset.y, 70);

    editor.process_keystroke(Key::Char('H'), &console);
    assert_position_is(&editor, 0, 0);
    assert_eq!(editor.offset.y, 70);

    editor.process_keystroke(Key::Char('M'), &console);
    assert_position_is(&editor, 0, 40);
    assert_eq!(editor.offset.y, 70);

    editor.process_keystroke(Key::Char('L'), &console);
    assert_position_is(&editor, 0, 80);
    assert_eq!(editor.offset.y, 70);
}

#[test]
fn test_editor_simple_utilities() {
    let editor = get_test_editor();
    assert_eq!(editor.current_row_index(), 0);
    assert_eq!(editor.current_line_number(), 1);
    assert_eq!(editor.current_x_position(), 0);
    assert_eq!(editor.current_grapheme(), "H");
    assert_eq!(editor.current_row().string, "Hello world");
}

#[test]
fn test_editor_status() {
    let mut editor = get_test_editor();
    let console = MockConsole::default();

    assert_eq!(
        editor.generate_status(&console),
        format!("[test] NORMAL{}Ln 1, Col 1\r", " ".repeat(96))
    );

    editor.is_dirty = true;
    assert_eq!(
        editor.generate_status(&console),
        format!("[test] + NORMAL{}Ln 1, Col 1\r", " ".repeat(94))
    );
    editor.is_dirty = false;

    editor.cursor_position.x = 1;
    editor.cursor_position.y = 2;
    assert_eq!(
        editor.generate_status(&console),
        format!("[test] NORMAL{}Ln 3, Col 2\r", " ".repeat(96))
    );
    editor.cursor_position.x = 0;
    editor.cursor_position.y = 0;

    editor.config.display_stats = true;
    assert_eq!(
        editor.generate_status(&console),
        format!("[test] NORMAL{}[3L/6W] Ln 1, Col 1\r", " ".repeat(88))
    );
}

#[test]
fn test_editor_quit() {
    let mut editor = get_test_editor();
    assert!(!editor.should_quit);
    editor.quit(false);
    assert!(editor.should_quit);

    editor.should_quit = false;
    editor.is_dirty = true;
    assert!(!editor.should_quit);
    editor.quit(false);
    assert!(!editor.should_quit);
    assert_eq!(
        editor.message,
        "\u{1b}[38;5;1mUnsaved changes! Run :q! to override\u{1b}[39m"
    );

    editor.quit(true);
    assert!(editor.should_quit);
}
