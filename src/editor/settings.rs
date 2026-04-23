// Part of Eddy - A lightweight text editor for the terminal.
use crate::editor::{view::EditMode};

#[allow(unused)]
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum TabType {
    #[default]
    Space,
    Tab,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct EditorSettings {
    pub poll_interval: u64,
    pub edit_mode: EditMode,
    pub tab_type: TabType,
    pub tab_size: usize,
    pub msg_delay: usize,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            poll_interval: 100,
            edit_mode: EditMode::Normal,
            tab_type: TabType::Space,
            tab_size: 4,
            msg_delay: 10000,
        }
    }
}

impl EditorSettings {
    pub fn load() -> Self {
        Self::default()
    }
    pub fn save(&self) {
        // ToDo
    }
}
