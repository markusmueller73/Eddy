use std::fs;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Action {
    WriteFile { path: String, content: String },
}

impl Action {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Action::WriteFile { path, content } => {
                fs::write(path, content)?;
            }
        }
        Ok(())
    }

    fn undo(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Action::WriteFile { path, .. } => {
                // To make this simple, we'll just delete the file on undo
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }

    fn redo(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.execute()?;
        Ok(())
    }
}


pub struct HistoryManager {
    undo_stack: VecDeque<Action>,
    redo_stack: VecDeque<Action>,
}

impl HistoryManager {
    pub fn new() -> Self {
        HistoryManager {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn add_action(&mut self, action: Action) -> Result<(), Box<dyn std::error::Error>> {
        action.execute()?;
        self.undo_stack.push_back(action);
        self.redo_stack.clear(); // Clear the redo stack when adding a new action
        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(action) = self.undo_stack.pop_back() {
            action.undo()?;
            self.redo_stack.push_back(action);
        }
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(action) = self.redo_stack.pop_back() {
            action.redo()?;
            self.undo_stack.push_back(action);
        }
        Ok(())
    }
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut history = HistoryManager::new();

//     // Example usage: writing a file and then undoing it
//     let action1 = Action::WriteFile { path: "test.txt".to_string(), content: "Hello, world!".to_string() };
//     history.add_action(action1)?;

//     println!("File written. Undoing...");
//     history.undo()?;

//     println!("File undone. Redoing...");
//     history.redo()?;

//     Ok(())
// }
