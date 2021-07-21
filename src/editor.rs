use crate::{Document, Row, Terminal};
use std::env;
use std::io::{self, stdout};
use termion::color;
use termion::event::Key;
use termion::raw::IntoRawMode;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG: &str = env!("CARGO_PKG_NAME");

enum Mode {
    Insert,
    Normal,
}

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
    offset: Position,
    message: String,
    mode: Mode,
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
            offset: Position::default(),
            message: "".to_string(),
            mode: Mode::Normal,
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

    fn process_normal_command(&mut self, key: Key) {
        match key {
            Key::Char('h' | 'j' | 'k' | 'l') => self.move_cursor(key),
            _ => (),
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match self.mode {
            Mode::Normal => self.process_normal_command(pressed_key),
            _ => (),
        }
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let size = self.terminal.size();
        let term_height = size.height.saturating_sub(1) as usize;
        let term_width = size.width.saturating_sub(1) as usize;
        let Position { mut x, mut y } = self.cursor_position;
        match key {
            Key::Up | Key::Char('k') => y = y.saturating_sub(1), // cannot be < 0
            Key::Down | Key::Char('j') => {
                if y < term_height && y < self.document.len() {
                    // don't scroll past the last line
                    y = y.saturating_add(1);
                }
            }
            Key::Left | Key::Char('h') => x = x.saturating_sub(1), // cannot be < 0
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
        let y = self.cursor_position.y;
        let term_height = self.terminal.size().height as usize;
        if y == 0 && self.offset.y > 0 {
            self.offset.y = self.offset.y.saturating_sub(1);
        } else if y + 1 >= term_height {
            self.offset.y = self.offset.y.saturating_add(1);
        }

        #[cfg(debug_assertions)]
        self.display_message(format!(
            "y={}, offset.y={}, total.y={}",
            y,
            self.offset.y,
            self.offset.y.saturating_add(y)
        ))
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::set_cursor_position(&Position::default());
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
        let left_status = format!("[{}]", self.document.filename);
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
        for terminal_row_idx in 0..term_height {
            Terminal::clear_current_line();
            if let Some(row) = self
                .document
                .get_row(terminal_row_idx as usize + self.offset.y)
            {
                self.draw_row(&row);
            } else if terminal_row_idx == term_height / 2 && self.document.is_empty() {
                self.display_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row(&self, row: &Row) {
        let row_visible_start = self.offset.x;
        let row_visible_end = self.offset.y + self.terminal.size().width as usize;
        let rendered_row = row.render(row_visible_start, row_visible_end);
        println!("{}\r", rendered_row);
    }
}
