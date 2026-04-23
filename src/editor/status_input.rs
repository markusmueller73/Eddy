// Part of Eddy - A lightweight text editor for the terminal.
use crate::editor::{view::EditMode, row::Row};

pub const SEARCH_TEXT: &str = "Search:";
pub const REPLACE_TEXT: &str = "Replace:";
pub const LOAD_FILE_TEXT: &str = "Filename:";
pub const SAVE_FILE_TEXT: &str = "Set new filename:";

#[derive(Debug, Default)]
pub struct StatusInput {
    content: Row,
}

impl StatusInput {
    pub fn new() -> Self {
        Self {
            content: Row::default(),
        }
    }
    pub fn get(&self) -> String {
        self.content.as_string()
    }
    pub fn get_line(&self, edit_mode: EditMode) -> String {
        let str = match edit_mode {
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
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
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
    pub fn get_start_pos(&self, edit_mode: EditMode) -> usize {
        let x = match edit_mode {
            EditMode::InputFind => SEARCH_TEXT.len(),
            EditMode::InputLoad => LOAD_FILE_TEXT.len(),
            EditMode::InputReplace => REPLACE_TEXT.len(),
            EditMode::InputSaveAs => SAVE_FILE_TEXT.len(),
            _ => 0,
        };
        x + 1
    }
}
