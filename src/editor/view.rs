use crate::editor::{
    TITLE, VERSION,
    buffer::TextBuffer,
    color_pairs::ColorPairs,
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
use std::io::{Stdout, Write, stdout};


pub struct TerminalView {
    stdout: Stdout,
    buffer: TextBuffer,
    position: Position,
    offset: Position,
    size: Size,
    colors: ColorPairs,
    messages: Vec<StatusMessage>
}

impl TerminalView {

    pub fn new() -> TerminalView {
        let mut tv = TerminalView {
            stdout: stdout(),
            buffer: TextBuffer::new(),
            position: pos(0, 1),
            offset: pos(0, 0),
            size: size(0, 0),
            colors: ColorPairs::new(),
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
    }

    fn terminal_size(&self) -> Size {
        let (w,h) = crossterm::terminal::size().unwrap_or((80, 25));
        size(w as usize, h as usize)
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
            let row = match self.buffer.row(self.offset.y + y) {
                Some(row) => row,
                None => break,
            };
            let line = row.get(0, self.size.width-1);
            self.clear_line(y+1);
            self.print_at(pos(0, y+1), &line);
        }
        self.print_statusbar();
        self.move_to(self.position);
        self.flush();
    }

    fn print_titlebar(&mut self) {
        let bar = TerminalView::bar(self.size.width);
        self.use_colors(self.colors.bar_pair());
        self.print_at(pos(0, 0), &bar);
        let title = format!("{} - {}", TITLE, self.buffer.filename());
        let x = (self.size.width - title.len()) / 2;
        self.print_at(pos(x, 0), &title);
    }

    fn print_statusbar(&mut self) {
        let bar = TerminalView::bar(self.size.width);
        let status = self.get_msg();
        let mut msg_txt = String::new();
        if let Some(msg) = status { //&& msg.is_expired() {
            msg_txt = msg.get().to_string();
        }
        let pos_txt = format!("↓{} →{}", self.position.x + 1, self.position.y);
        self.use_colors(self.colors.bar_pair());
        self.print_at(pos(0, self.size.height + 1), &bar);
        self.print_at(pos(0, self.size.height + 1), &msg_txt);
        self.print_at(pos(self.size.width - pos_txt.chars().count() - 2, self.size.height + 1), &pos_txt);
    }

    fn bar(length: usize) -> String {
        let mut string = String::new();
        for _ in 0..length {
            string.push(' ');
        }
        string
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn clear_screen(&mut self) {
        self.stdout.queue(terminal::Clear(ClearType::All)).unwrap();
    }

    fn clear_line(&mut self, line: usize) {
        self.stdout.queue(cursor::MoveTo(0, line as u16)).unwrap();
        self.stdout.queue(terminal::Clear(ClearType::CurrentLine)).unwrap();
    }

    fn move_to(&mut self, at: Position) {
        self.stdout.queue(cursor::MoveTo(at.x as u16, at.y as u16)).unwrap();
    }

    fn print_at(&mut self, at: Position, text: &str) {
        self.stdout.queue(cursor::MoveTo(at.x as u16, at.y as u16)).unwrap();
        self.stdout.queue(style::Print(text)).unwrap();
    }

    fn use_colors(&mut self, colors: style::Colors) {
        self.stdout.queue(style::SetColors(colors)).unwrap();
    }

    pub fn move_cursor(&mut self, key_code: KeyCode) {
        let term_height = self.size.height;
        let mut term_pos = self.position;
        let buff_height = self.buffer.len();
        let mut buff_width = if let Some(row) = self.buffer.row(term_pos.y) {
            row.len()
        } else {
            0
        };
        match key_code {
            KeyCode::Up => {
                if term_pos.y > 0 {
                    term_pos.y -= 1;
                }
            },
            KeyCode::Down => {
                term_pos.y += 1;
            },
            KeyCode::Left => {
                if term_pos.x > 0 {
                    term_pos.x -= 1;
                } else if term_pos.y > 0 {
                    term_pos.y -= 1;
                    if let Some(row) = self.buffer.row(term_pos.y) {
                        term_pos.x = row.len();
                    } else {
                        term_pos.x = 0;
                    }
                }
            },
            KeyCode::Right => {
                if term_pos.x < buff_width {
                    term_pos.x += 1;
                } else if term_pos.y < buff_height {
                    term_pos.y += 1;
                    term_pos.x = 0;
                }
            },
            KeyCode::PageUp => {
                if term_pos.y > term_height {
                    term_pos.y -= term_height;
                } else {
                    term_pos.y = 0;
                }
            }
            KeyCode::PageDown => {
                if term_pos.y + term_height < buff_height {
                    term_pos.y += term_height;
                } else {
                    term_pos.y = term_height;
                }
            }
            KeyCode::Home => {
                term_pos.x = 0;
            },
            KeyCode::End => {
                term_pos.x = buff_width;
            },
            _ => {}
        }
        buff_width = if let Some(row) = self.buffer.row(term_pos.y) {
            row.len()
        } else {
            0
        };
        if term_pos.x > buff_width {
            term_pos.x = buff_width;
        }
        self.position = term_pos;
    }
}

#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub fn pos(x: usize, y: usize) -> Position {
    Position {x, y}
}

#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub fn size(cols: usize, rows: usize) -> Size {
    Size {width: cols, height: rows}
}
