#[derive(Default)]
pub struct Row {
    content: Vec<char>,
    len: usize,
}

impl Row {

    pub fn new(content: String) -> Self {
        Self {
            content: content.chars().collect(),
            len: content.chars().count()
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn as_string(&self) -> String {
        self.content.iter().collect()
    }

    pub fn get(&self, from: usize, to: usize) -> String {
        if self.is_empty() {
            return String::new();
        }
        let start = from;
        let end = if to >= self.len {
            self.len - 1
        } else {
            to
        };
        let temp_vec = self.content[start..=end].to_vec();
        let content: String = temp_vec.iter().collect();
        content
    }

    pub fn add(&mut self, row: &Row) {
        for c in &row.content {
            self.content.push(*c);
        }
        self.len += row.len;
    }

    pub fn append(&mut self, static_str: &str) {
        let len = static_str.chars().count();
        for c in static_str.chars() {
            self.content.push(c);
        }
        self.len += len;
    }

    pub fn insert(&mut self, at: usize, char: char) {
        if at >= self.len {
            self.content.push(char);
            self.len += 1;
            return;
        }
        self.content.insert(at, char);
        self.len = self.content.len();
    }

    pub fn insert_str(&mut self, at: usize, static_str: &str) {
        if at >= self.len {
            self.append(static_str);
            self.len += static_str.chars().count();
            return;
        }
        for (i, c) in static_str.chars().enumerate() {
            self.content.insert(at + i, c);
        }
        self.len = self.content.len();
    }

    pub fn split(&mut self, at: usize) -> Row {
        let tmp_vec = self.content.split_off(at);
        let tmp_len = tmp_vec.len();
        self.len = self.content.len();
        Row {
            content: tmp_vec,
            len: tmp_len
        }
    }

    pub fn delete (&mut self, at: usize) {
        if at >= self.len {
            return;
        }
        self.content.remove(at);
        self.len = self.content.len();
    }

    pub fn find(&self, to_find: &str, from: usize) -> Option<usize> {
        if to_find.is_empty() || from >= self.len {
            return None;
        }
        let content: String = self.content.iter().skip(from).collect();
        content.find(to_find)
    }

}
