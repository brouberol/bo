use crate::{Console, Position, Size};
use std::cmp;
use std::fmt;
use std::io::{self, stdout, Write};
use termion::color;
use termion::cursor::{SteadyBar, SteadyBlock};
use termion::event::{Event, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, ToAlternateScreen, ToMainScreen};

#[derive(Debug, PartialEq)]
pub struct AnsiPosition {
    pub x: u16,
    pub y: u16,
}

impl From<Position> for AnsiPosition {
    #[allow(clippy::cast_possible_truncation)]
    fn from(p: Position) -> Self {
        Self {
            x: (p.x as u16).saturating_add(1),
            y: (p.y as u16).saturating_add(1),
        }
    }
}

pub struct Terminal {
    _stdout: AlternateScreen<MouseTerminal<RawTerminal<std::io::Stdout>>>,
    stdin_event_stream: termion::input::Events<io::Stdin>,
}

impl fmt::Debug for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Terminal").finish()
    }
}

impl Console for Terminal {
    fn clear_screen(&self) {
        print!("{}", termion::clear::All);
    }

    fn clear_current_line(&self) {
        print!("{}", termion::clear::CurrentLine);
    }

    /// # Errors
    ///
    /// Returns an error if stdout can't be flushed
    fn flush(&self) -> Result<(), std::io::Error> {
        std::io::stdout().flush()
    }

    /// # Errors
    ///
    /// Returns an error if a event can't be read
    fn read_event(&mut self) -> Result<Event, std::io::Error> {
        loop {
            let opt_key = self.stdin_event_stream.next();
            // at that point, event is a Result<Event, Error>, as the Option was unwrapped
            if let Some(event) = opt_key {
                return event;
            }
        }
    }

    fn hide_cursor(&self) {
        print!("{}", termion::cursor::Hide);
    }

    fn show_cursor(&self) {
        print!("{}", termion::cursor::Show);
    }

    fn set_bg_color(&self, color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    fn reset_bg_color(&self) {
        print!("{}", color::Bg(color::Reset));
    }

    fn set_fg_color(&self, color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    fn reset_fg_color(&self) {
        print!("{}", color::Fg(color::Reset));
    }

    fn to_alternate_screen(&self) {
        print!("{}", ToAlternateScreen);
    }

    fn to_main_screen(&self) {
        print!("{}", ToMainScreen);
    }

    fn clear_all(&self) {
        print!("{}", termion::clear::All);
    }

    fn size(&self) -> Size {
        Size::from(termion::terminal_size().unwrap_or_default())
    }

    fn middle_of_screen_line_number(&self) -> usize {
        self.size().height as usize / 2
    }

    fn set_cursor_position(&self, position: &Position, mut row_prefix_length: u8) {
        let ansi_position = AnsiPosition::from(*position);
        // hiding the fact that the terminal position is 1-based, while preventing an overflow
        row_prefix_length += if row_prefix_length > 0 { 1 } else { 0 };
        print!(
            "{}",
            termion::cursor::Goto(
                cmp::min(
                    ansi_position.x.saturating_add(row_prefix_length.into()),
                    self.size().width
                ),
                cmp::min(ansi_position.y, self.size().height)
            )
        );
    }

    #[must_use]
    fn get_cursor_index_from_mouse_event(
        &self,
        mouse_event: MouseEvent,
        row_prefix_length: u8,
    ) -> Position {
        if let MouseEvent::Press(_, x, y) = mouse_event {
            let offset_adjustment: u8 = if row_prefix_length > 0 {
                row_prefix_length.saturating_add(1)
            } else {
                0
            };
            let ansi_position = AnsiPosition {
                x: x.saturating_sub(u16::from(offset_adjustment)),
                y,
            };
            Position::from(ansi_position)
        } else {
            Position::top_left()
        }
    }

    fn set_cursor_as_steady_bar(&self) {
        print!("{}", SteadyBar);
    }

    fn set_cursor_as_steady_block(&self) {
        print!("{}", SteadyBlock);
    }
}

impl Terminal {
    /// # Errors
    ///
    /// will return an error if the terminal size can't be acquired
    /// or if the stdout cannot be put into raw mode.
    pub fn default() -> Result<Self, std::io::Error> {
        let mut term_stdout = stdout();
        write!(term_stdout, "{}", termion::cursor::Goto(1, 1))?;
        term_stdout.flush()?;
        Ok(Self {
            _stdout: AlternateScreen::from(MouseTerminal::from(term_stdout.into_raw_mode()?)),
            stdin_event_stream: io::stdin().events(),
        })
    }
}

#[cfg(test)]
#[path = "./terminal_test.rs"]
mod terminal_test;
