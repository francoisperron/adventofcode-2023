use itertools::Itertools;
use crate::day18::direction::Direction;
use crate::day18::direction::Direction::{Down, Left, Right, Up};

#[derive(Debug, PartialEq)]
pub struct DigStep {
    pub direction: Direction,
    pub meters: isize,
}

impl DigStep {
    pub fn from(input: &str) -> DigStep {
        let (direction, meters, _color) = input.split_whitespace().collect_tuple().unwrap();
        DigStep { direction: Direction::from(direction), meters: meters.parse().unwrap() }
    }

    pub fn from_color(input: &str) -> DigStep {
        let (_, color) = input.split_once(" (#").unwrap();
        let direction = &color[color.len() - 2..color.len() - 1];
        let meters = &color[0..color.len() - 2];
        DigStep { direction: Direction::from_color(direction), meters: isize::from_str_radix(meters, 16).unwrap() }
    }

    pub fn dig(&self, position: &mut (isize, isize)) {
        match self.direction {
            Up => position.0 -= self.meters,
            Right => position.1 += self.meters,
            Down => position.0 += self.meters,
            Left => position.1 -= self.meters
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::day18::dig_step::DigStep;

    #[test]
    fn digs_up() {
        let mut p = (1, 2);

        DigStep::from("U 6 (#70c710)").dig(&mut p);
        assert_eq!(p, (1 - 6, 2))
    }

    #[test]
    fn digs_right() {
        let mut p = (1, 2);

        DigStep::from("R 6 (#70c710)").dig(&mut p);
        assert_eq!(p, (1, 2 + 6))
    }

    #[test]
    fn digs_down() {
        let mut p = (1, 2);

        DigStep::from("D 6 (#70c710)").dig(&mut p);
        assert_eq!(p, (1 + 6, 2))
    }

    #[test]
    fn digs_left() {
        let mut p = (1, 2);

        DigStep::from("L 6 (#70c710)").dig(&mut p);
        assert_eq!(p, (1, 2 - 6))
    }
}
