//! Eddy
//! A lightweight text editor for the terminal.
//! Written in Rust for Linux, MacOS and Windows.
//! (c) 2026 by Markus Müller
mod editor;

fn main() -> Result<(), i32> {
    crate::editor::run()
}
