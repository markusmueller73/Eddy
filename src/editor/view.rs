use crate::{
    editor::{
        TITLE,
        buffer::TextBuffer,
        color_pairs::ColorPairs,
        position::{Position, Size},
        status_input::StatusInput,
        status_message::StatusMessage,
    },
};
use crossterm::{
    ExecutableCommand,
    QueueableCommand,
    clipboard::{CopyToClipboard},
    cursor,
    event::KeyCode,
    style,
    terminal::{self, ClearType}
};
use std::io::{Stdout, Write, stdout};

pub struct TerminalView {
    stdout: Stdout,
    position: Position,
    offset: Position,
    marking: bool,
    marking_start: Position,
    marking_end: Position,
    size: Size,
    colors: ColorPairs,
}

impl TerminalView {

    /// Creates a new `TerminalView` instance with default settings.
    pub fn new() -> TerminalView {
        let mut tv = TerminalView {
            stdout: stdout(),
            position: Position::default(),
            offset: pos!(0, 1), // leave space for line nummbers and title bar
            marking: false,
            marking_start: Position::default(),
            marking_end: Position::default(),
            size: Size::default(),
            colors: ColorPairs::new(),
        };
        if terminal::enable_raw_mode().is_err() {
            error!("<TerminalView::new>: can't enable terminal raw mode.");
            std::process::exit(1);
        }
        if tv.stdout.execute(terminal::EnterAlternateScreen).is_err() {
            error!("<TerminalView::new>: can't enter alternate screen.");
            terminal::disable_raw_mode().unwrap();
            std::process::exit(2);
        }
        tv.size = tv.terminal_size();
        debug!("<TerminalView::new>: size={}x{}", tv.size.width, tv.size.height);
        tv.size.height -= 2; // leave space for title and status bar
        tv
    }

    pub fn quit(&mut self) {
        if self.stdout.execute(terminal::LeaveAlternateScreen).is_err() {
            error!("<TerminalView::quit>: can't leave alternate screen.");
            std::process::exit(2);
        }
        if terminal::disable_raw_mode().is_err() {
            error!("<TerminalView::quit>: can't disable terminal raw mode.");
            std::process::exit(1);
        }
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

    pub fn position(&self) -> Position {
        self.position
    }

    //pub fn render(&mut self, buffer: &TextBuffer, last_message: &str) {
    pub fn render(&mut self, buffer: &TextBuffer, input: &StatusInput, message: &StatusMessage) {
        self.use_colors(self.colors.default_pair());
        self.clear_screen();
        self.print_titlebar(buffer);
        // Draw the buffer rows
        self.use_colors(self.colors.default_pair());
        for y in 0..self.size.height {
            if buffer.is_empty() {
                break;
            }
            let row = match buffer.row(y) {
                Some(row) => row,
                None => break,
            };
            let start = if row.len() >= self.size.width && self.offset.x + self.position.x >= self.size.width {
                (self.offset.x + self.position.x) - self.size.width
            } else {
                self.offset.x
            };
            let end = start + self.size.width - 1;
            let line = row.get_range(start, end);
            if y  == self.position.y {
                // highlight the current line
                self.use_colors(self.colors.hilite_pair());
                self.clear_line(y + self.offset.y);
                self.print_at(pos!(0, y + self.offset.y), &line);
            } else {
                self.use_colors(self.colors.default_pair());
                self.clear_line(y + self.offset.y);
                self.print_at(pos!(0, y + self.offset.y), &line);
            }
            if self.is_marking() {
                self.use_colors(self.colors.marker_pair());
                let len = line.len().saturating_sub(1);
                if y == self.marking_start.y && y == self.marking_end.y {
                    for x in self.marking_start.x..=self.marking_end.x {
                        let ch = line.chars().nth(x).unwrap_or(' ').to_string();
                        self.print_at(pos!(x, y + self.offset.y), &ch);
                    }
                } else if  y == self.marking_start.y {
                    for x in self.marking_start.x..=len {
                        let ch = line.chars().nth(x).unwrap_or(' ').to_string();
                        self.print_at(pos!(x, y + self.offset.y), &ch);
                    }
                } else if y == self.marking_end.y {
                    for x in 0..=self.marking_end.x {
                        let ch = line.chars().nth(x).unwrap_or(' ').to_string();
                        self.print_at(pos!(x, y + self.offset.y), &ch);
                    }
                } else if y > self.marking_start.y && y < self.marking_end.y {
                    self.print_at(pos!(0, y + self.offset.y), &line);
                }
                self.use_colors(self.colors.default_pair());
            }
        }
        self.print_statusbar(input, message);
        self.move_to(self.position + self.offset);
        self.flush();
    }

    fn print_titlebar(&mut self, buffer: &TextBuffer) {
        let title: String = if buffer.is_modified() {
            format!("{} - {} [*]", TITLE, buffer.filename())
        } else {
            format!("{} - {}", TITLE, buffer.filename())
        };
        let x = (self.size.width - title.len()) / 2;
        self.use_colors(self.colors.bar_pair());
        self.clear_line(0);
        self.print_at(pos!(x, 0), &title);
    }

    fn print_statusbar(&mut self, input: &StatusInput, message: &StatusMessage) {
        let pos_txt = format!("yx: ↓{} →{}", self.position.y, self.position.x + 1);
        let mark_txt = format!("[yx]: {} → {}", self.marking_start, self.marking_end);
        let mode_txt = format!("{}", input.get_mode());
        self.use_colors(self.colors.bar_pair());
        let y_pos = self.size.height + 1;
        self.clear_line(y_pos);
        self.print_at(pos!(self.size.width - 8, y_pos), &mode_txt);
        self.print_at(pos!(self.size.width - pos_txt.chars().count() - 10, y_pos), &pos_txt);
        if self.is_marking() {
            self.print_at(pos!(self.size.width - mark_txt.chars().count() - pos_txt.chars().count() - 12, self.size.height + 1), &mark_txt);
        }
        if input.is_active() {
            self.print_at(pos!(0, y_pos), &input.get_content());
        } else {
            if !message.is_expired() {
                self.print_at(pos!(0, y_pos), message.get());
            }
        }
    }

    fn flush(&mut self) {
        if self.stdout.flush().is_err() {
            warning!("<TerminalView::flush> fails.");
        }
    }

    fn clear_screen(&mut self) {
        if self.stdout.queue(terminal::Clear(ClearType::All)).is_err() {
            warning!("<TerminalView::clear_screen> fails.");
        }
    }

    fn clear_line(&mut self, line: usize) {
        self.move_to(pos!(0, line));
        if self.stdout.queue(terminal::Clear(ClearType::CurrentLine)).is_err() {
            warning!("<TerminalView::clear_line> fails.");
        }
    }

    fn move_to(&mut self, at: Position) {
        if self.stdout.queue(cursor::MoveTo(at.x as u16, at.y as u16)).is_err() {
            warning!("<TerminalView::move_to> fails.");
        }
    }

    fn print(&mut self, text: &str) {
        if self.stdout.queue(style::Print(text)).is_err() {
            warning!("<TerminalView::print> fails.");
        }
    }

    fn print_at(&mut self, at: Position, text: &str) {
        self.move_to(at);
        self.print(text);
    }

    fn use_colors(&mut self, colors: style::Colors) {
        if self.stdout.queue(style::SetColors(colors)).is_err() {
            warning!("<TerminalView::use_colors> fails.");
        }
    }

    pub fn place_cursor(&mut self, position: Position) {
        self.position = position;
        self.move_to(position);
    }

    pub fn move_cursor(&mut self, buffer: &TextBuffer, key_code: KeyCode) {
        let term_height = self.size.height;
        let mut cursor = self.position;
        let buff_height = buffer.len();
        let mut buff_width = if let Some(row) = buffer.row(cursor.y) {
            debug!("<TerminalView::move_cursor> row.len() = {}", row.len());
            row.len()
        } else {
            debug!("<TerminalView::move_cursor> row is empty");
            0
        };
        match key_code {
            KeyCode::Up => {
                cursor.y = cursor.y.saturating_sub(1);
            },
            KeyCode::Down => {
                cursor.y = if cursor.y < buff_height {
                    cursor.y.saturating_add(1)
                } else {
                    cursor.y
                };
            },
            KeyCode::Left => {
                if cursor.x > 0 {
                    cursor.x -= 1;
                } else if cursor.y > 0 {
                    cursor.y -= 1;
                    if let Some(row) = buffer.row(cursor.y) {
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
        buff_width = if let Some(row) = buffer.row(cursor.y) {
            row.len()
        } else {
            0
        };
        if cursor.x > buff_width {
            cursor.x = buff_width;
        }
        self.position = cursor;
    }

    pub fn is_marking(&self) -> bool {
        self.marking
    }

    pub fn start_marking(&mut self) {
        self.marking = true;
        self.marking_start = self.position;
    }

    pub fn end_marking(&mut self) {
        self.marking_end = self.position;
    }

    pub fn get_marked_positions(&self) -> (Position,Position) {
        (self.marking_start, self.marking_end)
    }

    pub fn copy_text_to_clipboard(&mut self, text: &str) {
        if self.stdout.execute(CopyToClipboard::to_clipboard_from(text)).is_err() {
            warning!("<TerminalView::copy_text_to_clipboard> fails.");
        }
    }

    pub fn paste_from_clipboard(&mut self, at: &Position) {
        todo!("Paste from clipboard not implemented.");
    }

    pub fn reset_marking(&mut self) {
        self.marking = false;
        self.marking_start = Position::default();
        self.marking_end = Position::default();
    }

    pub fn user_input_move_cursor(&mut self, min_x: usize, length: usize, key_code: KeyCode) {
        let term_width = self.size.width;
        let mut cursor = self.position;
        if key_code == KeyCode::Left && cursor.x > min_x {
            cursor.x -= 1;
        } else if key_code == KeyCode::Right && cursor.x < min_x + length {
            cursor.x += 1;
        }
        if cursor.x > term_width - 1 {
            cursor.x = term_width - 1;
        }
        self.position = cursor;
    }

}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum EditMode {
    InputFind,
    InputLoad,
    InputReplace,
    InputSaveAs,
    Insert,
    #[default]
    Normal,
}

impl std::fmt::Display for EditMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditMode::Insert => write!(f, "INSERT"),
            EditMode::Normal => write!(f, "NORMAL"),
            _ => write!(f, "INPUT"),
        }
    }
}
