use crate::{Document, Row, Terminal};
use std::env;
use std::io::{self, stdout};
use termion::event::Key;
use termion::raw::IntoRawMode;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG: &str = env!("CARGO_PKG_NAME");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    message: String,
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
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document,
            message: "".to_string(),
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
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let size = self.terminal.size();
        let term_height = size.height.saturating_sub(1) as usize;
        let term_width = size.width.saturating_sub(1) as usize;
        let Position { mut x, mut y } = self.cursor_position;
        match key {
            Key::Up => y = y.saturating_sub(1), // cannot be < 0
            Key::Down => {
                if y < term_height && y < self.document.len() {
                    // don't scroll past the last line
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1), // cannot be < 0
            Key::Right => {
                if x < term_width {
                    x = x.saturating_add(1);
                }
            }
            _ => (),
        }
        self.cursor_position = Position { x, y };
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::set_cursor_position(&Position::default());
        if !self.should_quit {
            self.draw_rows();
            self.draw_message_bar();
            Terminal::set_cursor_position(&self.cursor_position);
        }
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        print!("{}\r", self.message);
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
        for terminal_row_idx in 0..term_height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.get_row(terminal_row_idx as usize) {
                self.draw_row(&row);
            } else if terminal_row_idx == term_height / 2 && self.document.is_empty() {
                self.display_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row(&self, row: &Row) {
        let row_visible_start = 0;
        let row_visible_end = self.terminal.size().width as usize;
        let rendered_row = row.render(row_visible_start, row_visible_end);
        println!("{}\r", rendered_row);
    }
}
