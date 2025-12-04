use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Tile {
    Empty,
    Paper,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '@' => Ok(Self::Paper),
            _ => Err("'.' or '@' expected".to_string()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    const fn all() -> [Self; 8] {
        [
            Self::Up,
            Self::UpRight,
            Self::Right,
            Self::DownRight,
            Self::Down,
            Self::DownLeft,
            Self::Left,
            Self::UpLeft,
        ]
    }
}

fn next_idx<T>(
    matrix: &VecMatrix<T>,
    idx: MatrixIndex,
    direction: Direction,
) -> Option<MatrixIndex> {
    match direction {
        Direction::Up => matrix.next_up(idx),
        Direction::UpRight => matrix.next_up(idx).and_then(|i| matrix.next_right(i)),
        Direction::Right => matrix.next_right(idx),
        Direction::DownRight => matrix.next_down(idx).and_then(|i| matrix.next_right(i)),
        Direction::Down => matrix.next_down(idx),
        Direction::DownLeft => matrix.next_down(idx).and_then(|i| matrix.next_left(i)),
        Direction::Left => matrix.next_left(idx),
        Direction::UpLeft => matrix.next_up(idx).and_then(|i| matrix.next_left(i)),
    }
}

struct TileInfo {
    kind: Tile,
    papers: usize,
    to_remove: bool,
}

pub fn remove_paper(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut tilemap = {
        let mut lines = lines.peekable();

        let width = lines.by_ref().peek().map_or(0, String::len);
        let data = lines
            .flat_map(String::into_bytes)
            .map(|byte| {
                Tile::try_from(byte as char).map(|tile| TileInfo {
                    kind: tile,
                    papers: 0,
                    to_remove: false,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        VecMatrix::with_data(data, width)
    };

    for row in 0..tilemap.height() {
        for col in 0..tilemap.width() {
            let idx = MatrixIndex { row, col };
            tilemap[idx].papers = Direction::all()
                .into_iter()
                .filter_map(|d| next_idx(&tilemap, idx, d))
                .filter(|&next| tilemap[next].kind == Tile::Paper)
                .count();
        }
    }

    let mut remove_stack = tilemap
        .iter_enumerate_mut()
        .filter_map(|(idx, t)| {
            if t.kind == Tile::Paper && t.papers < 4 {
                t.to_remove = true;
                Some(idx)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let accessible_count = remove_stack.len();
    let mut total_removed = 0;

    while let Some(idx) = remove_stack.pop() {
        total_removed += 1;
        for direction in Direction::all() {
            if let Some(next) = next_idx(&tilemap, idx, direction) {
                let tile = &mut tilemap[next];
                tile.papers -= 1;

                if tile.kind == Tile::Paper && tile.papers < 4 && !tile.to_remove {
                    tile.to_remove = true;
                    remove_stack.push(next);
                }
            }
        }
    }

    Ok((accessible_count, total_removed))
}
