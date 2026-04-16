use crate::editor::{
    TITLE, VERSION,
    buffer::TextBuffer,
    color_pairs::ColorPairs,
    settings::{EditorSettings, TabType},
    position::{Position, Size},
    status_message::StatusMessage
};
use crossterm::{
    ExecutableCommand,
    QueueableCommand,
    cursor,
    event::KeyCode,
    style,
    terminal::{self, ClearType}
};
use std::{io::{Stdout, Write, stdout}, time::Instant};


pub struct TerminalView {
    config: EditorSettings,
    stdout: Stdout,
    buffer: TextBuffer,
    position: Position,
    offset: Position,
    size: Size,
    colors: ColorPairs,
    start_time: Instant,
    messages: Vec<StatusMessage>
}

impl TerminalView {

    /// Creates a new `TerminalView` instance with default settings.
    pub fn new() -> TerminalView {
        let mut tv = TerminalView {
            config: EditorSettings::default(),
            stdout: stdout(),
            buffer: TextBuffer::new(),
            position: pos!(0, 0),
            offset: pos!(0, 1), // leave space for line nummbers and title bar
            size: size!(0, 0),
            colors: ColorPairs::new(),
            start_time: Instant::now(),
            messages: Vec::new()
        };
        if terminal::enable_raw_mode().is_err() {
            eprintln!("{}: Error, can't enable terminal raw mode.", TITLE);
            std::process::exit(1);
        }
        if tv.stdout.execute(terminal::EnterAlternateScreen).is_err() {
            eprintln!("{}: Error, can't enter alternate screen.", TITLE);
            terminal::disable_raw_mode().unwrap();
            std::process::exit(2);
        }
        tv.size = tv.terminal_size();
        tv.size.height -= 2; // leave space for title and status bar
        tv.add_msg(&format!("Welcome to {} v{}", TITLE, VERSION));
        tv
    }

    pub fn quit(&mut self) {
        if self.stdout.execute(terminal::LeaveAlternateScreen).is_err() {
            eprintln!("{}: Error, can't leave alternate screen.", TITLE);
        }
        if terminal::disable_raw_mode().is_err() {
            eprintln!("{}: Error, can't disable terminal raw mode.", TITLE);
            std::process::exit(3);
        }
        println!("Message Log:");
        for msg in &self.messages {
            println!("{}", msg.get());
        }
        println!("Text Buffer: {} entries", self.buffer.len());
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.size.width = width;
        self.size.height = height;
        self.size.height -= 2; // leave space for title and status bar
    }

    fn terminal_size(&self) -> Size {
        let (w,h) = crossterm::terminal::size().unwrap_or((80, 25));
        size!(w as usize, h as usize)
    }

    pub fn open_file(&mut self, filename: &str) {
        self.buffer = if let Some(buffer) = TextBuffer::open(filename) {
            self.add_msg(&format!("Loaded {} successfully.", filename));
            buffer
        } else {
            self.add_msg(&format!("Failed to load {}.", filename));
            TextBuffer::new()
        };
    }

    pub fn save_file(&mut self, filename: &str) {
        self.buffer.save(filename);
    }

    pub fn add_msg(&mut self, message: &str) {
        self.messages.push(StatusMessage::new(message));
    }

    fn get_msg(&self) -> Option<&StatusMessage> {
        self.messages.last()
    }

    pub fn render(&mut self) {
        self.use_colors(self.colors.default_pair());
        self.clear_screen();
        self.print_titlebar();
        // Draw the buffer rows
        self.use_colors(self.colors.default_pair());
        for y in 0..self.size.height {
            if self.buffer.is_empty() {
                break;
            }
            let row = match self.buffer.row(y) {
                Some(row) => row,
                None => break,
            };
            let start = if row.len() >= self.size.width && self.offset.x + self.position.x >= self.size.width {
                (self.offset.x + self.position.x) - self.size.width
            } else {
                self.offset.x
            };
            let end = start + self.size.width - 1;
            let line = row.get(start, end);
            if y  == self.position.y {
                // highlight the current line
                self.use_colors(self.colors.hilite_pair());
                self.clear_line(y + self.offset.y);
                self.print_at(pos!(0, y + self.offset.y), &line);
                self.use_colors(self.colors.default_pair());
            } else {
                self.clear_line(y + self.offset.y);
                self.print_at(pos!(0, y + self.offset.y), &line);
            }
        }
        self.print_statusbar();
        self.move_to(self.position + self.offset);
        self.flush();
    }

    fn print_titlebar(&mut self) {
        let title: String = if self.buffer.is_modified() {
            format!("{} - {} [*]", TITLE, self.buffer.filename())
        } else {
            format!("{} - {}", TITLE, self.buffer.filename())
        };
        let x = (self.size.width - title.len()) / 2;
        self.use_colors(self.colors.bar_pair());
        self.clear_line(0);
        self.print_at(pos!(x, 0), &title);
    }

    fn print_statusbar(&mut self) {
        let status = self.get_msg();
        let mut msg_txt = String::new();
        if let Some(msg) = status && !msg.is_expired() {
            msg_txt = msg.get().to_string();
        }
        let pos_txt = format!("↓{} →{}", self.position.y + self.offset.y, self.position.x + self.offset.x);
        self.use_colors(self.colors.bar_pair());
        self.clear_line(self.size.height + 1);
        self.print_at(pos!(0, self.size.height + 1), &msg_txt);
        self.print_at(pos!(self.size.width - pos_txt.chars().count() - 2, self.size.height + 1), &pos_txt);
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn clear_screen(&mut self) {
        self.stdout.queue(terminal::Clear(ClearType::All)).unwrap();
    }

    fn clear_line(&mut self, line: usize) {
        self.move_to(pos!(0, line));
        self.stdout.queue(terminal::Clear(ClearType::CurrentLine)).unwrap();
    }

    fn move_to(&mut self, at: Position) {
        self.stdout.queue(cursor::MoveTo(at.x as u16, at.y as u16)).unwrap();
    }

    fn print(&mut self, text: &str) {
        self.stdout.queue(style::Print(text)).unwrap();
    }

    fn print_at(&mut self, at: Position, text: &str) {
        self.move_to(at);
        self.print(text);
    }

    fn use_colors(&mut self, colors: style::Colors) {
        self.stdout.queue(style::SetColors(colors)).unwrap();
    }

    pub fn insert_char(&mut self, char: char) {
        self.buffer.insert(&self.position, char);
        if char == '\t' && self.config.tab_type == TabType::Space {
            self.position.x += self.config.tab_size - 1;
        }
        self.move_cursor(KeyCode::Right);
    }

    pub fn insert_newline(&mut self) {
        self.buffer.insert_newline(&self.position);
        self.move_cursor(KeyCode::Right);
    }

    pub fn delete_char(&mut self) {
        self.buffer.delete(&self.position);
    }

    pub fn delete_char_before(&mut self) {
        self.move_cursor(KeyCode::Left);
        self.buffer.delete(&self.position);
    }

    pub fn move_cursor(&mut self, key_code: KeyCode) {
        let term_height = self.size.height;
        let mut cursor = self.position;
        let buff_height = self.buffer.len();
        let mut buff_width = if let Some(row) = self.buffer.row(cursor.y) {
            row.len()
        } else {
            0
        };
        match key_code {
            KeyCode::Up => {
                if cursor.y > 0 {
                    cursor.y -= 1;
                }
            },
            KeyCode::Down => {
                if cursor.y < buff_height {
                    cursor.y += 1;
                }
            },
            KeyCode::Left => {
                if cursor.x > 0 {
                    cursor.x -= 1;
                } else if cursor.y > 0 {
                    cursor.y -= 1;
                    if let Some(row) = self.buffer.row(cursor.y) {
                        cursor.x = row.len();
                    } else {
                        cursor.x = 0;
                    }
                }
            },
            KeyCode::Right => {
                if cursor.x < buff_width {
                    cursor.x += 1;
                } else if cursor.y < buff_height {
                    cursor.y += 1;
                    cursor.x = 0;
                }
            },
            KeyCode::PageUp => {
                if cursor.y > term_height {
                    cursor.y -= term_height;
                } else {
                    cursor.y = 0;
                }
            }
            KeyCode::PageDown => {
                if cursor.y + term_height < buff_height {
                    cursor.y += term_height;
                } else {
                    cursor.y = buff_height;
                }
            }
            KeyCode::Home => {
                cursor.x = 0;
            },
            KeyCode::End => {
                cursor.x = buff_width;
            },
            _ => {}
        }
        buff_width = if let Some(row) = self.buffer.row(cursor.y) {
            row.len()
        } else {
            0
        };
        if cursor.x > buff_width {
            cursor.x = buff_width;
        }
        self.position = cursor;
    }

}
