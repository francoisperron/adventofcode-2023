use std::collections::HashSet;
use crate::day16::direction::Direction;
use crate::day16::grid::Grid;
use crate::day16::position::Position;
use crate::day16::tile::Tile;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Beam {
    pub position: Position,
    pub direction: Direction,
    pub energized: Vec<Position>,
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

#[cfg(test)]
mod tests {
    use crate::day16::beam::Beam;
    use crate::day16::direction::Direction;
    use crate::day16::grid::Grid;
    use crate::day16::position::Position;

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
}