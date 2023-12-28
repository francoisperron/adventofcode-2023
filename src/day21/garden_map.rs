use std::collections::HashSet;
use crate::day21::position::Position;
use crate::day21::tile::Tile;
use crate::day21::tile::Tile::Garden;

pub struct GardenMap {
    tiles: Vec<Vec<Tile>>,
    size: isize,
    positions: HashSet<Position>,
}

impl GardenMap {
    pub fn from(input: &str) -> GardenMap {
        let tiles: Vec<Vec<Tile>> = input.lines()
            .map(|l| l.chars().map(Tile::from).collect())
            .collect();

        let size = tiles.len() as isize;

        let starting_position = input.lines().enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().filter(|(_, t)| *t == 'S').map(move |(x, _)| Position::new(x as isize, y as isize)))
            .next().unwrap();

        GardenMap { tiles, size, positions: HashSet::from([starting_position]) }
    }

    pub fn gardens_visited(&mut self, steps: usize) -> usize {
        for _ in 0..steps {
            self.walk()
        }
        self.positions.len()
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
        p.x >= 0 && p.x <= self.size && p.y >= 0 && p.y <= self.size && self.tiles[p.x as usize][p.y as usize] == Garden
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
        assert_eq!(map.positions, HashSet::from([Position::new(0, 1)]));
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