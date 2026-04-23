// Part of Eddy - A lightweight text editor for the terminal.
#[macro_use]
mod position;
mod buffer;
mod color_pairs;
mod history;
mod row;
mod settings;
mod status_input;
mod status_message;
pub mod text_editor;
mod view;

pub const TITLE: &str = "Eddy";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
