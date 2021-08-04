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

pub struct Terminal {
    _stdout: AlternateScreen<MouseTerminal<RawTerminal<std::io::Stdout>>>,
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
    fn read_event(&self) -> Result<Event, std::io::Error> {
        loop {
            let opt_key = io::stdin().lock().events().next();
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

    fn set_cursor_position(&self, position: &Position) {
        let Position {
            mut x,
            mut y,
            mut x_offset,
        } = position;
        // hiding the fact that the terminal position is 1-based, while preventing an overflow
        x_offset += if x_offset > 0 { 1 } else { 0 };
        x = x.saturating_add(1);
        x = cmp::min(x.saturating_add(x_offset.into()), self.size().width.into());
        y = y.saturating_add(1);
        y = cmp::min(y, self.size().height.into());
        print!("{}", termion::cursor::Goto(x as u16, y as u16));
    }

    #[must_use]
    fn get_cursor_index_from_mouse_event(&self, mouse_event: MouseEvent, x_offset: u8) -> Position {
        if let MouseEvent::Press(_, x, y) = mouse_event {
            let offset_adjustment: u8 = if x_offset > 0 {
                x_offset.saturating_add(1)
            } else {
                0
            };
            Position {
                x: x.saturating_sub(1)
                    .saturating_sub(u16::from(offset_adjustment)) as usize,
                y: y.saturating_sub(1) as usize,
                x_offset,
            }
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
        Ok(Self {
            _stdout: AlternateScreen::from(MouseTerminal::from(stdout().into_raw_mode()?)),
        })
    }
}
