use std::collections::{HashMap, HashSet};

use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Node(char),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            ch => Self::Node(ch),
        }
    }
}

fn into_matrix_index(tilemap: &VecMatrix<Tile>, row: i64, col: i64) -> Option<MatrixIndex> {
    let (width, height) = (tilemap.width() as i64, tilemap.height() as i64);

    if (0..height).contains(&row) && (0..width).contains(&col) {
        Some(MatrixIndex { row: row as usize, col: col as usize })
    } else {
        None
    }
}

fn add_antinodes(
    tilemap: &VecMatrix<Tile>,
    nodes: (MatrixIndex, MatrixIndex),
    remote_range: &(impl IntoIterator<Item = i64> + Clone),
    set: &mut HashSet<MatrixIndex>,
) {
    let (left, right) = nodes;

    let left_row = left.row as i64;
    let right_row = right.row as i64;
    let left_col = left.col as i64;
    let right_col = right.col as i64;

    let row_diff = right_row - left_row;
    let col_diff = right_col - left_col;

    for i in remote_range.clone() {
        if let Some(left) =
            into_matrix_index(tilemap, left_row - i * row_diff, left_col - i * col_diff)
        {
            set.insert(left);
        } else {
            break;
        }
    }

    for i in remote_range.clone() {
        if let Some(right) =
            into_matrix_index(tilemap, right_row + i * row_diff, right_col + i * col_diff)
        {
            set.insert(right);
        } else {
            break;
        }
    }
}

fn add_all_antinodes(
    tilemap: &VecMatrix<Tile>,
    indices: &[MatrixIndex],
    remote_range: &(impl IntoIterator<Item = i64> + Clone),
    set: &mut HashSet<MatrixIndex>,
) {
    for (idx, first) in indices.iter().enumerate() {
        for second in indices.iter().skip(idx + 1) {
            add_antinodes(tilemap, (*first, *second), remote_range, set);
        }
    }
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let tilemap: VecMatrix<Tile> = {
        let mut lines = lines.peekable();

        let width = lines.by_ref().peek().map_or(0, String::len);
        let data = lines
            .flat_map(String::into_bytes)
            .map(|byte| byte as char)
            .map(Into::into)
            .collect::<Vec<_>>();

        VecMatrix::with_data(data, width)
    };

    let mut nodes: HashMap<char, Vec<MatrixIndex>> = HashMap::new();

    for (idx, tile) in tilemap.iter_enumerate() {
        if let Tile::Node(ch) = *tile {
            nodes.entry(ch).or_default().push(idx);
        }
    }

    let mut antinodes_close: HashSet<MatrixIndex> = HashSet::new();
    let mut antinodes_far: HashSet<MatrixIndex> = HashSet::new();

    for indices in nodes.into_values() {
        add_all_antinodes(&tilemap, &indices, &(1..=1), &mut antinodes_close);
        add_all_antinodes(&tilemap, &indices, &(0..), &mut antinodes_far);
    }

    Ok((antinodes_close.len(), antinodes_far.len()))
}
