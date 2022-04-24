use super::SPACES_PER_TAB;
use crate::{Console, Document, Editor, Mode, Position, Row, Size};
use std::fmt;
use std::io::Error;
use std::path::PathBuf;
use termion::color;
use termion::event::{Event, Key, MouseEvent};

#[derive(Default)]
struct MockConsole {}

impl Console for MockConsole {
    fn read_event(&mut self) -> Result<Event, Error> {
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

    fn size(&self) -> Size {
        Size::default()
    }

    fn middle_of_screen_line_number(&self) -> usize {
        self.size().height as usize / 2
    }

    fn set_cursor_position(&self, _position: &Position, _row_prefix_length: u8) {}

    #[must_use]
    fn get_cursor_index_from_mouse_event(
        &self,
        _mouse_event: MouseEvent,
        _x_offset: u8,
    ) -> Position {
        Position::default()
    }
}

impl fmt::Debug for MockConsole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Terminal").finish()
    }
}

fn get_short_document() -> Document {
    let lines: Vec<&str> = vec!["Hello world", "Hello world!", "Hello world!!"];
    let mut rows: Vec<Row> = vec![];
    for line in lines {
        rows.push(Row::from(line));
    }
    Document::new(rows, PathBuf::from("test"))
}

fn get_long_document() -> Document {
    let mut rows: Vec<Row> = vec![];
    for _ in 0..200 {
        rows.push(Row::from("Some line"));
    }
    Document::new(rows, PathBuf::from("test"))
}

fn get_test_editor() -> Editor {
    let console = Box::new(MockConsole::default());
    let mut editor = Editor::new(None, console);
    editor.document = get_short_document();
    editor.last_saved_hash = editor.document.hashed();
    editor
}

fn get_test_editor_with_long_document() -> Editor {
    let console = Box::new(MockConsole::default());
    let mut editor = Editor::new(None, console);
    editor.document = get_long_document();
    editor.last_saved_hash = editor.document.hashed();
    editor
}

fn assert_position_is(editor: &Editor, x: usize, y: usize) {
    assert_eq!(editor.cursor_position, Position { x, y });
}

fn assert_nth_row_is(editor: &Editor, n: usize, s: &str) {
    assert_eq!(editor.document.get_row(n).unwrap().string, String::from(s));
}

fn process_keystrokes(editor: &mut Editor, keys: Vec<char>) {
    for c in keys {
        editor.process_keystroke(Key::Char(c));
    }
}

fn process_command(editor: &mut Editor, command: &str) {
    let mut command = String::from(command);
    command.push('\n');
    for c in command.chars() {
        editor.process_keystroke(Key::Char(c));
    }
}

#[test]
fn test_editor_enter_mode() {
    let mut editor = get_test_editor();
    assert_eq!(editor.mode, Mode::Normal); // default mode
    editor.enter_insert_mode();
    assert_eq!(editor.mode, Mode::Insert);
    editor.enter_normal_mode();
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

    assert!(!editor.is_receiving_command());
    editor.process_keystroke(Key::Char(':'));
    assert!(editor.is_receiving_command());
}

#[test]
fn test_editor_process_keystroke_navigation() {
    let mut editor = get_test_editor();

    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('j'));
    assert_position_is(&editor, 0, 1);

    editor.process_keystroke(Key::Char('k'));
    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('l'));
    assert_position_is(&editor, 1, 0);

    editor.process_keystroke(Key::Char('h'));
    assert_position_is(&editor, 0, 0);

    process_keystrokes(&mut editor, vec!['2', 'j']);
    assert_position_is(&editor, 0, 2);
}

#[test]
fn test_editor_change_x_position_when_moving_down_or_up() {
    let mut editor = get_test_editor();

    assert_position_is(&editor, 0, 0);

    // move to 3rd line "Hello world!!"
    process_command(&mut editor, ":3");
    // move at the end of the line
    editor.process_keystroke(Key::Char('$'));
    assert_position_is(&editor, 12, 2);

    // move up to 2nd line "Hello world!", shorter than "Hello world!".
    // After having moved, the cursor should be on the last "!" and not after it
    editor.process_keystroke(Key::Char('k'));
    assert_position_is(&editor, 11, 1);
}

#[test]
fn test_editor_help_command() {
    let mut editor = get_test_editor();

    assert!(!editor.alternate_screen);
    process_command(&mut editor, ":help");
    assert!(editor.alternate_screen);
    editor.process_keystroke(Key::Char('q'));
    assert!(!editor.alternate_screen);
}

#[test]
fn test_editor_goto_line() {
    let mut editor = get_test_editor();

    assert_position_is(&editor, 0, 0);
    process_command(&mut editor, ":2");
    assert_position_is(&editor, 0, 1);
    assert_eq!(editor.current_line_number(), 2);
}

#[test]
fn test_editor_search() {
    let mut editor = get_test_editor();

    assert!(editor.search_matches.is_empty());
    process_command(&mut editor, "/world");
    assert_eq!(editor.search_matches.len(), 3);
    assert_eq!(
        editor.search_matches,
        vec![
            (Position { x: 6, y: 1 }, Position { x: 12, y: 1 }),
            (Position { x: 6, y: 2 }, Position { x: 12, y: 2 }),
            (Position { x: 6, y: 3 }, Position { x: 12, y: 3 })
        ]
    );
    assert_eq!(editor.message, "Match 1/3");
    assert_eq!(editor.current_search_match_index, 0);

    editor.process_keystroke(Key::Char('n'));
    assert_eq!(editor.current_search_match_index, 1);
    assert_position_is(&editor, 6, 1);

    editor.process_keystroke(Key::Char('n'));
    assert_eq!(editor.current_search_match_index, 2);
    assert_position_is(&editor, 6, 2);

    editor.process_keystroke(Key::Char('n'));
    assert_eq!(editor.current_search_match_index, 0);
    assert_position_is(&editor, 6, 0);

    editor.process_keystroke(Key::Esc);
    assert!(editor.search_matches.is_empty());
    assert_eq!(editor.current_search_match_index, 0);
}

#[test]
fn test_editor_unknown_command() {
    let mut editor = get_test_editor();

    process_command(&mut editor, ":derp");
    assert_eq!(
        editor.message,
        "\u{1b}[38;5;1mUnknown command 'derp'\u{1b}[39m"
    );
}

#[test]
fn test_editor_navigation() {
    let mut editor = get_test_editor();

    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('G'));
    assert_position_is(&editor, 0, 2);

    editor.process_keystroke(Key::Char('g'));
    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('$'));
    assert_position_is(&editor, 10, 0);

    editor.process_keystroke(Key::Char('^'));
    assert_position_is(&editor, 0, 0);

    editor.process_keystroke(Key::Char('w'));
    assert_position_is(&editor, 6, 0);

    editor.process_keystroke(Key::Char('b'));
    assert_position_is(&editor, 0, 0);

    process_keystrokes(&mut editor, vec!['2', 'w']);
    assert_position_is(&editor, 10, 0);

    process_keystrokes(&mut editor, vec!['2', 'b']);
    assert_position_is(&editor, 0, 0);
}

#[test]
fn test_editor_deletion() {
    let mut editor = get_test_editor();

    editor.goto_x_y(1, 1);
    editor.process_keystroke(Key::Char('i'));
    editor.process_keystroke(Key::Backspace);
    assert_eq!(editor.document.num_rows(), 3);
    assert_eq!(editor.document.get_row(1).unwrap().string, "ello world!");
    editor.goto_x_y(0, 1);
    editor.process_keystroke(Key::Backspace);
    assert_eq!(editor.document.num_rows(), 2);
    assert_eq!(
        editor.document.get_row(0).unwrap().string,
        "Hello worldello world!"
    );
    assert_eq!(editor.document.get_row(1).unwrap().string, "Hello world!!");
}

#[test]
fn test_editor_edition() {
    let mut editor = get_test_editor();

    assert_eq!(editor.document.num_rows(), 3);
    editor.process_keystroke(Key::Char('o'));
    assert_position_is(&editor, 0, 1);
    assert_eq!(editor.document.num_rows(), 4);
    assert_nth_row_is(&editor, 1, "");

    editor.process_keystroke(Key::Esc);
    editor.process_keystroke(Key::Char('O'));
    assert_position_is(&editor, 0, 1);
    assert_eq!(editor.document.num_rows(), 5);
    assert_nth_row_is(&editor, 1, "");
    assert_nth_row_is(&editor, 2, "");

    editor.process_keystroke(Key::Esc);
    assert_eq!(editor.document.num_rows(), 5);
    editor.process_keystroke(Key::Char('d'));
    assert_eq!(editor.document.num_rows(), 4);

    editor.goto_x_y(0, 1);
    editor.process_keystroke(Key::Char('i'));
    assert_eq!(editor.mode, Mode::Insert);
    process_keystrokes(&mut editor, vec!['b', 'o', 'o', 'p']);
    assert_nth_row_is(&editor, 1, "boop");
    editor.process_keystroke(Key::Backspace);
    assert_nth_row_is(&editor, 1, "boo");

    editor.process_keystroke(Key::Esc);
    assert_eq!(editor.mode, Mode::Normal);
    process_keystrokes(&mut editor, vec!['^', 'i']);
    assert_eq!(editor.mode, Mode::Insert);
    assert_eq!(editor.document.num_rows(), 4);
    editor.process_keystroke(Key::Backspace);
    assert_eq!(editor.document.num_rows(), 3);
    assert_nth_row_is(&editor, 0, "Hello worldboo");

    editor.goto_x_y(11, 0);
    assert_position_is(&editor, 11, 0);
    assert_eq!(editor.document.num_rows(), 3);
    editor.process_keystroke(Key::Char('\n'));
    assert_eq!(editor.document.num_rows(), 4);
    assert_nth_row_is(&editor, 0, "Hello world");
    assert_nth_row_is(&editor, 1, "boo");
    assert_position_is(&editor, 0, 1);

    editor.goto_x_y(0, 0);
    editor.process_keystroke(Key::Esc);
    editor.process_keystroke(Key::Char('x'));
    assert_nth_row_is(&editor, 0, "ello world");

    editor.process_keystroke(Key::Char('A'));
    assert_eq!(editor.mode, Mode::Insert);
    assert_position_is(&editor, 10, 0);
}

#[test]
fn test_editor_insert_spaces_for_tab() {
    let mut editor = get_test_editor();

    process_keystrokes(&mut editor, vec!['i', '\t']);
    assert_position_is(&editor, SPACES_PER_TAB, 0);
    assert_nth_row_is(&editor, 0, "    Hello world");
}

#[test]
fn test_editor_move_cursor_to_position_x() {
    let mut editor = get_test_editor();

    assert_position_is(&editor, 0, 0);
    editor.move_cursor_to_position_x(1);
    assert_position_is(&editor, 1, 0);
    assert_eq!(editor.offset.columns, 0);

    editor.move_cursor_to_position_x(140);
    assert_position_is(&editor, 119, 0);
    assert_eq!(editor.offset.columns, 21);
}

#[test]
fn test_editor_move_cursor_to_position_y() {
    let mut editor = get_test_editor_with_long_document();

    assert_position_is(&editor, 0, 0);
    assert_eq!(editor.offset.rows, 0);

    editor.move_cursor_to_position_y(10);
    assert_position_is(&editor, 0, 10);
    assert_eq!(editor.offset.rows, 0);

    editor.move_cursor_to_position_y(200);
    assert_position_is(&editor, 0, 80);
    assert_eq!(editor.offset.rows, 120);

    editor.move_cursor_to_position_y(110);
    assert_position_is(&editor, 0, 40);
    assert_eq!(editor.offset.rows, 70);

    editor.move_cursor_to_position_y(112);
    assert_position_is(&editor, 0, 42);
    assert_eq!(editor.offset.rows, 70);

    editor.move_cursor_to_position_y(180);
    assert_position_is(&editor, 0, 60);
    assert_eq!(editor.offset.rows, 120);
}

#[test]
fn test_editor_goto_percentage_in_document() {
    let mut editor = get_test_editor_with_long_document();

    process_keystrokes(&mut editor, vec!['1', '0', '%']);
    assert_position_is(&editor, 0, 19); // line 20
}

#[test]
fn test_editor_navigate_long_document() {
    let mut editor = get_test_editor_with_long_document();

    editor.move_cursor_to_position_y(110);
    assert_position_is(&editor, 0, 40);
    assert_eq!(editor.offset.rows, 70);

    editor.process_keystroke(Key::Char('H'));
    assert_position_is(&editor, 0, 0);
    assert_eq!(editor.offset.rows, 70);

    editor.process_keystroke(Key::Char('M'));
    assert_position_is(&editor, 0, 40);
    assert_eq!(editor.offset.rows, 70);

    editor.process_keystroke(Key::Char('L'));
    assert_position_is(&editor, 0, 80);
    assert_eq!(editor.offset.rows, 70);
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

    assert_eq!(
        editor.generate_status(),
        format!("[test] NORMAL{}Ln 1, Col 1\r", " ".repeat(96))
    );

    // insert new characters
    process_keystrokes(&mut editor, vec!['i', 'o']);

    assert_eq!(
        editor.generate_status(),
        format!("[test] + INSERT{}Ln 1, Col 2\r", " ".repeat(94))
    );

    editor.process_keystroke(Key::Esc);

    assert_eq!(
        editor.generate_status(),
        format!("[test] + NORMAL{}Ln 1, Col 2\r", " ".repeat(94))
    );

    editor.cursor_position.x = 1;
    editor.cursor_position.y = 2;
    assert_eq!(
        editor.generate_status(),
        format!("[test] + NORMAL{}Ln 3, Col 2\r", " ".repeat(94))
    );
    editor.cursor_position.x = 0;
    editor.cursor_position.y = 0;

    editor.config.display_stats = true;
    assert_eq!(
        editor.generate_status(),
        format!("[test] + NORMAL{}[3L/6W] Ln 1, Col 1\r", " ".repeat(86))
    );
}

#[test]
fn test_editor_quit() {
    let mut editor = get_test_editor();
    assert!(!editor.should_quit);
    assert!(!editor.is_dirty());
    editor.quit(false);
    assert!(editor.should_quit);

    editor.should_quit = false;
    // insert new characters
    process_keystrokes(&mut editor, vec!['i', 'o']);

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

#[test]
fn test_editor_join_lines() {
    let mut editor = get_test_editor();
    // Go to end of line and join it with the next one
    process_keystrokes(&mut editor, vec!['$', 'J']);
    assert_nth_row_is(&editor, 0, "Hello world Hello world!");
    assert_eq!(editor.document.num_rows(), 2);
}
