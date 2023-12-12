use Direction::{North, South, East, West};

#[cfg(test)]
mod tests {
    use crate::daily_input;
    use crate::day10::{Direction, Position, Sketch, Tile};
    use Direction::{North, South, East, West};

    const EXAMPLE_1: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[test]
    fn parses_input() {
        let sketch = Sketch::from(EXAMPLE_1);

        assert_eq!(sketch.tiles.len(), 25);
        assert_eq!(*sketch.tiles.get(0).unwrap(), Tile::new('-'));
        assert_eq!(*sketch.tiles.get(24).unwrap(), Tile::new('F'));
    }

    #[test]
    fn follows_vertical_pipes() {
        let vertical_pipe = Tile::new('|');
        assert_eq!(vertical_pipe.follow(&Position::new(11, North, 5)), Position::new(6, North, 5));
        assert_eq!(vertical_pipe.follow(&Position::new(11, South, 5)), Position::new(16, South, 5));
    }

    #[test]
    fn follows_horizontal_pipes() {
        let horizontal_pipe = Tile::new('-');
        assert_eq!(horizontal_pipe.follow(&Position::new(7, East, 5)), Position::new(8, East, 5));
        assert_eq!(horizontal_pipe.follow(&Position::new(7, West, 5)), Position::new(6, West, 5));
    }

    #[test]
    fn follows_north_to_est_pipes() {
        let south_to_west_pipe = Tile::new('L');
        assert_eq!(south_to_west_pipe.follow(&Position::new(16, South, 5)), Position::new(17, East, 5));
        assert_eq!(south_to_west_pipe.follow(&Position::new(16, West, 5)), Position::new(11, North, 5));
    }

    #[test]
    fn follows_north_to_west_pipes() {
        let south_to_west_pipe = Tile::new('J');
        assert_eq!(south_to_west_pipe.follow(&Position::new(18, South, 5)), Position::new(17, West, 5));
        assert_eq!(south_to_west_pipe.follow(&Position::new(18, East, 5)), Position::new(13, North, 5));
    }

    #[test]
    fn follows_south_to_west_pipes() {
        let south_to_west_pipe = Tile { tile: '7' };
        assert_eq!(south_to_west_pipe.follow(&Position::new(8, East, 5)), Position::new(13, South, 5));
        assert_eq!(south_to_west_pipe.follow(&Position::new(8, North, 5)), Position::new(7, West, 5));
    }

    #[test]
    fn follows_south_to_east_pipes() {
        let south_to_east_pipe = Tile { tile: 'F' };
        assert_eq!(south_to_east_pipe.follow(&Position::new(6, West, 5)), Position::new(11, South, 5));
        assert_eq!(south_to_east_pipe.follow(&Position::new(6, North, 5)), Position::new(7, East, 5));
    }

    #[test]
    fn solve_examples_part1() {
        let sketch = Sketch::from(EXAMPLE_1);

        assert_eq!(sketch.steps_to_reach_farthest_point(), 4);
    }

    #[test]
    fn solve_part1() {
        let sketch = Sketch::from(&daily_input(10));

        assert_eq!(sketch.steps_to_reach_farthest_point(), 6864);
    }

    #[test]
    fn solve_examples_part2() {
        let sketch = Sketch::from(EXAMPLE_1);

        assert_eq!(sketch.enclosed_tiles(), 1);
    }


    #[test]
    fn solves_part2() {
        let sketch = Sketch::from(&daily_input(10));

        assert_eq!(sketch.enclosed_tiles(), 349);
    }
}

struct Sketch {
    tiles: Vec<Tile>,
    length: usize,
}

impl Sketch {
    pub fn from(input: &str) -> Sketch {
        let length = input.find('\n').unwrap();

        let tiles = input.chars()
            .filter(|c| *c != '\n')
            .map(|tile| Tile { tile })
            .collect();

        Sketch { tiles, length }
    }

    pub fn steps_to_reach_farthest_point(&self) -> u32 {
        self.find_loop().0
    }

    pub fn enclosed_tiles(&self) -> u32 {
        self.find_loop().1
    }

    fn find_loop(&self) -> (u32, u32) {
        let start = self.tiles.iter().position(|t| t.tile == 'S').unwrap();

        let mut corner = Position::new(start, East, self.length);
        let mut position = Position::new(start + 1, East, self.length);
        let mut steps = 1;
        let mut area: i32 = 0;
        while self.tiles.get(position.index).unwrap().tile != 'S' {
            let tile = self.tiles.get(position.index).unwrap();
            if tile.tile == '7' || tile.tile == 'J' || tile.tile == 'F' || tile.tile == 'L' {
                area += Self::shoelace(&corner, &position);
                corner = position;
            }
            position = tile.follow(&position);
            steps += 1;
        }

        area += Self::shoelace(&corner, &position);
        ((steps / 2) as u32, (area / 2 - steps / 2 + 1) as u32)
    }

    fn shoelace(a: &Position, b: &Position) -> i32 {
        ((a.index % a.length) * (b.index / b.length)) as i32 - ((b.index % b.length) * (a.index / a.length)) as i32
    }
}

#[derive(PartialEq, Debug)]
struct Tile {
    tile: char,
}

impl Tile {
    pub fn new(tile: char) -> Tile {
        Tile { tile }
    }

    pub fn follow(&self, position: &Position) -> Position {
        match (self.tile, &position.direction) {
            ('|', North) => Position::new(position.index - position.length, North, position.length),
            ('|', South) => Position::new(position.index + position.length, South, position.length),

            ('-', West) => Position::new(position.index - 1, West, position.length),
            ('-', East) => Position::new(position.index + 1, East, position.length),

            ('L', South) => Position::new(position.index + 1, East, position.length),
            ('L', West) => Position::new(position.index - position.length, North, position.length),

            ('J', South) => Position::new(position.index - 1, West, position.length),
            ('J', East) => Position::new(position.index - position.length, North, position.length),

            ('7', East) => Position::new(position.index + position.length, South, position.length),
            ('7', North) => Position::new(position.index - 1, West, position.length),

            ('F', West) => Position::new(position.index + position.length, South, position.length),
            ('F', North) => Position::new(position.index + 1, East, position.length),

            _ => panic!("dead end")
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Position {
    index: usize,
    direction: Direction,
    length: usize,
}

impl Position {
    pub fn new(index: usize, direction: Direction, length: usize) -> Position {
        Position { index, direction, length }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}
