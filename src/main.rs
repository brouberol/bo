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
use structopt::StructOpt;

pub use document::Document;
pub use editor::Position;
pub use mode::Mode;
pub use navigator::{Boundary, Navigator};
pub use row::Row;
pub use terminal::Terminal;
pub use utils::log;

#[derive(Debug, StructOpt)]
#[structopt(name = "bo", about = "An opinionated text editor")]
struct Opt {
    /// Version flag
    #[structopt(long)]
    version: bool,

    /// File name
    #[structopt(name = "FILE")]
    file_name: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    if opt.version {
        println!("{}", env!("CARGO_PKG_VERSION"));
    } else {
        Editor::default(opt.file_name).run();
    }
}
