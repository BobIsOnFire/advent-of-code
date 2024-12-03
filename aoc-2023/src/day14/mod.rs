use std::collections::HashMap;

use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Square,
    Circle,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Square,
            'O' => Self::Circle,
            _ => panic!("Unknown tile: {value}"),
        }
    }
}

fn tilt_north(tilemap: &mut VecMatrix<Tile>) {
    for col in 0..tilemap.width() {
        let mut first_row = 0;
        let mut circle_count = 0;

        for row in 0..tilemap.height() {
            let idx = MatrixIndex { col, row };
            match tilemap[idx] {
                Tile::Empty => {}
                Tile::Square => {
                    for row in first_row..(first_row + circle_count) {
                        tilemap[MatrixIndex { col, row }] = Tile::Circle;
                    }

                    first_row = row + 1;
                    circle_count = 0;
                }
                Tile::Circle => {
                    circle_count += 1;
                    tilemap[idx] = Tile::Empty;
                }
            }
        }

        for row in first_row..(first_row + circle_count) {
            tilemap[MatrixIndex { col, row }] = Tile::Circle;
        }
    }
}

fn tilt_south(tilemap: &mut VecMatrix<Tile>) {
    for col in 0..tilemap.width() {
        let mut last_row = tilemap.height() - 1;
        let mut circle_count = 0;

        for row in (0..tilemap.height()).rev() {
            let idx = MatrixIndex { col, row };
            match tilemap[idx] {
                Tile::Empty => {}
                Tile::Square => {
                    for row in (last_row - circle_count + 1)..=last_row {
                        tilemap[MatrixIndex { col, row }] = Tile::Circle;
                    }

                    last_row = row.saturating_sub(1);
                    circle_count = 0;
                }
                Tile::Circle => {
                    circle_count += 1;
                    tilemap[idx] = Tile::Empty;
                }
            }
        }

        for row in (last_row - circle_count + 1)..=last_row {
            tilemap[MatrixIndex { col, row }] = Tile::Circle;
        }
    }
}

fn tilt_west(tilemap: &mut VecMatrix<Tile>) {
    for row in 0..tilemap.height() {
        let mut first_col = 0;
        let mut circle_count = 0;

        for col in 0..tilemap.width() {
            let idx = MatrixIndex { col, row };
            match tilemap[idx] {
                Tile::Empty => {}
                Tile::Square => {
                    for col in first_col..(first_col + circle_count) {
                        tilemap[MatrixIndex { col, row }] = Tile::Circle;
                    }

                    first_col = col + 1;
                    circle_count = 0;
                }
                Tile::Circle => {
                    circle_count += 1;
                    tilemap[idx] = Tile::Empty;
                }
            }
        }

        for col in first_col..(first_col + circle_count) {
            tilemap[MatrixIndex { col, row }] = Tile::Circle;
        }
    }
}

fn tilt_east(tilemap: &mut VecMatrix<Tile>) {
    for row in 0..tilemap.height() {
        let mut last_col = tilemap.width() - 1;
        let mut circle_count = 0;

        for col in (0..tilemap.width()).rev() {
            let idx = MatrixIndex { col, row };
            match tilemap[idx] {
                Tile::Empty => {}
                Tile::Square => {
                    for col in (last_col - circle_count + 1)..=last_col {
                        tilemap[MatrixIndex { col, row }] = Tile::Circle;
                    }

                    last_col = col.saturating_sub(1);
                    circle_count = 0;
                }
                Tile::Circle => {
                    circle_count += 1;
                    tilemap[idx] = Tile::Empty;
                }
            }
        }

        for col in (last_col - circle_count + 1)..=last_col {
            tilemap[MatrixIndex { col, row }] = Tile::Circle;
        }
    }
}

#[allow(dead_code)]
fn draw(tilemap: &VecMatrix<Tile>) {
    for (idx, tile) in tilemap.iter_enumerate() {
        match tile {
            Tile::Empty => print!("."),
            Tile::Square => print!("#"),
            Tile::Circle => print!("O"),
        }
        if idx.col == tilemap.width() - 1 {
            println!();
        }
    }
    println!();
}

fn get_total_load(tilemap: &VecMatrix<Tile>) -> usize {
    let mut total_load = 0;

    for (idx, tile) in tilemap.iter_enumerate() {
        if *tile == Tile::Circle {
            total_load += tilemap.height() - idx.row;
        }
    }

    total_load
}

fn compress(tilemap: &VecMatrix<Tile>) -> Vec<u64> {
    let mut numbers = Vec::new();

    for chunk in tilemap.data().chunks(u64::BITS as usize) {
        let num = chunk.iter().fold(0, |acc, tile| (acc << 1) | u64::from(matches!(tile, Tile::Circle)));
        numbers.push(num);
    }

    numbers
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut data = Vec::new();
    let mut width = 0;

    for line in lines {
        width = line.len();
        data.extend(line.chars().map(Tile::from));
    }

    let mut tilemap = VecMatrix::with_data(data, width);

    let mut i: usize = 0;

    let mut compressed_to_idx = HashMap::new();
    let mut idx_to_load = HashMap::new();

    let loop_idx = loop {
        tilt_north(&mut tilemap);
        tilt_west(&mut tilemap);
        tilt_south(&mut tilemap);
        tilt_east(&mut tilemap);

        i += 1;

        let current = compress(&tilemap);
        if let Some(idx) = compressed_to_idx.get(&current) {
            break *idx;
        }

        idx_to_load.insert(i, get_total_load(&tilemap));
        compressed_to_idx.insert(current, i);
    };

    let loop_size = i - loop_idx;
    let billionth_idx = (1_000_000_000 - loop_idx) % loop_size + loop_idx;

    let cycle_total_load = *idx_to_load.get(&billionth_idx).unwrap();

    Ok((0, cycle_total_load))
}
