use std::collections::HashMap;
use crate::day16::position::Position;
use crate::day16::tile::Tile;

pub struct Grid {
    pub tiles: HashMap<Position, Tile>,
    pub max_x: isize,
    pub max_y: isize,
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::day16::grid::Grid;
    use crate::day16::position::Position;
    use crate::day16::tile::Tile;

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
}