use aoc_common::util::{self, MatrixIndex, VecMatrix};

type Coord = MatrixIndex;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct BoardBounds {
    first_left: Vec<Coord>,
    last_right: Vec<Coord>,
    first_up: Vec<Coord>,
    last_down: Vec<Coord>,
}

struct Board {
    tilemap: VecMatrix<Option<Tile>>,
    bounds: BoardBounds,
    coord: Coord,
    direction: Direction,
}

impl BoardBounds {
    fn from_tilemap(tilemap: &VecMatrix<Option<Tile>>) -> Self {
        let mut bounds = Self {
            first_left: vec![],
            last_right: vec![],
            first_up: vec![],
            last_down: vec![],
        };

        for row in 0..tilemap.height() {
            let mut tile_coords_iter = (0..tilemap.width())
                .map(|col| Coord { row, col })
                .filter(|coord| tilemap[*coord].is_some());

            let first = tile_coords_iter.next().unwrap_or(Coord { row, col: 0 });
            let last = tile_coords_iter.last().unwrap_or(first);

            bounds.first_left.push(first);
            bounds.last_right.push(last);
        }

        for col in 0..tilemap.width() {
            let mut tile_coords_iter = (0..tilemap.height())
                .map(|row| Coord { row, col })
                .filter(|coord| tilemap[*coord].is_some());

            let first = tile_coords_iter.next().unwrap_or(Coord { row: 0, col });
            let last = tile_coords_iter.last().unwrap_or(first);

            bounds.first_up.push(first);
            bounds.last_down.push(last);
        }

        bounds
    }
}

impl Board {
    fn from_tilemap(tilemap: VecMatrix<Option<Tile>>) -> Self {
        let bounds = BoardBounds::from_tilemap(&tilemap);
        let coord = bounds.first_left[1];
        Self {
            tilemap,
            bounds,
            coord,
            direction: Direction::Right,
        }
    }

    fn move_next(&mut self, count: usize) {
        for _ in 0..count {
            let mut next_coord = {
                let Coord { row, col } = self.coord;
                match self.direction {
                    Direction::Right => Coord { row, col: col + 1 },
                    Direction::Down => Coord { row: row + 1, col },
                    Direction::Left => Coord { row, col: col - 1 },
                    Direction::Up => Coord { row: row - 1, col },
                }
            };

            if self.tilemap[next_coord].is_none() {
                next_coord = match self.direction {
                    Direction::Right => self.bounds.first_left[next_coord.row],
                    Direction::Down => self.bounds.first_up[next_coord.col],
                    Direction::Left => self.bounds.last_right[next_coord.row],
                    Direction::Up => self.bounds.last_down[next_coord.col],
                };
            }

            match self.tilemap[next_coord].expect("Bounds should be filled correctly") {
                Tile::Empty => self.coord = next_coord,
                Tile::Wall => return,
            }
        }
    }

    fn turn_clockwise(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        };
    }

    fn turn_counter_clockwise(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        };
    }
}

fn tile_from_char(ch: char) -> Option<Tile> {
    match ch {
        '.' => Some(Tile::Empty),
        '#' => Some(Tile::Wall),
        _ => None,
    }
}

pub fn get_answer(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut tilemap = VecMatrix::new(200);

    // First row
    tilemap.extend((0..200).map(|_| None));

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let len = line.len();

        assert!(tilemap.width() >= len + 2);

        tilemap.push(None);
        tilemap.extend(line.chars().map(tile_from_char));
        tilemap.finish_row_with(|| None);
    }

    // Last row
    tilemap.extend((0..200).map(|_| None));

    let mut board = Board::from_tilemap(tilemap);

    let move_data = lines.next().expect("Move data should exist");
    let mut lexer = util::Lexer::of(&move_data);
    while lexer.end().is_err() {
        if let Ok(count) = lexer.unsigned_number() {
            board.move_next(count)
        } else {
            match lexer.symbol()? {
                'R' => board.turn_clockwise(),
                'L' => board.turn_counter_clockwise(),
                ch => panic!("Unknown turn symbol: {}", ch),
            }
        }
    }

    let password = board.coord.row * 1000 + board.coord.col * 4 + board.direction as usize;

    Ok((password, 0))
}
