use crate::day23::tile::Tile;
use crate::day23::tile::Direction::{Down, Left, Right, Up};

pub struct Trails {
    tiles: Vec<Vec<Tile>>,
}

impl Trails {
    pub fn from(input: &str) -> Trails {
        Trails { tiles: input.lines().map(|l| l.chars().map(Tile::from).collect()).collect() }
    }

    pub fn from_no_slopes(input: &str) -> Trails {
        Trails { tiles: input.lines().map(|l| l.chars().map(Tile::from_no_slopes).collect()).collect() }
    }

    pub fn longest_hike(&mut self) -> usize {
        self.tiles[0][1] = Tile::Walked;
        let mut hikes = Vec::new();

        self.hike_recursive(0, 1, 0, &mut hikes);
        hikes.into_iter().max().unwrap()
    }

    fn hike_recursive(&mut self, x: usize, y: usize, steps: usize, hikes: &mut Vec<usize>) {
        if y == self.tiles.len() - 1 {
            hikes.push(steps);
            return;
        }

        if y > 0 && self.tiles[y - 1][x].is_clear(Up) {
            self.hike_path(x, y - 1, steps, hikes);
        }

        if y < self.tiles.len() && self.tiles[y + 1][x].is_clear(Down) {
            self.hike_path(x, y + 1, steps, hikes);
        }

        if x > 0 && self.tiles[y][x - 1].is_clear(Left) {
            self.hike_path(x - 1, y, steps, hikes);
        }

        if x < self.tiles[0].len() && self.tiles[y][x + 1].is_clear(Right) {
            self.hike_path(x + 1, y, steps, hikes);
        }
    }

    fn hike_path(&mut self, x: usize, y: usize, steps: usize, hikes: &mut Vec<usize>) {
        let current_tile = self.tiles[y][x];
        self.tiles[y][x] = Tile::Walked;
        self.hike_recursive(x, y, steps + 1, hikes);
        self.tiles[y][x] = current_tile;
    }
}

#[cfg(test)]
mod tests {
    use crate::day23::tile::Tile::{Forest, Path, Slope};
    use crate::day23::tile::Direction::{Down, Left, Right, Up};
    use super::*;

    #[test]
    fn parses_trails() {
        let trails = Trails::from("#.##\n<>v^");

        let expected = vec![
            vec![Forest, Path, Forest, Forest],
            vec![Slope(Left), Slope(Right), Slope(Down), Slope(Up)],
        ];
        assert_eq!(trails.tiles, expected)
    }
}