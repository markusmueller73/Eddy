#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl std::ops::Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Position;
    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}x{})", self.x, self.y)
    }
}

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        Position { x: $x, y: $y }
    };
}

#[derive(Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[macro_export]
macro_rules! size {
    ($cols:expr, $rows:expr) => {
        Size { width: $cols, height: $rows }
    };
}
