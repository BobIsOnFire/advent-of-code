mod cube;
mod data;

use aoc_common::util::{self, VecMatrix};

use self::{
    cube::cube_mapping,
    data::{BoundsMapping, Coord, Direction, Tile, TileMap},
};

fn flat_mapping(tilemap: &TileMap) -> BoundsMapping {
    let mut bounds = BoundsMapping {
        right_side: vec![],
        left_side: vec![],
        down_side: vec![],
        up_side: vec![],
    };

    for row in 0..tilemap.height() {
        let mut tile_coords_iter = (0..tilemap.width())
            .map(|col| Coord { row, col })
            .filter(|coord| tilemap[*coord].is_some());

        let first = tile_coords_iter.next().unwrap_or(Coord { row, col: 0 });
        let last = tile_coords_iter.last().unwrap_or(first);

        bounds.right_side.push((first, Direction::Right));
        bounds.left_side.push((last, Direction::Left));
    }

    for col in 0..tilemap.width() {
        let mut tile_coords_iter = (0..tilemap.height())
            .map(|row| Coord { row, col })
            .filter(|coord| tilemap[*coord].is_some());

        let first = tile_coords_iter.next().unwrap_or(Coord { row: 0, col });
        let last = tile_coords_iter.last().unwrap_or(first);

        bounds.down_side.push((first, Direction::Down));
        bounds.up_side.push((last, Direction::Up));
    }

    bounds
}

struct Board {
    tilemap: VecMatrix<Option<Tile>>,
    bounds: BoundsMapping,
    coord: Coord,
    direction: Direction,
}

impl Board {
    fn new(tilemap: VecMatrix<Option<Tile>>, bounds: BoundsMapping) -> Self {
        let mut coord = Coord { row: 1, col: 1 };
        while tilemap[coord].is_none() {
            coord.col += 1;
        }

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
            let mut next_direction = self.direction;

            if self.tilemap[next_coord].is_none() {
                (next_coord, next_direction) = match self.direction {
                    Direction::Right => self.bounds.right_side[next_coord.row],
                    Direction::Down => self.bounds.down_side[next_coord.col],
                    Direction::Left => self.bounds.left_side[next_coord.row],
                    Direction::Up => self.bounds.up_side[next_coord.col],
                };
            }

            match self.tilemap[next_coord].expect("Bounds should be filled correctly") {
                Tile::Empty => {
                    self.coord = next_coord;
                    self.direction = next_direction;
                }
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

    let move_data = lines.next().expect("Move data should exist");

    let mappings = [flat_mapping(&tilemap), cube_mapping(&tilemap)];
    let mut passwords = [0, 0];

    for (mapping, password) in mappings.into_iter().zip(passwords.iter_mut()) {
        let mut board = Board::new(tilemap.clone(), mapping);

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

        *password = board.coord.row * 1000 + board.coord.col * 4 + board.direction as usize;
    }

    Ok((passwords[0], passwords[1]))
}
