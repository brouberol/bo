use crate::{commands, utils, Boundary, Config, Console, Document, Mode, Navigator, Row};
use std::cmp;
use std::env;
use std::io;
use termion::color;
use termion::event::{Event, Key, MouseButton, MouseEvent};

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG: &str = env!("CARGO_PKG_NAME");
const COMMAND_PREFIX: char = ':';
const SEARCH_PREFIX: char = '/';
const LINE_NUMBER_OFFSET: u8 = 4; // number of chars
const START_X: u8 = LINE_NUMBER_OFFSET as u8; // index, so that's actually an offset of 5 chars
const SPACES_PER_TAB: usize = 4;

#[derive(Debug, Default, PartialEq)]
pub struct Position {
    pub x: usize,
    pub x_offset: u8,
    pub y: usize,
}

impl Position {
    pub fn reset_x(&mut self) {
        self.x = 0;
    }
    #[must_use]
    pub fn top_left() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Editor {
    should_quit: bool,
    cursor_position: Position,
    document: Document,
    offset: Position,
    message: String,
    mode: Mode,
    command_buffer: String,
    config: Config,
    normal_command_buffer: Vec<String>,
    mouse_event_buffer: Vec<Position>,
    search_matches: Vec<(Position, Position)>,
    current_search_match_index: usize,
    alternate_screen: bool,
    is_dirty: bool,
    terminal: Box<dyn Console>,
}

fn die(e: &io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}

impl Editor {
    pub fn new(filename: Option<String>, terminal: Box<dyn Console>) -> Self {
        let document: Document = match filename {
            None => Document::default(),
            Some(path) => Document::open(utils::expand_tilde(&path).as_str()).unwrap_or_default(),
        };
        Self {
            should_quit: false,
            cursor_position: Position::top_left(),
            document,
            offset: Position::default(),
            message: "".to_string(),
            mode: Mode::Normal,
            command_buffer: "".to_string(),
            config: Config::default(),
            normal_command_buffer: vec![],
            mouse_event_buffer: vec![],
            search_matches: vec![],
            current_search_match_index: 0,
            alternate_screen: false,
            is_dirty: false,
            terminal,
        }
    }

    /// Main screen rendering loop
    pub fn run(&mut self) {
        loop {
            if let Err(error) = &self.refresh_screen() {
                die(&error);
            }
            if let Err(error) = self.process_event() {
                die(&error);
            }
            if self.should_quit {
                self.terminal.clear_screen();
                break;
            }
        }
    }

    /// Main event processing method. An event can be either be a keystroke or a mouse click
    fn process_event(&mut self) -> Result<(), std::io::Error> {
        let event = self.terminal.read_event()?;
        match event {
            Event::Key(pressed_key) => self.process_keystroke(pressed_key),
            Event::Mouse(mouse_event) => self.process_mouse_event(mouse_event),
            Event::Unsupported(_) => (),
        }
        Ok(())
    }

    /// React to a keystroke. The reaction itself depends on the editor
    /// mode (insert, command, normal) or whether the editor is currently
    /// receiving a user input command (eg: ":q", etc).
    fn process_keystroke(&mut self, pressed_key: Key) {
        if self.is_receiving_command() {
            // accumulate the command in the command buffer
            match pressed_key {
                Key::Esc => self.stop_receiving_command(),
                Key::Char('\n') => {
                    // Enter
                    self.process_received_command();
                    self.stop_receiving_command();
                }
                Key::Char(c) => self.command_buffer.push(c), // accumulate keystrokes into the buffer
                Key::Backspace => self
                    .command_buffer
                    .truncate(self.command_buffer.len().saturating_sub(1)),
                _ => (),
            }
        } else {
            match self.mode {
                Mode::Normal => self.process_normal_command(pressed_key),
                Mode::Insert => self.process_insert_command(pressed_key),
            }
        }
    }

    /// React to a mouse event. If the mouse is being pressed, record
    /// the coordinates, and
    fn process_mouse_event(&mut self, mouse_event: MouseEvent) {
        match mouse_event {
            MouseEvent::Press(MouseButton::Left, _, _) => self.mouse_event_buffer.push(
                self.terminal
                    .get_cursor_index_from_mouse_event(mouse_event, self.cursor_position.x_offset),
            ),
            MouseEvent::Release(_, _) => {
                if !self.mouse_event_buffer.is_empty() {
                    self.cursor_position = self.mouse_event_buffer.pop().unwrap();
                }
            }
            _ => (),
        }
    }

    fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
        self.terminal.set_cursor_as_steady_bar();
    }

    fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
        self.terminal.set_cursor_as_steady_block();
    }

    fn start_receiving_command(&mut self) {
        self.command_buffer.push(COMMAND_PREFIX);
    }

    fn start_receiving_search_pattern(&mut self) {
        self.command_buffer.push(SEARCH_PREFIX);
    }

    fn stop_receiving_command(&mut self) {
        self.command_buffer = "".to_string();
    }

    fn is_receiving_command(&self) -> bool {
        !self.command_buffer.is_empty()
    }

    fn pop_normal_command_repetitions(&mut self) -> usize {
        let times = match self.normal_command_buffer.len() {
            0 => 1,
            _ => self
                .normal_command_buffer
                .join("")
                .parse::<usize>()
                .unwrap(),
        };
        self.normal_command_buffer = vec![];
        times
    }

    /// Receive a command entered by the user in the command prompt
    /// and take appropriate actions
    fn process_received_command(&mut self) {
        let command = self.command_buffer.clone();
        match self.command_buffer.chars().next().unwrap() {
            SEARCH_PREFIX => {
                self.process_search_command(command.strip_prefix(SEARCH_PREFIX).unwrap())
            }
            COMMAND_PREFIX => {
                let command = command.strip_prefix(COMMAND_PREFIX).unwrap_or_default();
                if command.is_empty() {
                } else if command.chars().all(char::is_numeric) {
                    // :n will get you to line n
                    let line_index = command.parse::<usize>().unwrap();
                    self.goto_line(line_index, 0);
                } else if command.split(' ').count() > 1 {
                    let cmd_tokens: Vec<&str> = command.split(' ').collect();
                    match cmd_tokens[0] {
                        commands::OPEN => {
                            if let Ok(document) = Document::open(cmd_tokens[1]) {
                                self.document = document;
                                self.reset_message();
                            } else {
                                self.display_message(utils::red(&format!(
                                    "{} not found",
                                    cmd_tokens[1]
                                )));
                            }
                        }
                        commands::NEW => {
                            self.document = Document::new_empty(cmd_tokens[1].to_string());
                            self.enter_insert_mode();
                        }
                        _ => (),
                    }
                } else {
                    match command {
                        commands::FORCE_QUIT => self.quit(true),
                        commands::QUIT => self.quit(false),
                        commands::LINE_NUMBERS => {
                            self.config.display_line_numbers =
                                Config::toggle(self.config.display_line_numbers);
                            self.cursor_position.x_offset = if self.config.display_line_numbers {
                                START_X
                            } else {
                                0
                            };
                        }
                        commands::STATS => {
                            self.config.display_stats = Config::toggle(self.config.display_stats);
                        }
                        commands::HELP => {
                            self.alternate_screen = true;
                        }
                        commands::SAVE => self.save(),
                        commands::SAVE_AND_QUIT => {
                            self.save();
                            self.quit(false);
                        }
                        _ => self
                            .display_message(utils::red(&format!("Unknown command '{}'", command))),
                    }
                }
            }
            _ => (),
        }
    }

    fn save(&mut self) {
        if self.document.save().is_ok() {
            self.display_message("File saved successfully".to_string());
            self.is_dirty = false;
        } else {
            self.display_message(utils::red("Error writing to file!"));
        }
    }

    fn quit(&mut self, force: bool) {
        if self.is_dirty && !force {
            self.display_message(utils::red("Unsaved changes! Run :q! to override"));
        } else {
            self.should_quit = true;
        }
    }

    fn process_search_command(&mut self, search_pattern: &str) {
        self.reset_search();
        for (row_index, row) in self.document.iter().enumerate() {
            if row.contains(search_pattern) {
                if let Some(match_start_index) = row.find(search_pattern) {
                    let match_start = Position {
                        x: match_start_index,
                        y: row_index.saturating_add(1), // terminal line number, 1-bases
                        x_offset: 0,
                    };
                    let match_end = Position {
                        x: match_start_index
                            .saturating_add(1)
                            .saturating_add(search_pattern.len()),
                        y: row_index.saturating_add(1),
                        x_offset: 0,
                    };
                    self.search_matches.push((match_start, match_end));
                }
            }
        }
        self.display_message(format!("{} matches", self.search_matches.len()));
        self.current_search_match_index = self.search_matches.len().saturating_sub(1);
        self.goto_next_search_match()
    }

    fn reset_search(&mut self) {
        self.search_matches = vec![]; // erase previous search matches
        self.current_search_match_index = 0;
    }

    fn revert_to_main_screen(&mut self) {
        self.reset_message();
        self.alternate_screen = false;
    }

    /// Process navigation command issued in normal mode, that will
    /// resolve in having the cursor be moved around the document.
    ///
    /// Note: some commands are accumulative (ie: 2j will move the
    /// cursor down twice) and some are not (ie: g will move the cursor
    /// to the start of the document only once).
    /// A buffer is maintained for the accumulative commands, and is purged
    /// when the last char of the command is received. For now, only commans
    /// of the form <number>*<char> are supported and I'm not sure I'm
    /// planning to support anything more complex than that.
    fn process_normal_command(&mut self, key: Key) {
        if key == Key::Esc {
            self.reset_message();
            self.reset_search();
        }
        if let Key::Char(c) = key {
            match c {
                '0' => {
                    if self.normal_command_buffer.is_empty() {
                        self.goto_start_or_end_of_line(&Boundary::Start);
                    } else {
                        self.normal_command_buffer.push(c.to_string());
                    }
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    self.normal_command_buffer.push(c.to_string())
                }
                'i' => self.enter_insert_mode(),
                ':' => self.start_receiving_command(),
                '/' => self.start_receiving_search_pattern(),
                'G' => self.goto_start_or_end_of_document(&Boundary::End),
                'g' => self.goto_start_or_end_of_document(&Boundary::Start),
                '$' => self.goto_start_or_end_of_line(&Boundary::End),
                '^' => self.goto_first_non_whitespace(),
                'H' => self.goto_first_line_of_terminal(),
                'M' => self.goto_middle_of_terminal(),
                'L' => self.goto_last_line_of_terminal(),
                'm' => self.goto_matching_closing_symbol(),
                'n' => self.goto_next_search_match(),
                'N' => self.goto_previous_search_match(),
                'q' => self.revert_to_main_screen(),
                'd' => self.delete_current_line(),
                'x' => self.delete_current_grapheme(),
                'o' => self.insert_newline_after_current_line(),
                'O' => self.insert_newline_before_current_line(),
                'A' => self.append_to_line(),
                _ => {
                    // at that point, we've iterated over all non accumulative commands
                    // meaning the command we're processing is an accumulative one.
                    // we thus pop the repeater value from self.normal_command_buffer
                    // and we use that value as the number of times the comamnd identified
                    // by the `c` char must be repeated.
                    let times = self.pop_normal_command_repetitions();
                    self.process_normal_command_n_times(c, times);
                }
            }
        };
    }

    /// Execute the provided normal movement command n timess
    fn process_normal_command_n_times(&mut self, c: char, n: usize) {
        match c {
            'b' => self.goto_start_or_end_of_word(&Boundary::Start, n),
            'w' => self.goto_start_or_end_of_word(&Boundary::End, n),
            'h' => self.move_cursor(&Direction::Left, n),
            'j' => self.move_cursor(&Direction::Down, n),
            'k' => self.move_cursor(&Direction::Up, n),
            'l' => self.move_cursor(&Direction::Right, n),
            '}' => self.goto_start_or_end_of_paragraph(&Boundary::End, n),
            '{' => self.goto_start_or_end_of_paragraph(&Boundary::Start, n),
            '%' => self.goto_percentage_in_document(n),
            _ => (),
        }
    }

    /// Process a command issued when the editor is in normal mode
    fn process_insert_command(&mut self, pressed_key: Key) {
        match pressed_key {
            Key::Esc => {
                self.enter_normal_mode();
                return;
            }
            Key::Backspace => {
                // When Backspace is pressed on the first column of a line, it means that we
                // should append the current line with the previous one
                if self.cursor_position.x == 0 {
                    if self.cursor_position.y > 0 {
                        let previous_line_len = self
                            .document
                            .get_row(self.cursor_position.y - 1)
                            .unwrap()
                            .len();
                        // Delete newline from previous row
                        self.document.delete(0, 0, self.current_row_index());
                        self.goto_x_y(previous_line_len, self.cursor_position.y - 1);
                    }
                } else {
                    // Delete previous character
                    self.document.delete(
                        self.current_x_position().saturating_sub(1),
                        self.current_x_position(),
                        self.current_row_index(),
                    );
                    self.move_cursor(&Direction::Left, 1);
                }
            }
            Key::Char('\n') => {
                self.document
                    .insert_newline(self.current_x_position(), self.current_row_index());
                self.goto_x_y(0, self.cursor_position.y + 1);
            }
            Key::Char('\t') => {
                for _ in 0..SPACES_PER_TAB {
                    self.document
                        .insert(' ', self.current_x_position(), self.current_row_index());
                }
                self.move_cursor(&Direction::Right, SPACES_PER_TAB);
            }
            Key::Char(c) => {
                self.document
                    .insert(c, self.current_x_position(), self.current_row_index());
                self.move_cursor(&Direction::Right, 1);
            }
            _ => (),
        }
        self.is_dirty = true;
    }

    /// Return the index of the row associated to the current cursor position / vertical offset
    fn current_row_index(&self) -> usize {
        self.cursor_position.y.saturating_add(self.offset.y)
    }

    fn current_x_position(&self) -> usize {
        self.cursor_position.x.saturating_add(self.offset.x)
    }

    /// Return the character currently under the cursor
    fn current_grapheme(&self) -> &str {
        self.current_row().index(self.current_x_position())
    }

    /// Return the line number associated to the current cursor position / vertical offset
    fn current_line_number(&self) -> usize {
        self.current_row_index().saturating_add(1)
    }

    /// Return the Row object associated to the current cursor position / vertical offset
    fn current_row(&self) -> &Row {
        self.document.get_row(self.current_row_index()).unwrap()
    }

    /// Delete the line currently under the cursor
    fn delete_current_line(&mut self) {
        self.document.delete_row(self.current_row_index());
        self.cursor_position.reset_x();
    }

    /// Delete the grapheme currently under the cursor
    fn delete_current_grapheme(&mut self) {
        self.document.delete(
            self.current_x_position(),
            self.current_x_position(),
            self.current_row_index(),
        );
    }

    /// Insert a newline after the current one, move cursor to it in insert mode
    fn insert_newline_after_current_line(&mut self) {
        let next_row_index = self.current_row_index().saturating_add(1);
        self.document
            .insert_newline(self.current_row().len(), self.current_row_index());
        self.goto_x_y(0, next_row_index);
        self.enter_insert_mode();
    }

    /// Insert a newline before the current one, move cursor to it in insert mode
    fn insert_newline_before_current_line(&mut self) {
        self.document.insert_newline(0, self.current_row_index());
        self.goto_x_y(0, self.current_row_index());
        self.enter_insert_mode();
    }

    fn append_to_line(&mut self) {
        self.goto_start_or_end_of_line(&Boundary::End);
        self.move_cursor(&Direction::Right, 1);
        self.enter_insert_mode();
    }

    /// Move the cursor to the next line after the current paraghraph, or the line
    /// before the current paragraph.
    fn goto_start_or_end_of_paragraph(&mut self, boundary: &Boundary, times: usize) {
        for _ in 0..times {
            let next_line_number = Navigator::find_line_number_of_start_or_end_of_paragraph(
                &self.document,
                self.current_line_number(),
                boundary,
            );
            self.goto_line(next_line_number, 0);
        }
    }

    /// Move the cursor either to the first or last line of the document
    fn goto_start_or_end_of_document(&mut self, boundary: &Boundary) {
        match boundary {
            Boundary::Start => self.goto_line(1, 0),
            Boundary::End => self.goto_line(self.document.last_line_number(), 0),
        }
    }

    /// Move the cursor either to the start or end of the line
    fn goto_start_or_end_of_line(&mut self, boundary: &Boundary) {
        match boundary {
            Boundary::Start => self.move_cursor_to_position_x(0),
            Boundary::End => {
                self.move_cursor_to_position_x(self.current_row().len().saturating_sub(1))
            }
        }
    }

    /// Move to the start of the next word or previous one.
    fn goto_start_or_end_of_word(&mut self, boundary: &Boundary, times: usize) {
        for _ in 0..times {
            let x = Navigator::find_index_of_next_or_previous_word(
                self.current_row(),
                self.current_x_position(),
                boundary,
            );
            self.move_cursor_to_position_x(x);
        }
    }

    /// Move the cursor to the first non whitespace character in the line
    fn goto_first_non_whitespace(&mut self) {
        if let Some(x) = Navigator::find_index_of_first_non_whitespace(&self.current_row()) {
            self.move_cursor_to_position_x(x);
        }
    }

    /// Move the cursor to the middle of the terminal
    fn goto_middle_of_terminal(&mut self) {
        self.goto_line(
            self.terminal
                .middle_of_screen_line_number()
                .saturating_add(self.offset.y)
                .saturating_add(1),
            0,
        );
    }

    /// Move the cursor to the middle of the terminal
    fn goto_first_line_of_terminal(&mut self) {
        self.goto_line(self.offset.y.saturating_add(1), 0);
    }

    /// Move the cursor to the last line of the terminal
    fn goto_last_line_of_terminal(&mut self) {
        self.goto_line(
            (self.terminal.size().height as usize)
                .saturating_add(self.offset.y)
                .saturating_add(1),
            0,
        );
    }

    /// Move to {n}% in the file
    fn goto_percentage_in_document(&mut self, percent: usize) {
        let percent = cmp::min(percent, 100);
        let line_number = (self.document.last_line_number() * percent) / 100;
        self.goto_line(line_number, 0)
    }

    /// Go to the matching closing symbol (whether that's a quote, curly/square/regular brace, etc).
    fn goto_matching_closing_symbol(&mut self) {
        let current_grapheme = self.current_grapheme();
        match current_grapheme {
            "\"" | "'" | "{" | "<" | "(" | "[" => {
                if let Some(position) = Navigator::find_matching_closing_symbol(
                    &self.document,
                    &self.cursor_position,
                    &self.offset,
                ) {
                    self.goto_x_y(position.x, position.y);
                }
            }
            "}" | ">" | ")" | "]" => {
                if let Some(position) = Navigator::find_matching_opening_symbol(
                    &self.document,
                    &self.cursor_position,
                    &self.offset,
                ) {
                    self.goto_x_y(position.x, position.y);
                }
            }
            _ => (),
        };
    }

    /// Move to the first character of the next search match
    fn goto_next_search_match(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }
        if self.current_search_match_index == self.search_matches.len().saturating_sub(1) {
            self.current_search_match_index = 0;
        } else {
            self.current_search_match_index = self.current_search_match_index.saturating_add(1);
        }
        self.display_message(format!(
            "Match {}/{}",
            self.current_search_match_index.saturating_add(1),
            self.search_matches.len()
        ));
        if let Some(search_match) = self.search_matches.get(self.current_search_match_index) {
            let x_position = search_match.0.x;
            let line_number = search_match.0.y;
            self.goto_line(line_number, x_position);
        }
    }

    /// Move to the first character of the previous search match
    fn goto_previous_search_match(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }
        if self.current_search_match_index == 0 {
            self.current_search_match_index = self.search_matches.len().saturating_sub(1);
        } else {
            self.current_search_match_index = self.current_search_match_index.saturating_sub(1);
        }
        self.display_message(format!(
            "Match {}/{}",
            self.current_search_match_index.saturating_add(1),
            self.search_matches.len()
        ));
        if let Some(search_match) = self.search_matches.get(self.current_search_match_index) {
            let line_number = search_match.0.y;
            let x_position = search_match.0.x;
            self.goto_line(line_number, x_position);
        }
    }

    /// Move the cursor to the nth line in the file and adjust the viewport
    fn goto_line(&mut self, line_number: usize, x_position: usize) {
        let y = line_number.saturating_sub(1);
        self.goto_x_y(x_position, y);
    }

    /// Move the cursor to the first column of the nth line
    fn goto_x_y(&mut self, x: usize, y: usize) {
        self.move_cursor_to_position_x(x);
        self.move_cursor_to_position_y(y);
    }

    /// Move the cursor up/down/left/right by adjusting its x/y position
    fn move_cursor(&mut self, direction: &Direction, times: usize) {
        let size = self.terminal.size();
        let term_height = size.height.saturating_sub(1) as usize;
        let term_width = size.width.saturating_sub(1) as usize;
        let Position {
            mut x,
            mut y,
            x_offset: _,
        } = self.cursor_position;

        let Position {
            x: mut offset_x,
            y: mut offset_y,
            x_offset: _,
        } = self.offset;

        for _ in 0..times {
            match direction {
                Direction::Up => {
                    if y == 0 {
                        // we reached the top of the terminal so adjust offset instead
                        offset_y = offset_y.saturating_sub(1);
                    } else {
                        y = y.saturating_sub(1);
                    }
                } // cannot be < 0
                Direction::Down => {
                    if y.saturating_add(offset_y)
                        < self.document.last_line_number().saturating_sub(1)
                    {
                        // don't scroll past the last line in the document
                        if y < term_height {
                            // don't scroll past the confine the of terminal itself
                            y = y.saturating_add(1);
                        } else {
                            // increase offset to that scrolling adjusts the viewport
                            offset_y = offset_y.saturating_add(1);
                        }
                    }
                }
                Direction::Left => {
                    if x >= term_width {
                        offset_x = offset_x.saturating_sub(1);
                    } else {
                        x = x.saturating_sub(1);
                    }
                }
                Direction::Right => {
                    if x.saturating_add(offset_x) <= self.current_row().len().saturating_sub(1) {
                        if x < term_width {
                            x = x.saturating_add(1);
                        } else {
                            offset_x = offset_x.saturating_add(1)
                        }
                    }
                }
            }
        }
        self.cursor_position.x = x;
        self.cursor_position.y = y;
        self.offset.x = offset_x;
        self.offset.y = offset_y;
    }

    fn move_cursor_to_position_y(&mut self, y: usize) {
        let max_line_number = self.document.last_line_number(); // last line number in the document
        let term_height = self.terminal.size().height as usize;
        let middle_of_screen_line_number = self.terminal.middle_of_screen_line_number(); // number of the line in the middle of the terminal

        let y = cmp::max(0, y);
        let y = cmp::min(y, max_line_number);
        if y < middle_of_screen_line_number {
            // move to the first "half-view" of the document
            self.offset.y = 0;
            self.cursor_position.y = y;
        } else if y > max_line_number - middle_of_screen_line_number {
            // move to the last "half view" of the document
            self.offset.y = max_line_number - term_height;
            self.cursor_position.y = y.saturating_sub(self.offset.y);
        } else if self.offset.y <= y && y <= self.offset.y + term_height {
            // move around in the same view
            self.cursor_position.y = y.saturating_sub(self.offset.y);
        } else {
            // move to another view in the document, and position the cursor at the
            // middle of the terminal/view.
            self.offset.y = y - middle_of_screen_line_number;
            self.cursor_position.y = middle_of_screen_line_number;
        }
    }

    fn move_cursor_to_position_x(&mut self, x: usize) {
        let term_width = self.terminal.size().width as usize;
        let x = cmp::max(0, x);
        if x > term_width {
            self.cursor_position.x = term_width - 1;
            self.offset.x = x - term_width - self.offset.x + 1;
        } else {
            self.cursor_position.x = x;
            self.offset.x = 0;
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.terminal.hide_cursor();
        if !self.should_quit {
            if self.alternate_screen {
                self.terminal.clear_all();
                self.terminal.to_alternate_screen();
                self.draw_help_screen();
            } else {
                self.terminal.to_main_screen();
                self.draw_rows();
            }
            self.draw_status_bar();
            self.draw_message_bar();
            if self.alternate_screen {
                self.terminal.set_cursor_position(&Position::top_left());
            } else {
                self.terminal.set_cursor_position(&self.cursor_position);
            }
        }
        self.terminal.show_cursor();
        self.terminal.flush()
    }

    fn generate_status(&self) -> String {
        let dirty_marker = if self.is_dirty { " +" } else { "" };
        let left_status = format!("[{}]{} {}", self.document.filename, dirty_marker, self.mode);
        let stats = if self.config.display_stats {
            format!(
                "[{}L/{}W]",
                self.document.last_line_number(),
                self.document.num_words()
            )
        } else {
            "".to_string()
        };
        let position = format!(
            "Ln {}, Col {}",
            self.current_line_number(),
            self.cursor_position
                .x
                .saturating_add(self.offset.x)
                .saturating_add(1),
        );
        let right_status = format!("{} {}", stats, position);
        let right_status = right_status.trim_start();
        let spaces = " "
            .repeat(self.terminal.size().width as usize - left_status.len() - right_status.len());
        format!("{}{}{}\r", left_status, spaces, right_status)
    }

    fn draw_status_bar(&self) {
        self.terminal.set_bg_color(STATUS_BG_COLOR);
        self.terminal.set_fg_color(STATUS_FG_COLOR);
        println!("{}", self.generate_status());
        self.terminal.reset_fg_color();
        self.terminal.reset_bg_color();
    }

    fn draw_message_bar(&self) {
        self.terminal.clear_current_line();
        if self.is_receiving_command() {
            print!("{}\r", self.command_buffer)
        } else {
            print!("{}\r", self.message);
        }
    }

    fn display_message(&mut self, message: String) {
        self.message = message;
    }

    fn reset_message(&mut self) {
        self.message = "".to_string();
    }

    fn display_welcome_message(&self) {
        let term_width = self.terminal.size().width as usize;
        let welcome_msg = format!("{} v{}", PKG, VERSION);
        let padding_len = (term_width - welcome_msg.chars().count() - 2) / 2; // -2 because of the starting '~ '
        let padding = String::from(" ").repeat(padding_len);
        let mut padded_welcome_message = format!("~ {}{}{}", padding, welcome_msg, padding);
        padded_welcome_message.truncate(term_width); // make it fit on screen
        println!("{}\r", padded_welcome_message);
    }

    fn draw_help_screen(&mut self) {
        let help_text = "Normal commands\r\n  \
                            j => move cursor down one row (<n>j moves it by n rows)\r\n  \
                            k => move cursor up one row (<n>k moves it by n rows)\r\n  \
                            h => move cursor left (<n>h moves it n times)\r\n  \
                            l => move cursor right (<n>l moves it n times)\r\n  \
                            } => move to the end of the current paragraph (<n>} moves n times)\r\n  \
                            { => move to the start of the current paragraph (<n>{ moves n times)\r\n  \
                            w => move to the end of the current word (<n>w moves n times)\r\n  \
                            b => move to the start of the current word (<n>b moves n times)\r\n  \
                            i => switch to insert mode\r\n  \
                            g => go to beginining of document\r\n  \
                            G => go to end of document\r\n  \
                            0 => go to first character in line\r\n  \
                            ^ => go to first non-whitespace character in line\r\n  \
                            $ => go to end of line\r\n  \
                            H => go to first line in screen\r\n  \
                            M => go to line in the middle of the screen\r\n  \
                            L => go to last line in screen\r\n  \
                           n% => move to n% in the file\r\n  \
                            / => open search prompt\r\n  \
                            n => go to next search match\r\n  \
                            N => go to previous search match\r\n  \
                            d => delete current line\r\n  \
                            x => delete current character\r\n  \
                            o => insert newline after current line & enter insert mode\r\n  \
                            O => insert newline before current line & enter insert mode\r\n  \
                            A => go to end of line & enter insert moder\n  \
                            : => open command prompt\r\n\n\
                        Prompt commands\r\n  \
                            help            => display this help screen\r\n  \
                            ln              => toggle line numbers\r\n  \
                            new  <filename> => open a new file\r\n  \
                            open <filename> => open a file\r\n  \
                            q               => quit bo\r\n  \
                            stats           => toggle line/word stats\r\n  \
                            w               => save\r\n  \
                            wq              => save and quit\r\n\n\
                        Insert commands\r\n  \
                            Esc => go back to normal mode";
        let help_text_lines = help_text.split('\n');
        let help_text_lines_count = help_text_lines.count();
        let term_height = self.terminal.size().height;
        let v_padding = (term_height - 2 - help_text_lines_count as u16) / 2;
        let max_line_length = help_text.split('\n').map(str::len).max().unwrap();
        let h_padding = " ".repeat((self.terminal.size().width as usize - max_line_length) / 2);
        for _ in 0..=v_padding {
            println!("\r");
        }
        for line in help_text.split('\n') {
            println!("{}{}\r", h_padding, line);
        }
        for _ in 0..=v_padding {
            println!("\r");
        }
        if (v_padding + help_text_lines_count as u16 + v_padding) == (term_height - 1) {
            println!("\r");
        }
        self.display_message("Press q to quit".to_string());
    }

    fn draw_rows(&self) {
        let term_height = self.terminal.size().height;
        for terminal_row_idx in self.offset.y..(term_height as usize + self.offset.y) {
            let line_number = terminal_row_idx.saturating_add(1);
            self.terminal.clear_current_line();
            if let Some(row) = self.document.get_row(terminal_row_idx) {
                self.draw_row(&row, line_number);
            } else if terminal_row_idx == self.terminal.middle_of_screen_line_number()
                && self.document.filename.is_empty()
            {
                self.display_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row(&self, row: &Row, line_number: usize) {
        let row_visible_start = self.offset.x;
        let mut row_visible_end = self.terminal.size().width as usize + self.offset.x;
        if self.cursor_position.x_offset > 0 {
            row_visible_end = row_visible_end
                .saturating_sub(self.cursor_position.x_offset as usize)
                .saturating_sub(1)
        }
        let rendered_row = row.render(
            row_visible_start,
            row_visible_end,
            line_number,
            self.cursor_position.x_offset as usize,
        );
        println!("{}\r", rendered_row);
    }
}

#[cfg(test)]
#[path = "./editor_test.rs"]
mod editor_test;
