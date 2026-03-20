#[derive(Default)]
pub struct Row {
    content: String,
    len: usize,
}

impl Row {

    pub fn new(content: String) -> Self {
        let len = content.chars().count();
        Self {
            content,
            len
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.content.as_bytes()
    }

    pub fn get(&self, from: usize, to: usize) -> String {
        if self.is_empty() {
            return String::new();
        }
        let content: String = self.content.chars().skip(from).take(to - from).collect();
        content
    }

    pub fn add(&mut self, row: &Row) {
        self.content.push_str(&row.content);
        self.len += row.len;
    }

    pub fn append(&mut self, str: &str) {
        let len = str.chars().count();
        self.content.push_str(str);
        self.len += len;
    }

    pub fn insert(&mut self, at: usize, char: char) {
        if at >= self.len {
            self.content.push(char);
            self.len += 1;
            return;
        }
        self.content.insert(at, char);
        self.len = self.content.chars().count();
    }

    pub fn insert_str(&mut self, at: usize, str: &str) {
        if at >= self.len {
            self.content.push_str(str);
            self.len += 1;
            return;
        }
        self.content.insert_str(at, str);
        self.len = self.content.chars().count();
    }

    pub fn split(&mut self, at: usize) -> Row {
        let string = self.content.clone();
        let (str1,str2) = string.split_at(at);
        self.content = str1.to_string();
        Row::new(str2.to_string())
    }

    pub fn delete (&mut self, at: usize) {
        if at >= self.len {
            return;
        }
        self.content.remove(at);
        self.len = self.content.chars().count();
    }

    pub fn find(&self, to_find: &str, from: usize) -> Option<usize> {
        if to_find.is_empty() || from > self.len {
            return None;
        }
        let content: String = self.content.chars().skip(from).collect();
        content.find(to_find)
    }

}
