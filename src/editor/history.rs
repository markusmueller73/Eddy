use crate::editor::{position::Position, row::Row, buffer::TextBuffer};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum HistoryAction {
    AddChar { char: char, position: Position },
    DelChar { char: char, position: Position },
    AddNewline { position: Position },
    DelNewline { position: Position },
}

impl HistoryAction {

    fn redo(&self, buffer: &mut TextBuffer) {
        match self {
            HistoryAction::AddChar { char, position } => {
                buffer.insert(position, *char);
            }
            HistoryAction::DelChar { char: _, position } => {
                buffer.delete(position);
            }
            HistoryAction::AddNewline { position } => {
                //
            }
            HistoryAction::DelNewline { position } => {
                //
            }
        }
    }

    fn undo(&self, buffer: &mut TextBuffer) {
        match self {
            HistoryAction::AddChar { char: _, position } => {
                buffer.delete(position);
            }
            HistoryAction::DelChar { char, position } => {
                buffer.insert(position, *char);
            }
            HistoryAction::AddNewline { position } => {
                //
            }
            HistoryAction::DelNewline { position } => {
                //
            }
        }
    }

}


pub struct HistoryManager {
    undo_stack: VecDeque<HistoryAction>,
    redo_stack: VecDeque<HistoryAction>,
}

impl HistoryManager {
    pub fn new() -> Self {
        HistoryManager {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn add_action(&mut self, action: HistoryAction) {
        self.undo_stack.push_back(action);
        self.redo_stack.clear(); // Clear the redo stack when adding a new action
    }

    pub fn undo(&mut self, buffer: &mut TextBuffer) {
        if let Some(action) = self.undo_stack.pop_back() {
            action.undo(buffer);
            self.redo_stack.push_back(action);
        }
    }

    pub fn redo(&mut self, buffer: &mut TextBuffer) {
        if let Some(action) = self.redo_stack.pop_back() {
            action.redo(buffer);
            self.undo_stack.push_back(action);
        }
    }
}
