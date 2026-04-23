// Part of Eddy - A lightweight text editor for the terminal.
//! TextBuffer, a structure representing a text buffer containing multiple rows of text.
//! Any row is is a struct of `Row`. All `Row`s are stored in a `Vec`.
use crate::editor::{row::Row, position::Position};
use std::{fs::File, io::{BufRead, BufReader, BufWriter, Write}};

/// The default file name used when no file is specified.
pub const DEFAULT_FILENAME: &str = "Untitled";

/// EOL is the end-of-line character sequence, depending on the target platform.
#[cfg(target_family = "windows")]
pub const EOL: &str = "\r\n";
#[cfg(not(target_family = "windows"))]
pub const EOL: &str = "\n";

pub struct TextBuffer {
    rows: Vec<Row>,
    modified: bool,
    file_name: String,
}

impl TextBuffer {

    /// Creates a new, empty `TextBuffer` with the default file name.
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            modified: false,
            file_name: String::from(DEFAULT_FILENAME),
        }
    }

    /// Opens a file and returns a `TextBuffer` with its contents. For file reading
    /// the rust `BufReader` is used. It reads the file line by line.
    pub fn open(file_name: &str) -> Result<TextBuffer, std::io::Error> {
        match File::open(file_name) {
            Ok(file) => {
                let mut row_vec: Vec<Row> = Vec::new();
                let buf_reader = BufReader::new(file);
                for line in buf_reader.lines() {
                    match line {
                        Ok(line) => {
                            row_vec.push(Row::new(line));
                        },
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                Ok(TextBuffer{
                    rows: row_vec,
                    modified: false,
                    file_name: file_name.to_string(),
                })
            },
            Err(err) => {
                Err(err)
            }
        }
    }

    /// Saves the buffer to a file.
    pub fn save(&mut self) -> Result<(), std::io::Error> {
        match File::create(&self.file_name) {
            Ok(file) => {
                let mut buf_writer = BufWriter::new(file);
                for row in &self.rows {
                    let row_string = row.as_string();
                    match buf_writer.write_all(row_string.as_bytes()) {
                        Ok(()) => {}
                        Err(err) => {
                            return Err(err);
                        }
                    }
                    match buf_writer.write_all(EOL.as_bytes()) {
                        Ok(()) => {}
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                match buf_writer.flush() {
                    Ok(()) => {}
                    Err(err) => {
                        return Err(err);
                    }
                }
                self.modified = false;
            }
            Err(err) => {
                return Err(err);
            }
        }
        Ok(())
    }

    /// Returns `true` if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Returns `true` if the buffer is modified.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Returns the number of rows in the buffer.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Returns the filename of the buffer.
    pub fn filename(&self) -> &str {
        &self.file_name
    }

    /// Sets the filename of the buffer.
    pub fn new_filename(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }

    /// Returns a reference to the row at the given index, if it exists.
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    /// Returns the content of the buffer from `start` to `end` `Position` as a string.
    pub fn get_range(&self, start: &Position, end: &Position) -> String {
        let mut result = String::new();
        for y in start.y..=end.y {
            if let Some(row) = self.rows.get(y) {
                if y == start.y && y == end.y {
                    result.push_str(&row.get_range(start.x, end.x));
                } else if y == start.y {
                    result.push_str(&row.get_range(start.x, row.len().saturating_sub(1)));
                } else if y == end.y {
                    result.push_str(&row.get_range(0, end.x));
                } else {
                    result.push_str(&row.get_range(0, row.len().saturating_sub(1)));
                }
            }
        }
        result
    }

    /// Deletes the content of the buffer from `start` to `end` `Position`.
    pub fn delete_range(&mut self, start: &Position, end: &Position) {
        for y in start.y..=end.y {
            let row_len = self.rows[y].len();
            if y == start.y && y == end.y {
                self.rows[y].delete_range(start.x, end.x);
            } else if y == start.y {
                self.rows[y].delete_range(start.x, row_len);
            } else if y == end.y {
                self.rows[y].delete_range(0, end.x);
            } else {
                self.rows[y].delete_range(0, row_len);
            }
        }
    }

    /// Inserts a `char` at the given `Position`.
    /// If `char` is a newline or carriage return, inserts a newline.
    pub fn insert(&mut self, at: &Position, char: char) {
        let y = at.y;
        if y > self.rows.len() {
            return;
        }
        if char == '\n' || char == '\r'{
            self.insert_newline(at);
        } else if y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, char);
            self.rows.push(row);
        } else {
            let row = &mut self.rows[y];
            if char == '\t' {
                row.insert_str(at.x, "    ");
            } else {
                row.insert(at.x, char);
            }
        }
        self.modified = true;
    }

    /// Inserts a newline at the given `Position`.
    pub fn insert_newline(&mut self, at: &Position) {
        let y = at.y;
        if y > self.rows.len() {
            return;
        }
        if y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }
        let current_row = &mut self.rows[y];
        let new_row = current_row.split(at.x);
        self.rows.insert(y + 1, new_row);
        self.modified = true;
    }

    /// Deletes the character at the given `Position`.
    pub fn delete(&mut self, at: &Position) {
        if at.y >= self.rows.len() {
            return;
        }
        if at.x == self.rows[at.y].len() && at.y + 1 < self.rows.len() {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.add(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
        }
        self.modified = true;
    }

    /// Finds the first occurrence of `query` in the buffer starting at `at`.
    pub fn find(&self, query: &str, at: &Position) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }
        let mut x_pos = at.x;
        for y in at.y..self.rows.len() {
            if let Some(row) = self.rows.get(y) {
                if let Some(x) = row.find(query, x_pos) {
                    return Some(Position { x, y });
                }
                x_pos = 0;
            } else {
                return None;
            }
        }
        None
    }

}
