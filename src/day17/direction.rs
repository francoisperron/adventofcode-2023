#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Direction(pub isize, pub isize);

impl Direction {
    pub fn on_other_axis(&self, other: &Direction) -> bool {
        self != other && self != &Direction(-other.0, -other.1)
    }
}

pub const START: Direction = Direction(0, 0);
pub const UP: Direction = Direction(-1, 0);
pub const DOWN: Direction = Direction(1, 0);
pub const LEFT: Direction = Direction(0, -1);
pub const RIGHT: Direction = Direction(0, 1);
pub const DIRECTIONS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];

#[cfg(test)]
mod tests {
    use crate::day17::direction::{DOWN, LEFT, RIGHT, UP};

    #[test]
    fn checks_if_direction_is_on_other_axis() {
        assert!(!UP.on_other_axis(&DOWN));
        assert!(!DOWN.on_other_axis(&UP));
        assert!(!LEFT.on_other_axis(&RIGHT));
        assert!(!RIGHT.on_other_axis(&LEFT));
    }
}