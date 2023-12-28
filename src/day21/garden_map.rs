use std::collections::HashSet;
use crate::day21::position::Position;
use crate::day21::tile::Tile;
use crate::day21::tile::Tile::Garden;

pub struct GardenMap {
    tiles: Vec<Vec<Tile>>,
    size: usize,
    pub starting_position: Position,
    positions: HashSet<Position>,
}

impl GardenMap {
    pub fn from(input: &str) -> GardenMap {
        let tiles: Vec<Vec<Tile>> = input.lines()
            .map(|l| l.chars().map(Tile::from).collect())
            .collect();

        let size = tiles.len();

        let starting_position = input.lines().enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().filter(|(_, t)| *t == 'S').map(move |(x, _)| Position::new(x as isize, y as isize)))
            .next().unwrap();

        GardenMap { tiles, size, starting_position, positions: HashSet::from([starting_position]) }
    }

    pub fn gardens_visited(&mut self, steps: usize) -> usize {
        self.gardens_visited_from(self.starting_position, steps)
    }

    pub fn gardens_visited_from(&mut self, position: Position, steps: usize) -> usize {
        self.positions = HashSet::from([position]);
        for _ in 0..steps {
            self.walk()
        }
        self.positions.len()
    }

    // algo from https://www.youtube.com/watch?v=9UOMZSL0JTg
    // Assumption 1: start in the middle
    // Assumption 2: start line and column are all gardens
    // Assumption 3: edges lines and column are all gardens
    // Assumption 4: grid is of odd size
    // Assumption 5: grid is square
    pub fn infinite_gardens_visited(&mut self, steps: usize) -> usize {
        let infinite_grid_size = steps / self.size - 1;
        let farthest = (self.size - 1) as isize;

        let number_of_odd_tiles = (infinite_grid_size / 2 * 2 + 1).pow(2);
        let gardens_in_odd_tiles = self.gardens_visited_from(self.starting_position, self.size * 2 + 1);
        let number_of_even_tiles = ((infinite_grid_size + 1) / 2 * 2).pow(2);
        let gardens_in_even_tiles = self.gardens_visited_from(self.starting_position, self.size * 2);
        let gardens_in_fully_walked_tiles = number_of_odd_tiles * gardens_in_odd_tiles + number_of_even_tiles * gardens_in_even_tiles;

        let gardens_in_north_tiles = self.gardens_visited_from(Position::new(farthest, self.starting_position.y), self.size - 1);
        let gardens_in_south_tiles = self.gardens_visited_from(Position::new(0, self.starting_position.y), self.size - 1);
        let gardens_in_east_tiles = self.gardens_visited_from(Position::new(self.starting_position.x, 0), self.size - 1);
        let gardens_in_west_tiles = self.gardens_visited_from(Position::new(self.starting_position.x, farthest), self.size - 1);
        let gardens_in_corner_tiles = gardens_in_north_tiles + gardens_in_east_tiles + gardens_in_south_tiles + gardens_in_west_tiles;

        let small_steps = self.size / 2 - 1;
        let big_steps = self.size * 3 / 2 - 1;
        let gardens_in_north_east_small = self.gardens_visited_from(Position::new(farthest, 0), small_steps);
        let gardens_in_north_east_big = self.gardens_visited_from(Position::new(farthest, 0), big_steps);
        let gardens_in_south_east_small = self.gardens_visited_from(Position::new(0, 0), small_steps);
        let gardens_in_south_east_big = self.gardens_visited_from(Position::new(0, 0), big_steps);
        let gardens_in_south_west_small = self.gardens_visited_from(Position::new(0, farthest), small_steps);
        let gardens_in_south_west_big = self.gardens_visited_from(Position::new(0, farthest), big_steps);
        let gardens_in_north_west_small = self.gardens_visited_from(Position::new(farthest, farthest), small_steps);
        let gardens_in_north_west_big = self.gardens_visited_from(Position::new(farthest, farthest), big_steps);
        let gardens_in_diagonals =
            ((infinite_grid_size + 1) * (gardens_in_north_east_small + gardens_in_south_east_small + gardens_in_south_west_small + gardens_in_north_west_small)) +
            (infinite_grid_size * (gardens_in_north_east_big + gardens_in_south_east_big + gardens_in_south_west_big + gardens_in_north_west_big));

        gardens_in_fully_walked_tiles + gardens_in_corner_tiles + gardens_in_diagonals
    }

    fn walk(&mut self) {
        let mut new_positions: HashSet<Position> = HashSet::new();
        for p in self.positions.clone() {
            for adjacent in p.around() {
                if self.is_a_garden(&adjacent) {
                    new_positions.insert(adjacent);
                }
            }
        }
        self.positions = new_positions;
    }

    fn is_a_garden(&self, p: &Position) -> bool {
        p.x >= 0 && p.x < self.size as isize && p.y >= 0 && p.y < self.size as isize && self.tiles[p.x as usize][p.y as usize] == Garden
    }
}

#[cfg(test)]
mod tests {
    use Tile::{Garden, Rock};
    use super::*;

    #[test]
    fn parses_map() {
        let map = GardenMap::from(".#\nS.");

        assert_eq!(map.tiles, vec![
            vec![Garden, Rock],
            vec![Garden, Garden],
        ]);
        assert_eq!(map.starting_position, Position::new(0, 1));
    }

    #[test]
    fn walks_one_step_in_all_directions() {
        let mut map = GardenMap::from("...\n.S.\n...");

        map.walk();

        let expected = HashSet::from([
            Position::new(0, 1),
            Position::new(1, 2),
            Position::new(1, 0),
            Position::new(2, 1)
        ]);
        assert_eq!(map.positions, expected);
    }

    #[test]
    fn walks_one_step_in_no_directions() {
        let mut map = GardenMap::from("###\n#S#\n###");

        map.walk();

        assert_eq!(map.positions, HashSet::new());
    }

    #[test]
    fn walks_two_steps() {
        let mut map = GardenMap::from("...\n.S.\n...");

        map.walk();
        map.walk();

        let expected = HashSet::from([
            Position::new(2, 2),
            Position::new(0, 2),
            Position::new(0, 0),
            Position::new(1, 1),
            Position::new(2, 0)
        ]);
        assert_eq!(map.positions, expected);
    }
}