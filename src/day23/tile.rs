use Tile::{Forest, Path, Slope};
use crate::day23::tile::Direction::{Down, Left, Right, Up};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Path,
    Forest,
    Slope(Direction),
    Walked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Tile {
    pub fn from(tile: char) -> Tile {
        match tile {
            '.' => Path,
            '#' => Forest,
            '<' => Slope(Left),
            '>' => Slope(Right),
            'v' => Slope(Down),
            '^' => Slope(Up),
            _ => panic!()
        }
    }

    pub fn from_no_slopes(tile: char) -> Tile {
        match tile {
            '.' | '<' | '>' | 'v' | '^' => Path,
            '#' => Forest,
            _ => panic!()
        }
    }

    pub fn is_clear(&self, direction: Direction) -> bool {
        matches!(
            (direction, self),
            (Up, Path | Slope(Up)) |
            (Down, Path | Slope(Down)) |
            (Left, Path | Slope(Left)) |
            (Right, Path | Slope(Right))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_if_the_tile_is_clear() {
        assert!(Path.is_clear(Up));
        assert!(Slope(Up).is_clear(Up));

        assert!(Path.is_clear(Down));
        assert!(Slope(Down).is_clear(Down));

        assert!(Path.is_clear(Left));
        assert!(Slope(Left).is_clear(Left));

        assert!(Path.is_clear(Right));
        assert!(Slope(Right).is_clear(Right));
    }
}