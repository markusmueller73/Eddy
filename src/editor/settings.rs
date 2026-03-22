#[allow(unused)]
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum TabType {
    #[default]
    Space,
    Tab,
}

pub struct EditorSettings {
    pub tab_type: TabType,
    pub tab_size: usize,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            tab_type: TabType::Space,
            tab_size: 4,
        }
    }
}
