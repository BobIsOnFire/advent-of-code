use aoc_common::util::{self, VecMatrix};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}
impl TryFrom<char> for Tile {
    type Error = CharParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            ch => Err(CharParseError {
                expected: "Tile ('.', '#', 'S' or 'E')".to_string(),
                actual: ch,
            }),
        }
    }
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let tilemap: VecMatrix<Tile> = {
        let mut lines = lines.peekable();

        let width = lines.by_ref().peek().map_or(0, String::len);
        let data = lines
            .flat_map(String::into_bytes)
            .map(|b| b as char)
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;

        VecMatrix::with_data(data, width)
    };

    let start = tilemap
        .iter_enumerate()
        .find_map(|(idx, tile)| matches!(tile, Tile::Start).then_some(idx))
        .ok_or("No start")?;

    let mut path = vec![];

    let mut from = start;
    let mut current = start;
    loop {
        path.push(current);

        if tilemap[current] == Tile::End {
            break;
        }

        for next in std::iter::empty()
            .chain(tilemap.next_up(current))
            .chain(tilemap.next_down(current))
            .chain(tilemap.next_left(current))
            .chain(tilemap.next_right(current))
        {
            if next == from || tilemap[next] == Tile::Wall {
                continue;
            }

            from = current;
            current = next;
            break;
        }
    }

    let mut count_short = 0;
    let mut count_long = 0;

    for (i, from) in path.iter().enumerate() {
        for (j, to) in path.iter().enumerate().skip(i + 1) {
            let track_distance = j - i;
            let cheat_distance =
                usize::abs_diff(from.row, to.row) + usize::abs_diff(from.col, to.col);
            if track_distance - cheat_distance < 100 {
                continue;
            }

            if cheat_distance <= 2 {
                count_short += 1;
            }
            if cheat_distance <= 20 {
                count_long += 1;
            }
        }
    }

    Ok((count_short, count_long))
}
