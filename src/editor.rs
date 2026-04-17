// Part of Eddy - A lightweight text editor for the terminal.
#[macro_use]
mod position;
mod buffer;
mod color_pairs;
mod row;
mod settings;
mod status_input;
mod status_message;
mod view;

use crate::editor::{view::{EditMode, TerminalView}};
use crossterm::{event::{self, Event, KeyCode, KeyModifiers}};

pub const TITLE: &str = "Eddy";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_FILENAME: &str = "Untitled";

/// The main entry function to start the editor.
/// Processed the whole user input here and handle events.
pub fn run() -> Result<(), i32> {
    // The instance for the terminal view, initialize the raw mode and the alt screen buffer
    let mut view = TerminalView::new();
    // A switch, to show a command help similar to a menu bar
    let mut show_menu = false;
    // A switch to indicate if a new file should be created
    let mut new_file = false;
    // Parse the command line arguments for the initial file to open,
    // the function is at the bottom of this rs-file
    let mut filename = parse_command_line_arguments();
    if !filename.is_empty() {
        view.add_msg(&format!("Opening file: {}", filename));
        view.open_file(&filename);
    }
    // Show the initial view at the frist time
    view.render();
    // Enter the main loop
    loop {
        // Get the user input
        if event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
            // It's guaranteed that the `read()` won't block when the `poll()` function returns `true`
            if let Ok(event) = event::read() {
                match event {
                    // Handle key events
                    Event::Key(key) => {
                        // First check the used modifiers like Alt, Control, Shift
                        if key.modifiers == KeyModifiers::CONTROL && key.modifiers == KeyModifiers::SHIFT {
                            match key.code {
                                KeyCode::Char('s') => {
                                    // Save File As
                                    view.user_input_clear();
                                    view.set_edit_mode(EditMode::InputSaveAs);
                                }
                                KeyCode::Char('z') => {
                                    // Redo
                                }
                                _ => {}
                            }
                            continue;
                        }
                        if key.modifiers == KeyModifiers::CONTROL {
                            match key.code {
                                KeyCode::Char('c') => {
                                    // Copy
                                    view.copy_marked_text_to_clipboard();
                                }
                                KeyCode::Char('f') => {
                                    // Find
                                    view.user_input_clear();
                                    view.set_edit_mode(EditMode::InputFind);
                                }
                                KeyCode::Char('h') => {
                                    // Show help
                                    show_menu = !show_menu;
                                }
                                KeyCode::Char('n') => {
                                    // New File
                                    if view.is_modified() {
                                        if filename == DEFAULT_FILENAME || filename.is_empty() {
                                            view.user_input_clear();
                                            view.set_edit_mode(EditMode::InputSaveAs);
                                            new_file = true;
                                        } else {
                                            view.save_file(&filename);
                                            view.new_file();
                                            view.set_last_mode();
                                        }
                                    } else {
                                        view.new_file();
                                    }
                                }
                                KeyCode::Char('o') => {
                                    view.user_input_clear();
                                    view.set_edit_mode(EditMode::InputLoad);
                                }
                                KeyCode::Char('s') => {
                                    // Save File
                                    if filename == DEFAULT_FILENAME || filename.is_empty() {
                                        view.user_input_clear();
                                        view.set_edit_mode(EditMode::InputSaveAs);
                                    } else {
                                        view.save_file(&filename);
                                        view.set_last_mode();
                                    }
                                }
                                KeyCode::Char('q') => {
                                    break;
                                }
                                KeyCode::Char('r') => {
                                    // Replace
                                }
                                KeyCode::Char('v') => {
                                    // Paste
                                }
                                KeyCode::Char('x') => {
                                    // Cut
                                    view.cut_marked_text_to_clipboard();
                                }
                                KeyCode::Char('z') => {
                                    // Undo
                                }
                                _ => {}
                            }
                            continue;
                        }
                        if key.modifiers == KeyModifiers::SHIFT {
                            match key.code {
                                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                                    // Mark
                                    if view.is_marking() {
                                        //view.move_cursor(key.code);
                                        view.end_marking();
                                        view.move_cursor(key.code);
                                    } else {
                                        view.start_marking();
                                        view.move_cursor(key.code);
                                    }
                                }
                                KeyCode::Char(char) => {
                                    // Insert the upper case character into the buffer or into the status bar
                                    if view.user_input_mode() {
                                        view.user_input_insert_char(char);
                                    } else {
                                        view.insert_char(char);
                                    }
                                }
                                _ => {}
                            }
                            continue;
                        }
                        // Without modifiers, process the key normally
                        match key.code {
                            KeyCode::Backspace => {
                                if view.user_input_mode() {
                                    view.user_input_move_cursor(KeyCode::Left);
                                    view.user_input_delete_char();
                                } else {
                                    view.delete_char_before();
                                }
                            }
                            KeyCode::Char(char) => {
                                // Insert the character into the buffer or the user input in the status bar
                                if view.user_input_mode() {
                                    view.user_input_insert_char(char);
                                } else {
                                    view.insert_char(char);
                                }

                            }
                            KeyCode::Delete => {
                                if view.user_input_mode() {
                                    view.user_input_delete_char();
                                } else {
                                    view.delete_char();
                                }
                            }
                            KeyCode::Enter => {
                                match view.edit_mode() {
                                    EditMode::InputFind => {
                                        let (x,y) = view.find_text(&view.user_input_get());
                                        if x == 0 && y == 0 {
                                            view.add_msg(&format!("Search phrase '{}' not found.", view.user_input_get()));
                                        } else {
                                            view.add_msg(&format!("Found text at position {}x{}.", x, y));
                                        }
                                        view.set_last_mode();
                                    },
                                    EditMode::InputLoad => {
                                        filename = view.user_input_get();
                                        if filename.is_empty() {
                                            view.add_msg("Please enter a valid filename.");
                                        } else {
                                            view.open_file(&filename);
                                            view.set_last_mode();
                                        }
                                    },
                                    EditMode::InputSaveAs => {
                                        filename = view.user_input_get();
                                        if filename.is_empty() {
                                            view.add_msg("Please enter a valid filename.");
                                        } else {
                                            view.save_file(&filename);
                                            if new_file {
                                                view.new_file();
                                                new_file = false;
                                            }
                                            view.set_last_mode();
                                        }
                                    },
                                    _ => {
                                        view.insert_newline();
                                    }
                                }
                            }
                            KeyCode::Insert => {
                                if view.edit_mode() == EditMode::Insert {
                                    view.set_edit_mode(EditMode::Normal);
                                } else if view.edit_mode() == EditMode::Normal {
                                    //view.set_edit_mode(EditMode::Insert);
                                }
                            }
                            KeyCode::Tab => {
                                view.insert_char('\t');
                            }
                            // Cursor movement
                            KeyCode::Up | KeyCode::Down |
                            KeyCode::PageUp | KeyCode::PageDown |
                            KeyCode::End | KeyCode::Home => {
                                if view.is_marking() {
                                    view.reset_marking();
                                }
                                if !view.user_input_mode() {
                                    view.move_cursor(key.code);
                                }
                            }
                            KeyCode::Left | KeyCode::Right => {
                                if view.user_input_mode() {
                                    view.user_input_move_cursor(key.code);
                                } else {
                                    view.move_cursor(key.code);
                                }
                            }
                            _ => {}
                        }
                    }
                    // Resize the viewport
                    Event::Resize(width, height) => {
                        view.resize(width as usize, height as usize);
                    }
                    _ => {},
                }
            }
        }
        // Show the initial view
        view.render();
    }
    view.quit();
    Ok(())
}

/// Parse command line arguments and return the current file path.
fn parse_command_line_arguments() -> String {
    // Get the first command line argument, it is by default the name of the executeable
    let exe = std::env::args().next().unwrap_or(String::from("eddy"));
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
