use crate::Position;
use core::fmt;
use std::io::Error;
use termion::color;
use termion::event::{Event, MouseEvent};

#[derive(Debug)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub trait Console {
    /// Read the next event from the console input.termion
    ///
    /// # Errors
    /// Will return an error if an event can't be read from the console input.
    fn read_event(&self) -> Result<Event, Error>;

    fn clear_screen(&self);

    fn clear_current_line(&self);

    fn flush(&self) -> Result<(), Error>;

    fn hide_cursor(&self);

    fn show_cursor(&self);

    fn set_bg_color(&self, color: color::Rgb);

    fn reset_bg_color(&self);

    fn set_fg_color(&self, color: color::Rgb);

    fn reset_fg_color(&self);

    fn to_alternate_screen(&self);

    fn to_main_screen(&self);

    fn clear_all(&self);

    fn size(&self) -> &Size;

    fn middle_of_screen_line_number(&self) -> usize;

    fn get_cursor_index_from_mouse_event(&self, mouse_event: MouseEvent, x_offset: u8) -> Position;

    fn set_cursor_position(&self, position: &Position);
}

impl fmt::Debug for dyn Console {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Console").finish()
    }
}
