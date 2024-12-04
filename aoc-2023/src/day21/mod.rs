use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Plot,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '.' => Self::Plot,
            '#' => Self::Rock,
            _ => panic!("Unknown tile char {value}"),
        }
    }
}

fn count_tiles(tilemap: &VecMatrix<Tile>, start: MatrixIndex, steps: usize, odd: bool) -> usize {
    let mut visited = VecMatrix::with_data(vec![false; tilemap.len()], tilemap.width());
    visited[start] = true;

    let mut front = vec![start];

    // if-else looks much cleaner here IMO
    #[allow(clippy::bool_to_int_with_if)]
    let mut tile_count = if odd { 0 } else { 1 };

    for step in 1..=steps {
        for idx in std::mem::take(&mut front) {
            let neighbours = [
                tilemap.next_up(idx),
                tilemap.next_left(idx),
                tilemap.next_down(idx),
                tilemap.next_right(idx),
            ];

            for neighbour in neighbours.into_iter().flatten() {
                if tilemap[neighbour] != Tile::Rock && !visited[neighbour] {
                    visited[neighbour] = true;
                    front.push(neighbour);
                }
            }
        }

        if (odd && step % 2 == 1) || (!odd && step % 2 == 0) {
            tile_count += front.len();
        }
    }

    tile_count
}

pub fn count_garden_steps(
    lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let tilemap = {
        let mut data = vec![];
        let mut width = 0;
        for line in lines {
            width = line.len();
            data.extend(line.chars().map(Tile::from));
        }

        VecMatrix::with_data(data, width)
    };

    let start_idx = tilemap
        .iter_enumerate()
        .find_map(|(idx, tile)| (*tile == Tile::Start).then_some(idx))
        .expect("Start tile should exist");

    println!(
        "- Full odd:   {}",
        count_tiles(&tilemap, start_idx, 290, true)
    );
    println!(
        "- Full even:  {}",
        count_tiles(&tilemap, start_idx, 290, false)
    );

    let corners = [
        MatrixIndex { row: 0, col: 0 },
        MatrixIndex { row: 0, col: tilemap.width() - 1 },
        MatrixIndex { row: tilemap.height() - 1, col: 0 },
        MatrixIndex {
            row: tilemap.height() - 1,
            col: tilemap.width() - 1,
        },
    ];

    for corner in corners {
        println!("Corner [{}, {}]:", corner.row, corner.col);
        // println!("- Small odd:  {}", count_tiles(&tilemap, corner, 64, true));
        println!("- Small even: {}", count_tiles(&tilemap, corner, 64, false));
        println!("- Big odd:    {}", count_tiles(&tilemap, corner, 195, true));
        // println!("- Big even:   {}", count_tiles(&tilemap, corner, 195, false));
    }

    let sides = [
        MatrixIndex { row: 0, col: tilemap.width() / 2 },
        MatrixIndex {
            row: tilemap.height() - 1,
            col: tilemap.width() / 2,
        },
        MatrixIndex { row: tilemap.height() / 2, col: 0 },
        MatrixIndex {
            row: tilemap.height() / 2,
            col: tilemap.width() - 1,
        },
    ];

    for side in sides {
        println!("Side [{}, {}]:", side.row, side.col);
        // println!("- Big odd:    {}", count_tiles(&tilemap, side, 195, true));
        println!("- Big even:   {}", count_tiles(&tilemap, side, 130, false));
    }

    Ok((count_tiles(&tilemap, start_idx, 64, false), 0))
}
