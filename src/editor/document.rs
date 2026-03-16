use std::fs::{File};
use std::io::{BufReader, BufRead, BufWriter, Write};

#[cfg(target_family = "windows")]
pub const EOL: &str = "\r\n";
#[cfg(not(target_family = "windows"))]
pub const EOL: &str = "\n";

#[derive(Default)]
pub struct Document {
    content: Vec<String>,
    rows: usize,
    chars: usize,
    words: usize,
    filename: String,
}

impl Document {

    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            rows: 0,
            chars: 0,
            words: 0,
            filename: String::from("<Untitled>"),
        }
    }

    pub fn load(&mut self, filename: &str) -> bool {

        match File::open(filename) {

            Ok(file) => {
                self.filename = filename.to_string();
                // Use Rusts BufReader (a buffered reader) to read the file line by line
                // The BufReader is faster than the default File reader
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    // Check if the line can be read successfully
                    match line {
                        Ok(line) => {
                            // A very simple hack to count the 'words' in the line
                            self.words += line.chars().filter(|c| c.is_ascii_whitespace()).count();
                            // Count all chars, including whitespaces
                            self.chars += line.len();
                            self.rows += 1;
                            self.content.push(line);
                        }
                        Err(err) => {
                            eprint!(
                                "{}: Error while reading file '{}', can't read line: ({}).",
                                crate::TITLE,
                                filename,
                                err
                            );
                            continue;
                        }
                    }
                }
            }

            Err(err) => {
                eprint!("{}: Error while opening file '{}': {}", crate::TITLE, filename, err);
                return false;
            }

        }

        true

    }

    pub fn save(&mut self, filename: &str) -> bool {

        match File::create(filename) {

            Ok(file) => {

                self.filename = filename.to_string();

                let mut writer = BufWriter::new(file);
                for line in &self.content {
                    match writer.write_all(line.as_bytes()) {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("{}: Error while saving file '{}', {}.", crate::TITLE, filename, err);
                            return false;
                        }
                    } // match writer.write_all(line.as_bytes())
                } // for line in &self.content

                match writer.flush() {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{}: Error while saving file '{}', {}.", crate::TITLE, filename, err);
                        return false;
                    }
                } // match writer.flush()

            }

            Err(err) => {
                eprintln!("{}: Error while saving file '{}', {}.", crate::TITLE, filename, err);
                return false;
            }

        } // match File::create(filename)

        true

    }

    pub fn content(&self, row: usize) -> Option<&String> {
        self.content.get(row)
    }

    pub fn chars(&self) -> usize {
        self.chars
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

}
