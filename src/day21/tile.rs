#[derive(Debug, PartialEq)]
pub enum Tile {
    Garden,
    Rock,
}

impl Tile {
    pub fn from(tile: char) -> Tile {
        match tile {
            '.' | 'S' => Tile::Garden,
            '#' => Tile::Rock,
            _ => panic!()
        }
    }
}