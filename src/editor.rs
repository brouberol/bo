#[cfg(debug_assertions)]
use crate::log;
use crate::{Document, Mode, Row, Terminal};
use std::cmp;
use std::env;
use std::io::{self, stdout};
use termion::color;
use termion::event::Key;
use termion::raw::IntoRawMode;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG: &str = env!("CARGO_PKG_NAME");
const COMMAND_PREFIX: char = ':';
const LINE_NUMBER_OFFSET: u8 = 4;
const START_X: usize = LINE_NUMBER_OFFSET as usize + 1;

#[derive(Debug, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
    message: String,
    mode: Mode,
    command_buffer: String,
}

#[derive(PartialEq)]
enum Boundary {
    Start,
    End,
}

fn die(e: &io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();

        let document: Document = match args.len() {
            1 => Document::default(),
            2 => Document::open(&args[1]).unwrap_or_default(),
            _ => panic!("Can't (yet) open multiple files."),
        };
        let start_position: Position = match args.len() {
            2 => Position { x: START_X, y: 0 },
            _ => Position::default(),
        };
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: start_position,
            document,
            offset: Position::default(),
            message: "".to_string(),
            mode: Mode::Normal,
            command_buffer: "".to_string(),
        }
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = &self.refresh_screen() {
                die(&error);
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
            if self.should_quit {
                Terminal::clear_screen();
                break;
            }
        }
    }

    fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
    }

    fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    fn start_receiving_command(&mut self) {
        self.command_buffer.push(COMMAND_PREFIX);
    }

    fn stop_receiving_command(&mut self) {
        self.command_buffer = "".to_string();
    }

    fn is_receiving_command(&self) -> bool {
        !self.command_buffer.is_empty()
    }

    fn process_received_command(&mut self) {
        let command = self.command_buffer.clone();
        let command = command.strip_prefix(COMMAND_PREFIX).unwrap_or_default();
        if command.chars().all(char::is_numeric) && !command.is_empty() {
            let line_index = command.parse::<usize>().unwrap();
            self.goto_line(line_index);
        }
        match command {
            "q" => {
                self.should_quit = true;
            }
            _ => (),
        }
    }

    fn process_normal_command(&mut self, key: Key) {
        match key {
            Key::Char('h' | 'j' | 'k' | 'l') => self.move_cursor(key),
            Key::Char('i') => self.enter_insert_mode(),
            Key::Char(':') => self.start_receiving_command(),
            Key::Char('}') => self.goto_start_or_end_of_paragraph(&Boundary::End),
            Key::Char('{') => self.goto_start_or_end_of_paragraph(&Boundary::Start),
            Key::Char('G') => self.goto_start_or_end_of_document(&Boundary::End),
            Key::Char('g') => self.goto_start_or_end_of_document(&Boundary::Start),
            _ => (),
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        if self.is_receiving_command() {
            // accumulate the command in the command buffer
            match pressed_key {
                Key::Esc => self.stop_receiving_command(),
                Key::Char('\n') => {
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
                Mode::Insert => match pressed_key {
                    Key::Esc => self.enter_normal_mode(),
                    _ => (),
                },
            }
            self.scroll();
        }
        #[cfg(debug_assertions)]
        log(format!(
            "{:?} Offset= {:?}",
            self.cursor_position, self.offset
        ));
        Ok(())
    }

    fn current_line_number(&self) -> usize {
        self.cursor_position.y + self.offset.y + 1
    }

    fn last_line_number(&self) -> usize {
        self.document.len()
    }

    fn goto_start_or_end_of_paragraph(&mut self, boundary: &Boundary) {
        let mut current_line_number = self.current_line_number();
        let last_line_number = self.last_line_number();
        loop {
            current_line_number = match boundary {
                Boundary::Start => cmp::max(1, current_line_number.saturating_sub(1)),
                Boundary::End => cmp::min(last_line_number, current_line_number.saturating_add(1)),
            };
            if current_line_number == self.last_line_number()
                || current_line_number == 1
                || self
                    .document
                    .get_row(current_line_number.saturating_sub(1)) // rows inddices are 0 based
                    .unwrap()
                    .is_whitespace()
            {
                self.goto_line(current_line_number);
                return;
            }
        }
    }

    fn goto_start_or_end_of_document(&mut self, boundary: &Boundary) {
        match boundary {
            Boundary::Start => self.goto_line(1),
            Boundary::End => self.goto_line(self.last_line_number()),
        }
    }

    fn set_cursor_position_by_line_number(&mut self, x: usize, line_number: usize) {
        self.cursor_position = Position {
            x,
            y: line_number.saturating_sub(1),
        };
    }

    fn goto_line(&mut self, line_number: usize) {
        /*
            We want to move to the line `line_number`. If that line is
            out of the view, we need to adjust offset to make sure that we end up
            at the middle of the terminal. If the line is within the same view,
            we just move the cursor.
        */
        let max_line_number = self.last_line_number(); // last line number in the document
        let line_number = cmp::min(max_line_number, line_number); // we can't go after the last line
        let line_number = cmp::max(1, line_number); // line 0 is line 1, for the same reason
        let term_height = self.terminal.size().height as usize;
        let middle_of_screen_line_number = term_height / 2; // number of the row in the middle of the terminal

        if line_number < middle_of_screen_line_number {
            // move to the first "half-view" of the document
            self.offset.y = 0;
            self.set_cursor_position_by_line_number(START_X, line_number);
        } else if line_number > max_line_number - middle_of_screen_line_number {
            // move to the last "half view" of the document
            self.offset.y = max_line_number - term_height;
            self.set_cursor_position_by_line_number(START_X, line_number - self.offset.y);
        } else if self.offset.y <= line_number && line_number <= self.offset.y + term_height {
            // move around in the same view
            self.set_cursor_position_by_line_number(START_X, line_number - self.offset.y);
        } else {
            // move to another view in the document, and position the cursor at the
            // middle of the terminal/view.
            self.offset.y = line_number - middle_of_screen_line_number;
            self.set_cursor_position_by_line_number(START_X, middle_of_screen_line_number);
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let size = self.terminal.size();
        let term_height = size.height.saturating_sub(1) as usize;
        let term_width = size.width.saturating_sub(1) as usize;
        let Position { mut x, mut y } = self.cursor_position;
        match key {
            Key::Up | Key::Char('k') => {
                y = y.saturating_sub(1);
            } // cannot be < 0
            Key::Down | Key::Char('j') => {
                if y < term_height && y < self.document.len() {
                    // don't scroll past the last line
                    y = y.saturating_add(1);
                }
            }
            Key::Left | Key::Char('h') => x = cmp::max(x.saturating_sub(1), START_X), // cannot be < 0
            Key::Right | Key::Char('l') => {
                if x < term_width {
                    x = x.saturating_add(1);
                }
            }
            _ => (),
        }
        self.cursor_position = Position { x, y };
    }

    fn scroll(&mut self) {
        let current_position_y = self.cursor_position.y;
        let term_height = self.terminal.size().height as usize;
        if current_position_y == 0 && self.offset.y > 0 {
            // if we're going out of the view by the top scroll up by one row
            self.offset.y = self.offset.y.saturating_sub(1);
        } else if current_position_y.saturating_add(1) > term_height {
            // if we're going out of the view by the bottom, scroll down by one row
            self.offset.y = self.offset.y.saturating_add(1);
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::set_cursor_position(&Position { x: 0, y: 0 });
        if !self.should_quit {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::set_cursor_position(&self.cursor_position);
        }
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn generate_status(&self) -> String {
        let left_status = format!("[{}] {}", self.document.filename, self.mode);
        let right_status = format!("{}:{}", self.cursor_position.x, self.cursor_position.y);
        let spaces = " "
            .repeat(self.terminal.size().width as usize - left_status.len() - right_status.len());
        format!("{}{}{}\r", left_status, spaces, right_status)
    }

    fn draw_status_bar(&self) {
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}", self.generate_status());
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        if self.is_receiving_command() {
            print!("{}\r", self.command_buffer)
        } else {
            print!("{}\r", self.message);
        }
    }

    fn display_message(&mut self, message: String) {
        self.message = message;
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

    fn draw_rows(&self) {
        let term_height = self.terminal.size().height;
        for terminal_row_idx in self.offset.y..(term_height as usize + self.offset.y) {
            let line_number = terminal_row_idx.saturating_add(1);
            Terminal::clear_current_line();
            if let Some(row) = self.document.get_row(terminal_row_idx) {
                self.draw_row(&row, line_number);
            } else if terminal_row_idx == (term_height as usize / 2) && self.document.is_empty() {
                self.display_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row(&self, row: &Row, line_number: usize) {
        let row_visible_start = self.offset.x;
        let row_visible_end = self.offset.y + self.terminal.size().width as usize;
        let rendered_row = row.render(
            row_visible_start,
            row_visible_end,
            line_number,
            LINE_NUMBER_OFFSET as usize,
        );
        println!("{}\r", rendered_row);
    }
}
