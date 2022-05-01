use super::SPACES_PER_TAB;
use crate::LineNumber;
use crate::{AnsiPosition, Console, ConsoleSize, Document, Editor, Mode, Position, Row, RowIndex};
use std::fmt;
use std::fs;
use std::io::Error;
use std::io::Write;
use std::path::PathBuf;
use tempfile::{tempdir, NamedTempFile};
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

    fn size(&self) -> ConsoleSize {
        ConsoleSize::default()
    }

    fn text_area_size(&self) -> ConsoleSize {
        ConsoleSize {
            height: 78,
            width: 120,
        }
    }

    fn middle_of_screen_line_number(&self) -> LineNumber {
        LineNumber::new(self.text_area_size().height as usize / 2)
    }

    fn bottom_of_screen_line_number(&self) -> LineNumber {
        LineNumber::new(self.text_area_size().height as usize)
    }

    fn set_cursor_position_in_text_area(&self, _position: &Position, _row_prefix_length: u8) {}

    fn set_cursor_position_anywhere(&self, _position: &Position) {}

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
    assert_eq!(
        editor.document.get_row(RowIndex::new(n)).unwrap().string,
        String::from(s)
    );
}

fn assert_current_line_is(editor: &Editor, s: &str) {
    assert_eq!(editor.current_row().string, String::from(s));
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
    assert_eq!(editor.current_line_number(), LineNumber::new(2));
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

    editor.process_keystroke(Key::Char('N'));
    assert_eq!(editor.current_search_match_index, 2);
    assert_position_is(&editor, 6, 2);

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

    editor.goto_x_y(1, RowIndex::new(1));
    editor.process_keystroke(Key::Char('i'));
    editor.process_keystroke(Key::Backspace);
    assert_eq!(editor.document.num_rows(), 3);
    assert_eq!(
        editor.document.get_row(RowIndex::new(1)).unwrap().string,
        "ello world!"
    );
    editor.goto_x_y(0, RowIndex::new(1));
    editor.process_keystroke(Key::Backspace);
    assert_eq!(editor.document.num_rows(), 2);
    assert_eq!(
        editor.document.get_row(RowIndex::new(0)).unwrap().string,
        "Hello worldello world!"
    );
    assert_eq!(
        editor.document.get_row(RowIndex::new(1)).unwrap().string,
        "Hello world!!"
    );
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

    editor.goto_x_y(0, RowIndex::new(1));
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

    editor.goto_x_y(11, RowIndex::new(0));
    assert_position_is(&editor, 11, 0);
    assert_eq!(editor.document.num_rows(), 3);
    editor.process_keystroke(Key::Char('\n'));
    assert_eq!(editor.document.num_rows(), 4);
    assert_nth_row_is(&editor, 0, "Hello world");
    assert_nth_row_is(&editor, 1, "boo");
    assert_position_is(&editor, 0, 1);

    editor.goto_x_y(0, RowIndex::new(0));
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

    editor.move_cursor_to_position_y(RowIndex::new(10));
    assert_position_is(&editor, 0, 10);
    assert_eq!(editor.current_line_number(), LineNumber::new(11));
    assert_eq!(editor.offset.rows, 0);

    editor.move_cursor_to_position_y(RowIndex::new(199));
    assert_eq!(editor.current_line_number(), LineNumber::new(200));
    assert_eq!(editor.current_row_index(), RowIndex::new(199));
    // The editor is 78 lines high, and line 78 <--> row 77
    // and offset 122 + row 77 = row 199
    assert_position_is(&editor, 0, 77);
    assert_eq!(editor.offset.rows, 122);

    editor.move_cursor_to_position_y(RowIndex::new(110));
    assert_eq!(editor.current_line_number(), LineNumber::new(111));
    // The editor is 78 lines high, and its middle line is L39,
    // which means row 38, and row 38 + offset 72 = row 110
    assert_position_is(&editor, 0, 38);
    assert_eq!(editor.offset.rows, 72);

    // We stay in the same view
    editor.move_cursor_to_position_y(RowIndex::new(112));
    assert_position_is(&editor, 0, 40);
    assert_eq!(editor.offset.rows, 72);

    // We move to the last view
    editor.move_cursor_to_position_y(RowIndex::new(180));
    assert_eq!(editor.current_line_number(), LineNumber::new(181));
    // we see the last 78 lines, meaning the lines we see start at line
    // 200 - 78 = 122, and end at line 200.
    // Moving to line 181 means that we are located at the position
    // y = 181 - 122 - 1 = 58 (-1 because y is a rowindex)
    assert_position_is(&editor, 0, 58);
    assert_eq!(editor.offset.rows, 122);

    // We move to the first half view from the last
    editor.move_cursor_to_position_y(RowIndex::new(10));
    assert_position_is(&editor, 0, 10);
    assert_eq!(editor.current_line_number(), LineNumber::new(11));
    assert_eq!(editor.offset.rows, 0);
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

    editor.move_cursor_to_position_y(RowIndex::new(110));
    // The terminal is 78 lines high, middle line = 39, so middle
    // row = 38
    assert_position_is(&editor, 0, 38);
    assert_eq!(editor.offset.rows, 72);

    editor.process_keystroke(Key::Char('H'));
    assert_position_is(&editor, 0, 0);
    assert_eq!(editor.offset.rows, 72);

    editor.process_keystroke(Key::Char('M'));
    assert_position_is(&editor, 0, 38);
    assert_eq!(editor.offset.rows, 72);

    editor.process_keystroke(Key::Char('L'));
    assert_position_is(&editor, 0, 77);
    assert_eq!(editor.offset.rows, 72);
}

#[test]
fn test_editor_simple_utilities() {
    let editor = get_test_editor();
    assert_eq!(editor.current_row_index(), RowIndex::new(0));
    assert_eq!(editor.current_line_number(), LineNumber::new(1));
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

#[test]
fn test_editor_edit_long_document() {
    let mut editor = get_test_editor_with_long_document();
    assert_eq!(editor.document.num_rows(), 200);
    assert_eq!(editor.mode, Mode::Normal);
    editor.move_cursor_to_position_y(RowIndex::new(110));
    // line 111
    // terminal height is 78, and we're positioned at line 39, meaning
    // row 38, offset is 110 - 38 = 72
    assert_position_is(&editor, 0, 38);
    assert_eq!(editor.offset.rows, 72);

    // Go to Insert mode and append a new line
    editor.process_keystroke(Key::Char('o'));
    assert_eq!(editor.mode, Mode::Insert);
    assert_eq!(editor.document.num_rows(), 201);
    assert_position_is(&editor, 0, 39);
    assert_eq!(editor.offset.rows, 72);

    // write some text
    process_keystrokes(&mut editor, vec!['d', 'e', 'r', 'p']);
    assert_eq!(editor.document.num_rows(), 201);
    assert_current_line_is(&editor, "derp");
    assert_position_is(&editor, 4, 39);
    assert_eq!(editor.offset.rows, 72);

    // enter newline
    editor.process_keystroke(Key::Char('\n'));
    assert_eq!(editor.document.num_rows(), 202);
    assert_position_is(&editor, 0, 40);
    assert_eq!(editor.offset.rows, 72);
    assert_current_line_is(&editor, "");

    // delete line
    editor.process_keystroke(Key::Backspace);
    assert_position_is(&editor, 4, 39);
    assert_current_line_is(&editor, "derp");
}

#[test]
fn test_position_from_ansiposition() {
    let ap = AnsiPosition { x: 10, y: 8 }; // 1-indexed
    let p = Position::from(ap); // 0-indexed
    assert_eq!(p.x, 9);
    assert_eq!(p.y, 7);
}

#[test]
fn test_editor_serialize() {
    let editor = get_test_editor();
    let serialized_editor = serde_json::to_string_pretty(&editor).unwrap();
    assert_eq!(
        serialized_editor,
        r#"{
  "cursor_position": {
    "x": 0,
    "y": 0
  },
  "offset": {
    "rows": 0,
    "columns": 0
  },
  "mode": "NORMAL",
  "command_buffer": "",
  "normal_command_buffer": [],
  "search_matches": [],
  "current_search_match_index": 0,
  "unsaved_edits": 0,
  "last_saved_hash": 6894519061004685273,
  "row_prefix_length": 0,
  "document": {
    "rows": [
      {
        "string": "Hello world"
      },
      {
        "string": "Hello world!"
      },
      {
        "string": "Hello world!!"
      }
    ],
    "filename": "test"
  }
}"#
    );
}

#[test]
fn test_open_existing_file() {
    let console = Box::new(MockConsole::default());
    let mut f = NamedTempFile::new().unwrap();
    f.write_all("Hello\nHello!\nHello!!\n".as_bytes()).unwrap();
    let f_name_pathbuf: PathBuf = f.path().to_path_buf();
    let f_name_str: String = f_name_pathbuf.to_str().unwrap().to_string(); // gawd
    let editor = Editor::new(Some(f_name_str), console);
    assert_eq!(editor.document.filename, Some(f_name_pathbuf));
}

#[test]
fn test_stop_receiving_command_after_processing_esc_key() {
    let mut editor = get_test_editor();
    editor.process_keystroke(Key::Char(':'));
    assert!(editor.is_receiving_command());
    editor.process_keystroke(Key::Esc);
    assert!(!editor.is_receiving_command());
}

#[test]
fn test_process_backspace_mid_receiving_command() {
    let mut editor = get_test_editor();
    process_keystrokes(&mut editor, vec![':', 'o']);
    assert!(editor.is_receiving_command());
    assert_eq!(editor.command_buffer, String::from(":o"));
    editor.process_keystroke(Key::Backspace);
    assert!(editor.is_receiving_command());
    assert_eq!(editor.command_buffer, String::from(":"));
}

#[test]
fn test_open_non_existing_file() {
    let mut editor = get_test_editor();
    process_command(&mut editor, ":o nope.txt");
    // the file will be opened but unsaved
    assert_eq!(editor.document.filename, Some(PathBuf::from("nope.txt")));
}

#[test]
fn test_new_file() {
    let mut editor = get_test_editor();
    process_command(&mut editor, ":new nope.txt");
    // the file will be opened but unsaved
    assert_eq!(editor.document.filename, Some(PathBuf::from("nope.txt")));
}

#[test]
fn test_save_file() {
    let console = Box::new(MockConsole::default());
    let f = NamedTempFile::new().unwrap();
    let f_name_pathbuf: PathBuf = f.path().to_path_buf();
    let f_name_str: String = f_name_pathbuf.to_str().unwrap().to_string(); // gawd
    let mut editor = Editor::new(Some(f_name_str), console);

    process_keystrokes(&mut editor, vec!['i', 'h', 'e', 'l', 'l', 'o']);
    editor.process_keystroke(Key::Esc);
    process_command(&mut editor, ":w");
    assert_eq!(editor.unsaved_edits, 0);

    let content = fs::read_to_string(f).unwrap();
    assert_eq!(content, "hello\n");
}

#[test]
fn test_save_file_trim_whitespaces() {
    let console = Box::new(MockConsole::default());
    let f = NamedTempFile::new().unwrap();
    let f_name_pathbuf: PathBuf = f.path().to_path_buf();
    let f_name_str: String = f_name_pathbuf.to_str().unwrap().to_string(); // gawd
    let mut editor = Editor::new(Some(f_name_str), console);

    process_keystrokes(&mut editor, vec!['i', ' ', 'h', 'e', 'l', 'l', 'o', ' ']);
    editor.process_keystroke(Key::Esc);
    process_command(&mut editor, ":w");
    assert_eq!(editor.unsaved_edits, 0);

    let content = fs::read_to_string(f).unwrap();
    assert_eq!(content, " hello\n"); // trailing whitespace has been removed
}

#[test]
fn test_display_line_numbers() {
    let mut editor = get_test_editor();
    assert!(!editor.config.display_line_numbers);
    process_command(&mut editor, ":ln");
    assert!(editor.config.display_line_numbers);
    process_command(&mut editor, ":ln");
    assert!(!editor.config.display_line_numbers);
}

#[test]
fn test_display_stats() {
    let mut editor = get_test_editor();
    assert!(!editor.config.display_stats);
    process_command(&mut editor, ":stats");
    assert!(editor.config.display_stats);
    process_command(&mut editor, ":stats");
    assert!(!editor.config.display_stats);
}

#[test]
fn test_go_to_start_of_line() {
    let mut editor = get_test_editor();
    editor.process_keystroke(Key::Char('w'));
    assert_position_is(&editor, 6, 0);
    editor.process_keystroke(Key::Char('0'));
    assert_position_is(&editor, 0, 0);
}

#[test]
fn test_goto_matching_closing_symbol() {
    let mut editor = get_test_editor();
    editor.process_keystroke(Key::Char('A'));
    process_keystrokes(&mut editor, vec!['(', 'o', 'h', ')']);
    let first_line_content = editor
        .document
        .get_row(RowIndex::new(0))
        .unwrap()
        .string
        .clone();
    assert_eq!(first_line_content.chars().nth(11), Some('('));
    assert_eq!(first_line_content.chars().nth(14), Some(')'));
    editor.cursor_position = Position { x: 11, y: 0 }; // first paren
    editor.process_keystroke(Key::Esc);
    editor.process_keystroke(Key::Char('m'));
    assert_position_is(&editor, 14, 0);
}

#[test]
fn test_move_by_paragraph() {
    let mut editor = get_test_editor();
    assert_position_is(&editor, 0, 0);
    editor.process_keystroke(Key::Char('}'));
    assert_position_is(&editor, 0, 2);
    editor.process_keystroke(Key::Char('{'));
    assert_position_is(&editor, 0, 0);
}

#[test]
fn test_delete_last_line() {
    let mut editor = get_test_editor();
    assert_eq!(editor.document.num_rows(), 3);
    editor.process_keystroke(Key::Char('G'));
    assert_position_is(&editor, 0, 2);
    editor.process_keystroke(Key::Char('d'));
    assert_eq!(editor.document.num_rows(), 2);
    assert_position_is(&editor, 0, 1);
}

#[test]
fn test_process_command_not_found() {
    let mut editor = get_test_editor();
    process_command(&mut editor, ":nope");
    assert_eq!(editor.message, r#"[38;5;1mUnknown command 'nope'[39m"#);
}

#[test]
fn test_save_and_quit() {
    let dir = tempdir().unwrap();
    if std::env::set_current_dir(&dir).is_ok() {
        let mut editor = get_test_editor();
        process_keystrokes(&mut editor, vec!['G', 'o', 'd', 'e', 'r', 'p']);
        editor.process_keystroke(Key::Esc);
        assert_eq!(editor.unsaved_edits, 4);
        assert!(!editor.should_quit);
        process_command(&mut editor, ":wq");
        assert_eq!(editor.unsaved_edits, 0);
        assert!(editor.should_quit);
    };
}
