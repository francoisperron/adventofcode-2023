#[derive(PartialEq, Debug)]
pub enum Tile {
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