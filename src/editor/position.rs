// Part of Eddy - A lightweight text editor for the terminal.
//! Position, representing a cursor position in the editor.
#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// `Add` trait to add two `Position` structs together.
impl std::ops::Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// `Sub` trait to subtract two `Position` structs.
impl std::ops::Sub for Position {
    type Output = Position;
    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// `Display` trait to print out a `Position` in the editor.
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}x{})", self.x, self.y)
    }
}

/// Macro for creating a `Position` struct.
#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        Position { x: $x, y: $y }
    };
}

/// Size, representing the width and height of a region in the editor.
#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

/// Macro for creating a `Size` struct.
#[macro_export]
macro_rules! size {
    ($cols:expr, $rows:expr) => {
        Size { width: $cols, height: $rows }
    };
}
