// Part of Eddy - A lightweight text editor for the terminal.
//! Row, a structure representing a single row of text.
//! The `Row` struct holds a vector of `char` and provides methods for accessing
//! and modifying the row's content.
//! The vector of chars is used to store ascii and unicode (UTF-8) characters.
#[derive(Debug, Default)]
pub struct Row {
    content: Vec<char>,
    len: usize,
}

impl Row {

    /// Creates a new `Row` from a `String`. The string is converted to a vector of `char`.
    pub fn new(content: String) -> Self {
        Self {
            content: content.chars().collect(),
            len: content.chars().count()
        }
    }

    /// Returns the number of characters in the row.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the row is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Clears the row, removing all content, but didn't drop the underlying `Vec<char>`.
    pub fn clear(&mut self) {
        self.content.clear();
    }

    /// Returns a `String` representation of the row.
    pub fn as_string(&self) -> String {
        self.content.iter().collect()
    }

    /// Returns a `String` representation of the row from `from` to `to`, inclusive.
    pub fn get_range(&self, from: usize, to: usize) -> String {
        if self.is_empty() {
            return String::new();
        }
        let start = from;
        let end = if to >= self.len {
            self.len.saturating_sub(1)
        } else {
            to
        };
        let temp_vec = self.content[start..=end].to_vec();
        let content: String = temp_vec.iter().collect();
        content
    }

    /// Adds the content of another `Row` to this `Row`.
    pub fn add(&mut self, row: &Row) {
        for c in &row.content {
            self.content.push(*c);
        }
        self.len += row.len;
    }

    /// Appends a `str` to this `Row`.
    pub fn append(&mut self, static_str: &str) {
        let len = static_str.chars().count();
        for c in static_str.chars() {
            self.content.push(c);
        }
        self.len += len;
    }

    /// Inserts a `char` at the specified position in this `Row`.
    pub fn insert(&mut self, at: usize, char: char) {
        if at >= self.len {
            self.content.push(char);
            self.len += 1;
            return;
        }
        self.content.insert(at, char);
        self.len = self.content.len();
    }

    /// Inserts a `str` at the specified position in this `Row`.
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

    /// Splits this `Row` at the specified position, returning a new `Row` with the remaining content.
    pub fn split(&mut self, at: usize) -> Row {
        let tmp_vec = self.content.split_off(at);
        let tmp_len = tmp_vec.len();
        self.len = self.content.len();
        Row {
            content: tmp_vec,
            len: tmp_len
        }
    }

    /// Deletes the character at the specified position in this `Row`.
    pub fn delete(&mut self, at: usize) {
        if at >= self.len {
            return;
        }
        self.content.remove(at);
        self.len -= 1;
    }

    /// Deletes the characters in the specified range in this `Row`.
    pub fn delete_range(&mut self, from: usize, to: usize) {
        if self.content.is_empty() {
            return;
        }
        if from >= self.len {
            return;
        }
        let start = from;
        let end = if to >= self.len {
            self.len.saturating_sub(1)
        } else {
            to
        };
        self.content.drain(start..=end);
        self.len = self.content.len();
    }

    /// Finds the first occurrence of the specified `str` in this `Row`,
    /// starting from the specified position.
    pub fn find(&self, to_find: &str, from: usize) -> Option<usize> {
        if to_find.is_empty() || from >= self.len {
            return None;
        }
        let content: String = self.content.iter().skip(from).collect();
        content.find(to_find)
    }

}
