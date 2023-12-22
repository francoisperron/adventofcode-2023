#[derive(Eq, PartialEq, Hash, Debug, Clone, Ord, PartialOrd)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }
}