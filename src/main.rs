#![warn(clippy::all, clippy::pedantic)]

mod editor;
use editor::Editor;
mod terminal;
pub use terminal::Terminal;
pub use editor::Position;

fn main() {
    Editor::default().run();
}