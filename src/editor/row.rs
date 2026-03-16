pub struct Row {
    content: String,
    cursor_position: usize,
    highlighted: bool,
}

impl Row {

    pub fn new(content: String) -> Self {
        Self {
            content,
            cursor_position: 0,
            highlighted: false,
        }
    }

}
