// Part of Eddy - a lightweight text editor for the terminal.
//! Editor module.
//! This module contains the whole editor logic. Manage the
//! terminal view, the document state and the user interactions.

use crossterm::terminal;

use crate::{TITLE, VERSION};

mod buffer;
mod document;
mod row;
mod view;

pub struct Editor {
    document: document::Document,
    view: view::View,
}

impl Editor {

    /// Create a new editor instance.
    fn new() -> Self {
        Self {
            document: document::Document::new(),
            view: view::View::new(),
        }
    }

    /// Run the editor, this is the only public method that should be called to start the editor.
    pub fn run() -> Result<(), i32> {
        // Create new instance
        let mut editor = Self::new();
        // Check for command line arguments
        let current_file = Self::command_line_arguments();
        if !current_file.is_empty() {
            println!("{}: loading '{}'", TITLE, current_file);
            editor.document.load(&current_file);
        }
        editor.view.render();
        // 'main_loop: loop {
        //     editor.view.render();
        std::thread::sleep(std::time::Duration::from_secs(3));
        // }
        editor.view.exit();
        Ok(())
    }

    /// Parse command line arguments and return the current file path.
    fn command_line_arguments() -> String {
        // Get the first command line argument, it is by default the name of the executeable
        let exe = std::env::args().nth(0).unwrap_or(String::from("eddy"));
        // Create a String vector from the rest of the command line arguments...
        let argv: Vec<String> = std::env::args().skip(1).collect();
        // ...and check if any
        if argv.is_empty() {
            return String::new();
        }
        let mut filename = String::new();
        // iter through all arguments
        for arg in argv {
            if arg.starts_with("-") {
                // Check command line parameter
                match arg.as_str() {
                    "-h" | "--help" => {
                        println!("Usage: {} [options] [file]", exe);
                        println!("Options:");
                        println!("  -h, --help    Display this help message");
                        println!("  -V, --version Show the version number");
                        std::process::exit(0);
                    }
                    "-V" | "--version" => {
                        println!("{} Version {}", TITLE, VERSION);
                        std::process::exit(0);
                    }
                    _ => {
                        println!("Unknown option: {}", arg);
                        std::process::exit(0);
                    }
                }
            } else {
                // Append all arguments, the user may type the filename with spaces...
                // ... and without (double)quotes
                filename.push_str(&arg);
            }
        }
        filename
    }

}
