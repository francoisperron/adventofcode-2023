use crate::day17::direction::Direction;
use crate::day17::position::Position;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Block {
    pub cost: usize,
    pub position: Position,
    pub direction: Direction,
}

impl Block {
    pub fn new(cost: usize, position: Position, direction: Direction) -> Block {
        Block { cost, position, direction }
    }

    pub fn hash(&self) -> (Position, Direction) {
        (self.position, self.direction)
    }
}