use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::daily_input;
    use crate::day16::{Beam, Direction, Grid, Position, Tile};

    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn parses_grid() {
        let grid = Grid::from(".|-\n\\/.");

        assert_eq!(grid.tiles, HashMap::from([
            (Position::new(0, 0), Tile::EmptySpace),
            (Position::new(1, 0), Tile::VerticalMirror),
            (Position::new(2, 0), Tile::HorizontalMirror),
            (Position::new(0, 1), Tile::BackSlashMirror),
            (Position::new(1, 1), Tile::ForwardSlashMirror),
            (Position::new(2, 1), Tile::EmptySpace),
        ]));
    }

    #[test]
    fn beam_travels_in_empty_space() {
        let grid = Grid::from(".");
        let mut beam = Beam::new(0, 0, Direction::Right);

        beam.start_travel(&grid);

        assert_eq!(beam.energized, vec![Position::new(0, 0)]);
    }

    #[test]
    fn beam_travels_in_vertical_tile() {
        let grid = Grid::from("|\n|");
        let mut beam = Beam::new(0, 0, Direction::Right);

        beam.start_travel(&grid);

        assert_eq!(beam.energized, vec![Position::new(0, 0), Position::new(0, 1)]);
    }

    #[test]
    fn beam_travels_in_horizontal_tile() {
        let grid = Grid::from("--");
        let mut beam = Beam::new(0, 0, Direction::Down);

        beam.start_travel(&grid);

        assert_eq!(beam.energized, vec![Position::new(0, 0), Position::new(1, 0)]);
    }

    #[test]
    fn beam_travels_in_back_slash_tile() {
        let grid = Grid::from("-\\-");
        let mut beam = Beam::new(0, 0, Direction::Right);

        beam.start_travel(&grid);

        assert_eq!(beam.energized, vec![Position::new(0, 0), Position::new(1, 0)]);
    }

    #[test]
    fn beam_travels_in_forward_slash_tile() {
        let grid = Grid::from("-/-");
        let mut beam = Beam::new(0, 0, Direction::Right);

        beam.start_travel(&grid);

        assert_eq!(beam.energized, vec![Position::new(0, 0), Position::new(1, 0)]);
    }

    #[test]
    fn beam_stops_when_traveling_the_same_tile_on_the_same_direction_twice() {
        let input = "-\\\n\\/";
        let grid = Grid::from(input);
        println!("{:?}", grid.tiles);
        let mut beam = Beam::new(0, 0, Direction::Right);

        beam.start_travel(&grid);

        assert_eq!(beam.energized.len(), 4);
    }

    #[test]
    fn solves_example_part1() {
        let grid = Grid::from(EXAMPLE);
        let mut beam = Beam::new(0, 0, Direction::Right);

        beam.start_travel(&grid);

        assert_eq!(beam.energized.len(), 46);
    }

    #[test]
    fn solves_part1() {
        let grid = Grid::from(&daily_input(16));
        let mut beam = Beam::new(0, 0, Direction::Right);

        beam.start_travel(&grid);

        assert_eq!(beam.energized.len(), 8034);
    }

    #[test]
    fn solves_example_part2() {
        let grid = Grid::from(EXAMPLE);

        assert_eq!(max_energize(&grid), 51);
    }

    #[test]
    fn solves_part2() {
        let grid = Grid::from(&daily_input(16));

        assert_eq!(max_energize(&grid), 8225);
    }

    fn max_energize(grid: &Grid) -> usize {
        let mut max_energized = 0;

        for y in 0..=grid.max_y {
            let mut beam = Beam::new(0, y, Direction::Right);
            max_energized = max_energized.max(beam.start_travel(&grid).energized.len());
            let mut beam = Beam::new(grid.max_x, y, Direction::Left);
            max_energized = max_energized.max(beam.start_travel(&grid).energized.len());
        }
        for x in 0..=grid.max_x {
            let mut beam = Beam::new(x, 0, Direction::Down);
            max_energized = max_energized.max(beam.start_travel(&grid).energized.len());
            let mut beam = Beam::new(x, grid.max_y, Direction::Up);
            max_energized = max_energized.max(beam.start_travel(&grid).energized.len());
        }
        max_energized
    }
}

struct Grid {
    tiles: HashMap<Position, Tile>,
    max_x: isize,
    max_y: isize,
}

impl Grid {
    pub fn from(input: &str) -> Grid {
        let tiles = input.lines().enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, tile)| (Position::new(x as isize, y as isize), Tile::from(&tile))))
            .collect();
        let max_y = input.lines().count() as isize;
        let max_x = input.lines().next().unwrap().chars().count() as isize;
        Grid { tiles, max_x, max_y }
    }

    pub fn tile_at(&self, position: &Position) -> &Tile {
        self.tiles.get(position).unwrap()
    }

    pub fn out_of_bound(&self, position: &Position) -> bool {
        position.x < 0 || position.x >= self.max_x || position.y < 0 || position.y >= self.max_y
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Ord, PartialOrd)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }
}

#[derive(PartialEq, Debug)]
enum Tile {
    EmptySpace,
    VerticalMirror,
    HorizontalMirror,
    ForwardSlashMirror,
    BackSlashMirror,
}

impl Tile {
    pub fn from(tile: &char) -> Tile {
        match tile {
            '.' => Tile::EmptySpace,
            '|' => Tile::VerticalMirror,
            '-' => Tile::HorizontalMirror,
            '/' => Tile::ForwardSlashMirror,
            '\\' => Tile::BackSlashMirror,
            _ => panic!()
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Beam {
    position: Position,
    direction: Direction,
    energized: Vec<Position>,
}

impl Beam {
    pub fn new(x: isize, y: isize, direction: Direction) -> Beam {
        Beam { position: Position::new(x, y), direction, energized: vec![] }
    }

    pub fn next(&mut self) {
        match self.direction {
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
        }
    }

    pub fn start_travel(&mut self, grid: &Grid) -> &mut Beam {
        let mut seen = HashSet::new();
        self.travel(grid, &mut seen);
        self.energized.sort();
        self.energized.dedup();
        self
    }

    fn travel(&mut self, grid: &Grid, seen: &mut HashSet<(Position, Direction)>) -> &mut Beam {
        if grid.out_of_bound(&self.position) || seen.contains(&(self.position.clone(), self.direction.clone())) {
            return self;
        }

        self.energized.push(self.position.clone());
        seen.insert((self.position.clone(), self.direction.clone()));

        match (grid.tile_at(&self.position), &self.direction) {
            (Tile::VerticalMirror, Direction::Left | Direction::Right) => {
                let mut new_beam = Beam::new(self.position.x, self.position.y, Direction::Up);
                new_beam.next();

                self.direction = Direction::Down;

                self.energized.append(&mut new_beam.travel(grid, seen).energized);
            }
            (Tile::HorizontalMirror, Direction::Down | Direction::Up) => {
                let mut new_beam = Beam::new(self.position.x, self.position.y, Direction::Right);
                new_beam.next();

                self.direction = Direction::Left;

                self.energized.append(&mut new_beam.travel(grid, seen).energized);
            }
            (Tile::BackSlashMirror, Direction::Left) => self.direction = Direction::Up,
            (Tile::BackSlashMirror, Direction::Right) => self.direction = Direction::Down,
            (Tile::BackSlashMirror, Direction::Up) => self.direction = Direction::Left,
            (Tile::BackSlashMirror, Direction::Down) => self.direction = Direction::Right,
            (Tile::ForwardSlashMirror, Direction::Left) => self.direction = Direction::Down,
            (Tile::ForwardSlashMirror, Direction::Right) => self.direction = Direction::Up,
            (Tile::ForwardSlashMirror, Direction::Up) => self.direction = Direction::Right,
            (Tile::ForwardSlashMirror, Direction::Down) => self.direction = Direction::Left,
            _ => (),
        };

        self.next();
        self.travel(grid, seen);
        self
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}