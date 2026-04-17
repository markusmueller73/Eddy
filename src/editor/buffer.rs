use crate::editor::{row::Row, position::Position};
use std::{fs::File, io::{BufRead, BufReader, BufWriter, Write}};

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

    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            modified: false,
            file_name: String::from(crate::editor::DEFAULT_FILENAME),
        }
    }

    pub fn open(file_name: &str) -> Option<TextBuffer> {
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
                            eprintln!("Error: can't read line ({})", err);
                        }
                    }
                }
                return Some(TextBuffer{
                    rows: row_vec,
                    modified: false,
                    file_name: file_name.to_string(),
                })
            },
            Err(err) => {
                eprintln!("Error: can't open file {} ({}).", file_name, err);
            }
        }
        None
    }

    pub fn save(&mut self, file_name: &str) {
        match File::create(file_name) {
            Ok(file) => {
                let mut buf_writer = BufWriter::new(file);
                for row in &self.rows {
                    let row_string = row.as_string();
                    if buf_writer.write_all(row_string.as_bytes()).is_err() {
                        eprintln!("Error: can't write to file {}.", file_name);
                    }
                    if buf_writer.write_all(EOL.as_bytes()).is_err() {
                        eprintln!("Error: can't write EOL to file {}.", file_name);
                    }
                }
                if buf_writer.flush().is_err() {
                    eprintln!("Error: can't flush file {}.", file_name);
                }
                self.modified = false;
            }
            Err(err) => {
                eprintln!("Error: can't create file: {} ({})", file_name, err);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn filename(&self) -> &str {
        &self.file_name
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn get_range(&self, start: &Position, end: &Position) -> String {
        let mut result = String::new();
        for y in start.y..=end.y {
            if let Some(row) = self.rows.get(y) {
                if y == start.y && y == end.y {
                    result.push_str(&row.get(start.x, end.x));
                } else if y == start.y {
                    result.push_str(&row.get(start.x, row.len().saturating_sub(1)));
                } else if y == end.y {
                    result.push_str(&row.get(0, end.x));
                } else {
                    result.push_str(&row.get(0, row.len().saturating_sub(1)));
                }
            }
        }
        result
    }

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
