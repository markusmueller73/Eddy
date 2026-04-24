// Part of Eddy - A lightweight text editor for the terminal.
use crate::editor::{view::EditMode, row::Row};

pub const SEARCH_TEXT: &str = "Search:";
pub const REPLACE_TEXT: &str = "Replace:";
pub const LOAD_FILE_TEXT: &str = "Filename:";
pub const SAVE_FILE_TEXT: &str = "Set new filename:";

#[derive(Debug, Default)]
pub struct StatusInput {
    content: Row,
    mode: EditMode,
    position: usize,
}

impl StatusInput {
    pub fn new() -> Self {
        Self {
            content: Row::default(),
            mode: EditMode::default(),
            position: 0,
        }
    }
    pub fn is_active(&self) -> bool {
        self.mode != EditMode::Insert && self.mode != EditMode::Normal
    }
    pub fn as_string(&self) -> String {
        self.content.as_string()
    }
    pub fn set_mode(&mut self, mode: EditMode) {
        self.clear();
        self.mode = mode;
        self.position = self.get_start_pos();
    }
    pub fn get_mode(&self) -> EditMode {
        self.mode
    }
    pub fn get_start_pos(&self) -> usize {
        let x = match self.mode {
            EditMode::InputFind => SEARCH_TEXT.len(),
            EditMode::InputLoad => LOAD_FILE_TEXT.len(),
            EditMode::InputReplace => REPLACE_TEXT.len(),
            EditMode::InputSaveAs => SAVE_FILE_TEXT.len(),
            _ => 0,
        };
        x + 1
    }
    pub fn get_content(&self) -> String {
        let str = match self.mode {
            EditMode::InputFind => SEARCH_TEXT,
            EditMode::InputLoad => LOAD_FILE_TEXT,
            EditMode::InputReplace => REPLACE_TEXT,
            EditMode::InputSaveAs => SAVE_FILE_TEXT,
            _ => "",
        };
        format!("{} {}", str, self.content.as_string())
    }
    pub fn len(&self) -> usize {
        self.content.len()
    }
    pub fn insert(&mut self, pos: usize, ch: char) {
        self.content.insert(pos, ch);
    }
    pub fn delete(&mut self, pos: usize) {
        self.content.delete(pos);
    }
    pub fn clear(&mut self) {
        self.content.clear();
    }
}
