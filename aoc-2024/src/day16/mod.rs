use std::collections::{BTreeMap, HashMap, HashSet};

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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    idx: MatrixIndex,
    direction: Direction,
}

impl Point {
    fn try_forward(&self, tilemap: &VecMatrix<Tile>) -> Option<Self> {
        let next_idx = match self.direction {
            Direction::North => tilemap.next_up(self.idx),
            Direction::East => tilemap.next_right(self.idx),
            Direction::South => tilemap.next_down(self.idx),
            Direction::West => tilemap.next_left(self.idx),
        };

        next_idx.map(|idx| Self { idx, direction: self.direction })
    }

    const fn left(&self) -> Self {
        let next_direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };

        Self {
            idx: self.idx,
            direction: next_direction,
        }
    }

    const fn right(&self) -> Self {
        let next_direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        Self {
            idx: self.idx,
            direction: next_direction,
        }
    }
}

fn find_fastest_path(
    tilemap: &VecMatrix<Tile>,
    start: MatrixIndex,
    end: MatrixIndex,
) -> (usize, usize) {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut lowest_cost: HashMap<MatrixIndex, usize> = HashMap::new();
    let mut entry_from: HashMap<Point, Vec<Point>> = HashMap::new();

    let start = Point {
        idx: start,
        direction: Direction::East,
    };

    let mut to_visit: BTreeMap<usize, Vec<(Point, Point)>> = BTreeMap::new();
    to_visit.insert(0, vec![(start.clone(), start)]);

    while let Some((cost, nodes)) = to_visit.pop_first() {
        if lowest_cost.get(&end).is_some_and(|&v| cost > v) {
            break;
        }

        for (from, to) in &nodes {
            if visited.contains(to) {
                continue;
            }

            lowest_cost.entry(to.idx).or_insert(cost);
            entry_from.entry(to.clone()).or_default().push(from.clone());

            if let Some(next) = to.try_forward(tilemap) {
                if tilemap[next.idx] != Tile::Wall && !visited.contains(&next) {
                    to_visit
                        .entry(cost + 1)
                        .or_default()
                        .push((to.clone(), next));
                }
            }
            let right = to.right();
            if !visited.contains(&right) {
                to_visit
                    .entry(cost + 1000)
                    .or_default()
                    .push((to.clone(), right));
            }
            let left = to.left();
            if !visited.contains(&left) {
                to_visit
                    .entry(cost + 1000)
                    .or_default()
                    .push((to.clone(), left));
            }
        }

        for (_, to) in nodes {
            visited.insert(to);
        }
    }

    let mut best_tiles = HashSet::new();
    let mut visited = HashSet::new();
    let mut to_add = vec![
        Point {
            idx: end,
            direction: Direction::North,
        },
        Point { idx: end, direction: Direction::East },
        Point {
            idx: end,
            direction: Direction::South,
        },
        Point { idx: end, direction: Direction::West },
    ];

    while let Some(point) = to_add.pop() {
        for entry in entry_from.get(&point).unwrap_or(&vec![]) {
            if visited.contains(entry) {
                continue;
            }
            to_add.push(entry.clone());
        }
        best_tiles.insert(point.idx);
        visited.insert(point);
    }

    (*lowest_cost.get(&end).unwrap(), best_tiles.len())
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
