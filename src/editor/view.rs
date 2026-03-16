use crate::TITLE;
use crate::editor::row;
use crossterm::{ExecutableCommand, QueueableCommand, terminal};
use std::io::{stdout, Stdout, Write};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub rows: usize,
    pub cols: usize,
}

pub struct View {
    stdout: Stdout,
    pub pos: Position,
    pub size: Size,
    pub row_vec: Vec<row::Row>,
    pub statusbar: bool,
}

impl View {

    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap_or_else(|err| {
            eprintln!("{}: Error, can't enable raw mode: {}", TITLE, err);
            std::process::exit(1);
        });
        let mut result = Self {
            stdout: stdout(),
            pos: Position { x: 0, y: 1 },
            size: Size { rows: 0, cols: 0 },
            row_vec: Vec::new(),
            statusbar: true,
        };
        result.stdout.execute(terminal::EnterAlternateScreen).unwrap_or_else(|err| {
            eprintln!("{}: Error, can't enter alternate screen: {}", TITLE, err);
            std::process::exit(1);
        });
        result
    }

    pub fn render(&mut self) {

    }

    pub fn exit(&mut self) {
        self.stdout.execute(terminal::LeaveAlternateScreen).unwrap_or_else(|err| {
            eprintln!("{}: Error, can't leave alternate screen: {}", TITLE, err);
            std::process::exit(1);
        });
        terminal::disable_raw_mode().unwrap_or_else(|err| {
            eprintln!("{}: Error, can't enable raw mode: {}", TITLE, err);
            std::process::exit(1);
        });
    }

    pub fn update(&mut self) {

    }


    fn bar(&mut self, length: usize) -> String {
        let mut bar = String::new();
        for _ in 0..length {
            bar.push(' ');
        }
        bar
    }

}
