use crate::day17::direction::Direction;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn move_to(&self, direction: &Direction, moves: usize) -> Position {
        let new_row = (self.0 as isize + direction.0 * moves as isize) as usize;
        let new_col = (self.1 as isize + direction.1 * moves as isize) as usize;
        Position(new_row, new_col)
    }
}

#[cfg(test)]
mod tests {
    use crate::day17::direction::{DOWN, LEFT, RIGHT, UP};
    use crate::day17::position::Position;

    #[test]
    fn moves_in_given_direction() {
        let p1 = Position(1,2);

        assert_eq!(p1.move_to(&UP, 1), Position(0, 2));
        assert_eq!(p1.move_to(&DOWN, 2), Position(3, 2));
        assert_eq!(p1.move_to(&LEFT, 1), Position(1, 1));
        assert_eq!(p1.move_to(&RIGHT, 2), Position(1, 4));
    }
}