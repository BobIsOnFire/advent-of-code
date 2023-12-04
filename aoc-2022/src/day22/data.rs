pub type Coord = aoc_common::util::MatrixIndex;
pub type TileMap = aoc_common::util::VecMatrix<Option<Tile>>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn all() -> [Self; 4] {
        [Self::Right, Self::Down, Self::Left, Self::Up]
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
        }
    }
}

#[derive(Debug)]
pub struct BoundsMapping {
    pub right_side: Vec<(Coord, Direction)>,
    pub left_side: Vec<(Coord, Direction)>,
    pub down_side: Vec<(Coord, Direction)>,
    pub up_side: Vec<(Coord, Direction)>,
}
