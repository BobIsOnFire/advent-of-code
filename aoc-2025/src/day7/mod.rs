use std::collections::HashMap;

use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Start,
    Empty,
    Splitter,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Empty),
            '^' => Ok(Self::Splitter),
            _ => Err("'S', '.' or '^' expected".to_string()),
        }
    }
}

pub fn count_beams(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let tilemap = {
        let mut lines = lines.peekable();

        let width = lines.by_ref().peek().map_or(0, String::len);
        let data = lines
            .flat_map(String::into_bytes)
            .map(|b| Tile::try_from(b as char))
            .collect::<Result<Vec<_>, _>>()?;

        VecMatrix::with_data(data, width)
    };

    let start = tilemap
        .iter_enumerate()
        .find_map(|(idx, &tile)| (tile == Tile::Start).then_some(idx))
        .ok_or("Start tile not found")?;

    let mut beam_paths = HashMap::new();
    beam_paths.insert(start.col, 1);

    let mut splits = 0;

    for row in (start.row + 1)..tilemap.height() {
        let mut beam_paths_next = HashMap::new();
        for (col, count) in beam_paths.into_iter() {
            let idx = MatrixIndex { row, col };
            if tilemap[idx] == Tile::Splitter {
                splits += 1;
                if col > 0 {
                    *beam_paths_next.entry(col - 1).or_insert(0) += count;
                }
                if col < tilemap.width() - 1 {
                    *beam_paths_next.entry(col + 1).or_insert(0) += count;
                }
            } else {
                *beam_paths_next.entry(col).or_insert(0) += count;
            }
        }
        beam_paths = beam_paths_next;
    }

    let total_paths = beam_paths.into_values().sum();

    Ok((splits, total_paths))
}
