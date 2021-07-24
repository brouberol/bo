#![warn(clippy::all, clippy::pedantic)]

mod commands;
mod document;
mod editor;
mod mode;
mod navigator;
mod row;
mod terminal;
mod utils;

use editor::Editor;

pub use document::Document;
pub use editor::Position;
pub use mode::Mode;
pub use navigator::{Boundary, Navigator};
pub use row::Row;
pub use terminal::Terminal;
pub use utils::log;

fn main() {
    Editor::default().run();
}
