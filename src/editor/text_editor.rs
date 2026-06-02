use crate::editor::{
    TITLE, VERSION,
    buffer::{DEFAULT_FILENAME, TextBuffer},
    history::{HistoryAction, HistoryManager},
    position::Position,
    settings::EditorSettings,
    status_input::StatusInput,
    status_message::StatusMessage,
    view::{EditMode, TerminalView},
};
use crossterm::{event::{self, Event, KeyCode, KeyModifiers}};
use std::{char, time::Duration};

pub struct TextEditor {
    config: EditorSettings,
    view: TerminalView,
    buffer: TextBuffer,
    history: HistoryManager,
    status_input: StatusInput,
    status_message: Vec<StatusMessage>,
    show_help_menu: bool,
    create_new_file: bool,
    replace_mode: bool,
    current_edit_mode: EditMode,
    last_edit_mode: EditMode,
    to_find: String,
    to_replace: String,
}

impl TextEditor {

    fn new() -> Self {
        Self {
            config: EditorSettings::load(),
            view: TerminalView::new(),
            buffer: TextBuffer::new(),
            history: HistoryManager::new(),
            status_input: StatusInput::new(),
            status_message: Vec::new(),
            show_help_menu: false,
            create_new_file: false,
            replace_mode: false,
            current_edit_mode: EditMode::default(),
            last_edit_mode: EditMode::default(),
            to_find: String::new(),
            to_replace: String::new(),
        }
    }

    /// The main entry function to start the editor. It is the only public function.
    pub fn run(filename: &str) -> Result<(), i32> {
        // First, create a new TextEditor instance
        let mut editor = TextEditor::new();
        editor.add_status_message(&format!("Welcome to {} v{}", TITLE, VERSION));
        // render the editor view for the first time
        // this will initialize the terminal and draw the editor UI
        editor.view.render(&editor.buffer, &editor.status_input, editor.status_message.last().unwrap());
        // Then, load the file if a filename was provided
        if !filename.is_empty() {
            match TextBuffer::open(filename) {
                Ok(buffer) => editor.buffer = buffer,
                Err(err) => {
                    let msg = format!("Failed to open file: {} ({})", filename, err);
                    editor.add_status_message(&msg);
                }
            }
        }
        // Enter the main loop
        loop {
            // Get the user input
            if event::poll(std::time::Duration::from_millis(editor.config.poll_interval)).unwrap_or(false) {
                // It's guaranteed that the `read()` won't block when the `poll()` function returns `true`
                if let Ok(event) = event::read() && !editor.handle_input(event) {
                    break;
                }
            }
            editor.view.render(&editor.buffer, &editor.status_input, editor.status_message.last().unwrap());
        }
        editor.view.quit();
        editor.config.save();
        Ok(())
    }

    /// Adds a status message to be displayed to the user.
    pub fn add_status_message(&mut self, message: &str) {
        self.status_message.push(StatusMessage::new(message, Duration::from_millis(self.config.msg_delay)));
    }

    pub fn is_edit_mode(&self) -> bool {
        self.current_edit_mode == EditMode::Insert || self.current_edit_mode == EditMode::Normal
    }

    /// Processed the whole user input here and handle events.
    fn handle_input(&mut self, event: Event) -> bool {
        match event {
            //
            // Handle key events
            //
            Event::Key(key) => {
                //
                // Check the modifiers Control + Shift, hold by user
                //
                if key.modifiers == KeyModifiers::CONTROL && key.modifiers == KeyModifiers::SHIFT {
                    match key.code {
                        //
                        // Save File As
                        //
                        KeyCode::Char('s') => {
                            self.last_edit_mode = self.current_edit_mode;
                            self.current_edit_mode = EditMode::InputSaveAs;
                            self.status_input.set_mode(self.current_edit_mode);
                        }
                        //
                        // Redo
                        //
                        KeyCode::Char('z') => {
                            self.history.redo(&mut self.buffer);
                        }
                        _ => {}
                    }
                    return true;
                }
                //
                // Check the modifier Control, hold by user
                //
                if key.modifiers == KeyModifiers::CONTROL {
                    match key.code {
                        //
                        // Copy
                        //
                        KeyCode::Char('c') => {
                            let (start,end) = self.view.get_marked_positions();
                            let marked_text = self.buffer.get_range(&start, &end);
                            self.view.copy_text_to_clipboard(&marked_text);
                        }
                        //
                        // Find
                        //
                        KeyCode::Char('f') => {
                            self.last_edit_mode = self.current_edit_mode;
                            self.current_edit_mode = EditMode::InputFind;
                            self.status_input.set_mode(self.current_edit_mode);
                        }
                        //
                        // Help
                        //
                        KeyCode::Char('h') => {
                            self.show_help_menu = !self.show_help_menu;
                        }
                        //
                        // New File
                        //
                        KeyCode::Char('n') => {
                            if self.buffer.is_modified() {
                                if self.buffer.filename() == DEFAULT_FILENAME || self.buffer.filename().is_empty() {
                                    self.last_edit_mode = self.current_edit_mode;
                                    self.current_edit_mode = EditMode::InputSaveAs;
                                    self.status_input.set_mode(self.current_edit_mode);
                                    self.create_new_file = true;
                                } else {
                                    match self.buffer.save() {
                                        Ok(()) => {},
                                        Err(err) => {
                                            let msg = format!("Failed to save file: {} ({})", self.buffer.filename(), err);
                                            self.add_status_message(&msg);
                                        }
                                    }
                                    self.buffer = TextBuffer::new();
                                    self.current_edit_mode = EditMode::default();
                                }
                            } else {
                                self.buffer = TextBuffer::new();
                                self.current_edit_mode = EditMode::default();
                            }
                        }
                        //
                        // Open File
                        //
                        KeyCode::Char('o') => {
                            self.last_edit_mode = self.current_edit_mode;
                            self.current_edit_mode = EditMode::InputLoad;
                            self.status_input.set_mode(self.current_edit_mode);
                        }
                        //
                        // Save File
                        //
                        KeyCode::Char('s') => {
                            if self.buffer.filename() == DEFAULT_FILENAME || self.buffer.filename().is_empty() {
                                self.last_edit_mode = self.current_edit_mode;
                                self.current_edit_mode = EditMode::InputSaveAs;
                                self.status_input.set_mode(self.current_edit_mode);
                            } else {
                                match self.buffer.save() {
                                    Ok(_) => {
                                        self.add_status_message("Successfully saved.");
                                    }
                                    Err(err) => {
                                        self.add_status_message(&format!("Failed to save file: {}", err));
                                    }
                                }
                            }
                        }
                        //
                        // Quit
                        //
                        KeyCode::Char('q') => {
                            return false;
                        }
                        //
                        // Replace
                        //
                        KeyCode::Char('r') => {
                            self.last_edit_mode = self.current_edit_mode;
                            self.current_edit_mode = EditMode::InputFind;
                            self.status_input.set_mode(self.current_edit_mode);
                            self.replace_mode = true;
                        }
                        //
                        // Paste
                        //
                        KeyCode::Char('v') => {
                            self.view.paste_from_clipboard(&self.view.position());
                        }
                        //
                        // Cut
                        //
                        KeyCode::Char('x') => {
                            let (start,end) = self.view.get_marked_positions();
                            let marked_text = self.buffer.get_range(&start, &end);
                            self.view.copy_text_to_clipboard(&marked_text);
                            self.buffer.delete_range(&start, &end);
                        }
                        //
                        // Undo
                        //
                        KeyCode::Char('z') => {
                            self.history.undo(&mut self.buffer);
                        }
                        //
                        // Unused KeyCodes
                        //
                        _ => {}
                    }
                    return true;
                }
                //
                // Check the modifier Shift, hold by user
                //
                if key.modifiers == KeyModifiers::SHIFT {
                    //
                    // Mark the text
                    //
                    match key.code {
                        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                            if self.view.is_marking() {
                                self.view.end_marking();
                                self.view.move_cursor(&self.buffer, key.code);
                            } else {
                                self.view.start_marking();
                                self.view.move_cursor(&self.buffer, key.code);
                            }
                        }
                        //
                        // Insert the upper case character into the buffer or into the status bar
                        //
                        KeyCode::Char(char) => {

                            if self.current_edit_mode == EditMode::Insert {
                                self.buffer.insert(&self.view.position(), char);
                                self.view.move_cursor(&self.buffer, KeyCode::Right);
                            } else if self.current_edit_mode == EditMode::Normal {
                                let pos = pos!(self.view.position().x + 1, self.view.position().y);
                                self.buffer.insert(&pos, char);
                                self.view.move_cursor(&self.buffer, KeyCode::Right);
                            } else {
                                let pos = self.view.position().x + 1;
                                self.status_input.insert(pos, char);
                                self.view.user_input_move_cursor(self.status_input.get_start_pos(), self.status_input.len(), KeyCode::Right);
                            }
                        }
                        //
                        // Unused KeyCodes
                        //
                        _ => {}
                    }
                    return true;
                }
                //
                // Without modifiers, process the key normally
                //
                match key.code {
                    //
                    // Backspace - delete the character before the cursor
                    //
                    KeyCode::Backspace => {
                        if self.current_edit_mode == EditMode::Insert || self.current_edit_mode == EditMode::Normal {
                            self.view.move_cursor(&self.buffer, KeyCode::Left);
                            let char = self.buffer.get_char(&self.view.position());
                            self.buffer.delete(&self.view.position());
                            self.history.add_action(HistoryAction::DelChar { char, position: self.view.position() });
                        } else {
                            self.view.user_input_move_cursor(self.status_input.get_start_pos(), self.status_input.len(), KeyCode::Left);
                            let pos = self.view.position().x - self.status_input.get_start_pos();
                            self.status_input.delete(pos);
                        }
                    }
                    //
                    // Insert the character into the buffer or the user input in the status bar
                    //
                    KeyCode::Char(char) => {
                        if self.current_edit_mode == EditMode::Insert {
                            self.buffer.delete(&self.view.position());
                            self.buffer.insert(&self.view.position(), char);
                            self.view.move_cursor(&self.buffer, KeyCode::Right);
                        } else if self.current_edit_mode == EditMode::Normal {
                            let pos = pos!(self.view.position().x + 1, self.view.position().y);
                            self.buffer.insert(&pos, char);
                            self.view.move_cursor(&self.buffer, KeyCode::Right);
                            self.history.add_action(HistoryAction::AddChar { char, position: pos });
                        } else {
                            let pos = self.view.position().x + 1;
                            self.status_input.insert(pos, char);
                            self.view.user_input_move_cursor(self.status_input.get_start_pos(), self.status_input.len(), KeyCode::Right);
                        }
                    }
                    //
                    // Delete the character from the buffer or the user input in the status bar
                    //
                    KeyCode::Delete => {
                        if self.current_edit_mode == EditMode::Insert || self.current_edit_mode == EditMode::Normal {
                            let char = self.buffer.get_char(&self.view.position());
                            self.buffer.delete(&self.view.position());
                            self.history.add_action(HistoryAction::DelChar { char, position: self.view.position() });
                        } else {
                            self.status_input.delete(self.view.position().x);
                        }
                    }
                    //
                    // Enter key
                    //
                    KeyCode::Enter => {
                        match self.current_edit_mode {

                            EditMode::InputFind => {
                                self.to_find = self.status_input.as_string();
                                if self.replace_mode {
                                    self.current_edit_mode = EditMode::InputReplace;
                                    self.status_input.set_mode(self.current_edit_mode);
                                } else {
                                    if let Some(pos) = self.buffer.find(&self.to_find, &self.view.position()) {
                                        self.view.place_cursor(pos);
                                        self.add_status_message(&format!("Found text at position {}x{}.", pos.x, pos.y));
                                    } else {
                                        self.add_status_message(&format!("Search phrase '{}' not found.", self.status_input.as_string()));
                                    }
                                    self.current_edit_mode = self.last_edit_mode;
                                    self.status_input.set_mode(self.current_edit_mode);
                                }
                            },

                            EditMode::InputReplace => {
                                self.to_replace = self.status_input.as_string();
                                if let Some(pos) = self.buffer.find(&self.to_find, &self.view.position()) {
                                    self.view.place_cursor(pos);
                                    self.add_status_message(&format!("Found text at position {}x{}.", pos.x, pos.y));
                                } else {
                                    self.add_status_message(&format!("Search phrase '{}' not found.", self.status_input.as_string()));
                                }
                                self.current_edit_mode = self.last_edit_mode;
                                self.status_input.set_mode(self.current_edit_mode);
                                self.replace_mode = false;
                            },

                            EditMode::InputLoad => {
                                let filename = self.status_input.as_string();
                                if filename.is_empty() {
                                    self.add_status_message("Please enter a valid filename.");
                                } else {
                                    match TextBuffer::open(&filename) {
                                        Ok(buffer) => {
                                            self.buffer = buffer;
                                        }
                                        Err(err) => {
                                            let msg = format!("Failed to open file: {} ({})", filename, err);
                                            self.add_status_message(&msg);
                                        }
                                    }
                                }
                                self.current_edit_mode = self.last_edit_mode;
                                self.status_input.set_mode(self.current_edit_mode);
                            },

                            EditMode::InputSaveAs => {
                                let filename = self.status_input.as_string();
                                if filename.is_empty() {
                                    self.add_status_message("Please enter a valid filename.");
                                } else {
                                    self.buffer.new_filename(&filename);
                                    match self.buffer.save() {
                                        Ok(_) => {
                                            self.add_status_message("Successfully saved.");
                                            if self.create_new_file {
                                                self.buffer = TextBuffer::new();
                                                self.create_new_file = false;
                                            }
                                        }
                                        Err(err) => {
                                            self.add_status_message(&format!("Failed to save file: {}", err));
                                        }
                                    }
                                }
                                self.current_edit_mode = self.last_edit_mode;
                                self.status_input.set_mode(self.current_edit_mode);
                            },

                            _ => {
                                self.buffer.insert_newline(&self.view.position());
                                self.view.move_cursor(&self.buffer, KeyCode::Right);
                                self.history.add_action(HistoryAction::AddNewline { position: self.view.position() });
                            }
                        }
                    }
                    //
                    KeyCode::Insert => {
                        if self.current_edit_mode == EditMode::Insert {
                            self.current_edit_mode = EditMode::Normal;
                        } else if self.current_edit_mode == EditMode::Normal {
                            self.current_edit_mode = EditMode::Insert;
                        }
                        self.status_input.set_mode(self.current_edit_mode);
                    }
                    //
                    // Insert a tab character
                    //
                    KeyCode::Tab => {
                        self.buffer.insert(&self.view.position(), '\t');
                    }
                    //
                    // Cursor movement
                    //
                    KeyCode::Up | KeyCode::Down | KeyCode::PageUp | KeyCode::PageDown | KeyCode::End | KeyCode::Home => {
                        if self.view.is_marking() {
                            self.view.reset_marking();
                        }
                        if self.is_edit_mode() {
                            self.view.move_cursor(&self.buffer, key.code);
                        }
                    }
                    KeyCode::Left | KeyCode::Right => {
                        if self.is_edit_mode() {
                            self.view.move_cursor(&self.buffer, key.code);
                        } else {
                            self.view.user_input_move_cursor(self.status_input.get_start_pos(), self.status_input.len(), key.code);
                        }
                    }
                    //
                    // Unused KeyCode
                    //
                    _ => {}
                }
            }
            //
            // Resize the viewport
            //
            Event::Resize(width, height) => {
                self.view.resize(width as usize, height as usize);
            }
            _ => {},
        }
        true
    }

}
