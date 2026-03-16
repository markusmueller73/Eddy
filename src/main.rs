//! Eddy
//! A lightweight text editor for the terminal.
//! The app is written in Rust and should work for Linux, MacOS and Windows.

pub const TITLE: &str = "Eddy";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

mod editor;

fn main() -> Result<(), i32> {
    crate::editor::Editor::run()
}
