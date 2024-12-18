use std::collections::{BTreeMap, HashSet};

use aoc_common::util::{self, MatrixIndex, VecMatrix};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    const fn left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
}

fn next_idx(
    tilemap: &VecMatrix<Tile>,
    idx: MatrixIndex,
    direction: Direction,
) -> Option<MatrixIndex> {
    match direction {
        Direction::North => tilemap.next_up(idx),
        Direction::East => tilemap.next_right(idx),
        Direction::South => tilemap.next_down(idx),
        Direction::West => tilemap.next_left(idx),
    }
}

fn find_fastest_path(
    tilemap: &VecMatrix<Tile>,
    start: MatrixIndex,
    end: MatrixIndex,
) -> (usize, usize) {
    let mut visited = VecMatrix::with_data(vec![false; tilemap.len()], tilemap.width());
    let mut lowest_cost = VecMatrix::with_data(vec![usize::MAX; tilemap.len()], tilemap.width());
    let mut entry_from = VecMatrix::with_data(vec![vec![]; tilemap.len()], tilemap.width());

    let mut to_visit: BTreeMap<usize, Vec<(MatrixIndex, Direction)>> = BTreeMap::new();
    to_visit.insert(0, vec![(start, Direction::East)]);

    while let Some((cost, nodes)) = to_visit.pop_first() {
        if lowest_cost[end] < cost {
            break;
        }

        println!("{cost} : {nodes:?}");
        for (idx, direction) in nodes {
            if lowest_cost[idx] > cost {
                lowest_cost[idx] = cost;
            }
            if tilemap[idx] != Tile::Start {
                entry_from[idx].push(next_idx(tilemap, idx, direction.right().right()).unwrap());
            }

            // if visited[idx] {
            //     continue;
            // }
            visited[idx] = true;

            if let Some(next) = next_idx(tilemap, idx, direction) {
                if tilemap[next] != Tile::Wall && !visited[next] {
                    to_visit
                        .entry(cost + 1)
                        .or_default()
                        .push((next, direction));
                }
            }
            if let Some(next) = next_idx(tilemap, idx, direction.right()) {
                if tilemap[next] != Tile::Wall && !visited[next] {
                    to_visit
                        .entry(cost + 1001)
                        .or_default()
                        .push((next, direction.right()));
                }
            }
            if let Some(next) = next_idx(tilemap, idx, direction.left()) {
                if tilemap[next] != Tile::Wall && !visited[next] {
                    to_visit
                        .entry(cost + 1001)
                        .or_default()
                        .push((next, direction.left()));
                }
            }
        }
    }

    let mut best_tiles = HashSet::new();
    let mut to_add = vec![end];

    while let Some(idx) = to_add.pop() {
        best_tiles.insert(idx);
        for &entry in &entry_from[idx] {
            if best_tiles.contains(&entry) {
                continue;
            }
            to_add.push(entry);
        }
    }

    for (idx, &tile) in tilemap.iter_enumerate() {
        if best_tiles.contains(&idx) {
            print!("O");
        } else {
            match tile {
                Tile::Empty => print!("."),
                Tile::Wall => print!("#"),
                Tile::Start => print!("S"),
                Tile::End => print!("E"),
            }
        }

        if idx.col == tilemap.width() - 1 {
            println!();
        }
    }

    (lowest_cost[end], best_tiles.len())
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
        .ok_or("Start missing")?;

    let end = tilemap
        .iter_enumerate()
        .find_map(|(idx, tile)| matches!(tile, Tile::End).then_some(idx))
        .ok_or("End missing")?;

    let (fastest_path, best_tiles_count) = find_fastest_path(&tilemap, start, end);

    Ok((fastest_path, best_tiles_count))
}
