#![warn(clippy::all, clippy::pedantic)]

mod editor;
use editor::Editor;
mod terminal;
pub use terminal::Terminal;
pub use editor::Position;
mod document;
pub use document::Document;
mod row;
pub use row::Row;

fn main() {
    Editor::default().run();
}