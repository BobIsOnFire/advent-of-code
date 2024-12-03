use std::collections::HashSet;

use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Ground,
    Pipe(Direction, Direction),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Direction::{East, North, South, West};
        use Tile::{Ground, Pipe, Start};
        match value {
            '|' => Pipe(North, South),
            '-' => Pipe(West, East),
            'L' => Pipe(North, East),
            'J' => Pipe(North, West),
            '7' => Pipe(South, West),
            'F' => Pipe(South, East),
            '.' => Ground,
            'S' => Start,
            _ => panic!("Unknown tile: {value}"),
        }
    }
}

impl Tile {
    const fn has_south_pipe(self) -> bool {
        matches!(self, Self::Pipe(Direction::South, _) | Self::Pipe(_, Direction::South))
    }

    const fn has_east_pipe(self) -> bool {
        matches!(self, Self::Pipe(Direction::East, _) | Self::Pipe(_, Direction::East))
    }

    fn next_direction(self, direction: Direction) -> Option<Direction> {
        let direction_out = direction.opposite();

        match self {
            Self::Pipe(first, second) if first == direction_out => Some(second),
            Self::Pipe(first, second) if second == direction_out => Some(first),
            _ => None,
        }
    }
}

fn next_idx(tilemap: &VecMatrix<Tile>, idx: MatrixIndex, direction: Direction) -> Option<MatrixIndex> {
    use Direction::{East, North, South, West};
    match direction {
        North => tilemap.next_up(idx),
        South => tilemap.next_down(idx),
        West => tilemap.next_left(idx),
        East => tilemap.next_right(idx),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Closure {
    Outside,
    Inside,
}

impl Closure {
    const fn opposite(self) -> Self {
        match self {
            Self::Outside => Self::Inside,
            Self::Inside => Self::Outside,
        }
    }
}

pub fn find_enclosing_loop(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut tiles = Vec::new();
    let mut width = 0;

    for line in lines {
        width = line.len();
        tiles.extend(line.chars().map(Tile::from));
    }

    let mut tilemap = VecMatrix::with_data(tiles, width);

    let start_idx = tilemap
        .iter_enumerate()
        .find_map(|(idx, tile)| (*tile == Tile::Start).then_some(idx))
        .expect("Starting tile should exist");

    let mut valid_pipes = [Direction::North, Direction::South, Direction::West, Direction::East]
        .into_iter()
        .filter_map(|d| next_idx(&tilemap, start_idx, d).map(|idx| (d, idx)))
        .filter_map(|(d, idx)| tilemap[idx].next_direction(d).map(|_| d));

    let pipe1 = valid_pipes.next().expect("There should be two valid pipes coming out of starting tile");
    let pipe2 = valid_pipes.next().expect("There should be two valid pipes coming out of starting tile");

    tilemap[start_idx] = Tile::Pipe(pipe1, pipe2);

    let loop_tiles = std::iter::successors(Some((start_idx, pipe1.opposite())), |(idx, direction)| {
        let direction = tilemap[*idx].next_direction(*direction).expect("Main loop cannot be broken");
        let idx = next_idx(&tilemap, *idx, direction).expect("Main loop cannot go out of bounds");
        if idx == start_idx {
            None
        } else {
            Some((idx, direction))
        }
    })
    .map(|(idx, _)| idx)
    .collect::<HashSet<_>>();

    for (idx, tile) in tilemap.iter_enumerate_mut() {
        if !loop_tiles.contains(&idx) {
            *tile = Tile::Ground;
        }
    }

    let mut enclosed_map: VecMatrix<Closure> = VecMatrix::with_data(vec![Closure::Outside; tilemap.len()], tilemap.width());
    let mut enclosed_ground = 0;

    for (idx, tile) in tilemap.iter_enumerate() {
        let left = enclosed_map.next_left(idx).map_or(Closure::Outside, |i| enclosed_map[i]);
        let top = enclosed_map.next_up(idx).map_or(Closure::Outside, |i| enclosed_map[i]);
        if tile.has_south_pipe() {
            enclosed_map[idx] = left.opposite();
        } else if tile.has_east_pipe() {
            enclosed_map[idx] = top.opposite();
        } else {
            assert!(left == top);
            enclosed_map[idx] = top;
        }

        if left == Closure::Inside && top == Closure::Inside && *tile == Tile::Ground {
            enclosed_ground += 1;
        }
    }

    Ok((loop_tiles.len() / 2, enclosed_ground))
}
