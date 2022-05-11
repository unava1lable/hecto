#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod row;
mod document;

pub use row::Row;
pub use document::Document;
pub use terminal::Terminal;
pub use editor::Position;

use editor::Editor;
fn main() {
	Editor::default().run();
}