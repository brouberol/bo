#![warn(clippy::all, clippy::pedantic)]

mod document;
mod editor;
mod mode;
mod row;
mod terminal;

use editor::Editor;

pub use document::Document;
pub use editor::Position;
pub use mode::Mode;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
