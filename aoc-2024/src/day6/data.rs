#[allow(dead_code)] // linter warns on fields, we use them only to print in Debug/Display impl
#[derive(Debug)]
pub struct CharParseError {
    expected: String,
    actual: char,
}

impl std::fmt::Display for CharParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CharParseError {}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = CharParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            v => Err(CharParseError {
                expected: "Direction [^, >, v, <]".into(),
                actual: v,
            }),
        }
    }
}

impl Direction {
    pub const fn clockwise(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Tile {
    Path,
    Obstruction,
    Start(Direction),
}

impl TryFrom<char> for Tile {
    type Error = CharParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Path),
            '#' => Ok(Self::Obstruction),
            v => v.try_into().map(Self::Start),
        }
    }
}
