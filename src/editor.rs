// Part of Eddy - A lightweight text editor for the terminal.
mod buffer;
mod color_pairs;
#[macro_use]
mod position;
mod row;
mod settings;
mod status_message;
mod view;

use crate::editor::{row::Row, view::{EditMode, TerminalView}};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub const TITLE: &str = "Eddy";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_FILENAME: &str = "Untitled";

/// The main entry function to start the editor.
/// Process the whole user input here and handle events.
pub fn run() -> Result<(), i32> {
    let mut term_view = TerminalView::new();
    let mut filename = parse_command_line_arguments();
    let mut user_input = Row::default();
    let mut show_menu = false;
    if !filename.is_empty() {
        term_view.add_msg(&format!("Opening file: {}", filename));
        term_view.open_file(&filename);
    }
    // Show the initial view
    term_view.render(&user_input.as_string());
    // Enter the main loop
    loop {
        // Get the user input
        if event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
            // It's guaranteed that the `read()` won't block when the `poll()` function returns `true`
            if let Ok(event) = event::read() {
                match event {
                    Event::Key(key) => {
                        // Handle key events
                        // First check the used modifiers like Alt, Control, Shift
                        if key.modifiers == KeyModifiers::CONTROL && key.modifiers == KeyModifiers::SHIFT {
                            match key.code {
                                KeyCode::Char('s') => {
                                    // Save File As
                                    term_view.set_edit_mode(EditMode::InputSaveAs);
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
                                }
                                KeyCode::Char('f') => {
                                    // Find
                                    user_input.clear();
                                    term_view.set_edit_mode(EditMode::InputFind);
                                }
                                KeyCode::Char('h') => {
                                    // Show help
                                    if !show_menu {
                                        // Show menu
                                        show_menu = true;
                                    }
                                }
                                KeyCode::Char('n') => {
                                    // New File
                                }
                                KeyCode::Char('o') => {
                                    term_view.set_edit_mode(EditMode::InputLoad);
                                }
                                KeyCode::Char('s') => {
                                    // Save File
                                    if filename.is_empty(){
                                        // Set new file name
                                        filename = DEFAULT_FILENAME.to_string();
                                    }
                                    term_view.save_file(&filename);
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
                                KeyCode::Up => {
                                    // Mark
                                }
                                KeyCode::Down => {
                                    // Mark
                                }
                                KeyCode::Left => {
                                    // Mark
                                }
                                KeyCode::Right => {
                                    // Mark
                                }
                                KeyCode::Char(char) => {
                                    // Insert the upper case character into the buffer or into the status bar
                                    if term_view.user_input_mode() {
                                        user_input.insert(user_input.len(), char);
                                    } else {
                                        term_view.insert_char(char);
                                    }
                                }
                                _ => {}
                            }
                            continue;
                        }
                        // Without modifiers, process the key normally
                        match key.code {
                            KeyCode::Backspace => {
                                if term_view.user_input_mode() {
                                    if !user_input.is_empty() {
                                        term_view.move_user_input_cursor(KeyCode::Left);
                                        user_input.delete(user_input.len()-1);
                                    }
                                } else {
                                    term_view.delete_char_before();
                                }
                            }
                            KeyCode::Char(char) => {
                                // Insert the character into the buffer or the user input in the status bar
                                if term_view.user_input_mode() {
                                    user_input.insert(user_input.len(), char);
                                } else {
                                    term_view.insert_char(char);
                                }

                            }
                            KeyCode::Delete => {
                                if term_view.user_input_mode() {
                                    //user_input.delete(user_input.len());
                                } else {
                                    term_view.delete_char();
                                }
                            }
                            KeyCode::Enter => {
                                match term_view.edit_mode() {
                                    EditMode::InputFind => {
                                        let (x,y) = term_view.find_text(&user_input.as_string());
                                        if x == 0 && y == 0 {
                                            term_view.add_msg(&format!("Search phrase '{}' not found.", user_input.as_string()));
                                        } else {
                                            term_view.add_msg(&format!("Found text at position {}x{}.", x, y));
                                        }
                                        term_view.set_last_mode();
                                    },
                                    EditMode::InputLoad => {},
                                    EditMode::InputSaveAs => {},
                                    _ => {
                                        term_view.insert_newline();
                                    }
                                }
                            }
                            KeyCode::Insert => {
                                if term_view.edit_mode() == EditMode::Insert {
                                    term_view.set_edit_mode(EditMode::Normal);
                                } else if term_view.edit_mode() == EditMode::Normal {
                                    //term_view.set_edit_mode(EditMode::Insert);
                                }
                            }
                            KeyCode::Tab => {
                                term_view.insert_char('\t');
                            }
                            // Cursor movement
                            KeyCode::Up | KeyCode::Down |
                            KeyCode::PageUp | KeyCode::PageDown |
                            KeyCode::End | KeyCode::Home => {
                                if !term_view.user_input_mode() {
                                    term_view.move_cursor(key.code);
                                }
                            }
                            KeyCode::Left | KeyCode::Right => {
                                if term_view.user_input_mode() {
                                    term_view.move_user_input_cursor(key.code);
                                } else {
                                    term_view.move_cursor(key.code);
                                }
                            }
                            _ => {}
                        }
                    }
                    // Resize the viewport
                    Event::Resize(width, height) => {
                        term_view.resize(width as usize, height as usize);
                    }
                    _ => {},
                }
            }
        }
        // Show the initial view
        term_view.render(&user_input.as_string());
    }
    term_view.quit();
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
