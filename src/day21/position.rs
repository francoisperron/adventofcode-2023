#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize
}

impl Position {
    pub fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }

    pub fn around(&self) -> Vec<Position> {
        vec![
            Position::new(self.x, self.y - 1),
            Position::new(self.x, self.y + 1),
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
        ]
    }
}