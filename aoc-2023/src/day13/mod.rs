use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Invalid tile: {value}"),
        }
    }
}

fn has_vertical_mirror_at(tilemap: &VecMatrix<Tile>, at_col: usize, smudge_total: usize) -> bool {
    let mut smudge_count = 0;

    for row in 0..tilemap.height() {
        let mut left = MatrixIndex { row, col: at_col };
        let mut right = MatrixIndex { row, col: at_col + 1 };
        loop {
            if tilemap[left] != tilemap[right] {
                smudge_count += 1;
                if smudge_count > smudge_total {
                    return false;
                }
            }

            if let (Some(next_left), Some(next_right)) =
                (tilemap.next_left(left), tilemap.next_right(right))
            {
                left = next_left;
                right = next_right;
            } else {
                break;
            }
        }
    }

    smudge_count == smudge_total
}

fn has_horizontal_mirror_at(tilemap: &VecMatrix<Tile>, at_row: usize, smudge_total: usize) -> bool {
    let mut smudge_count = 0;

    for col in 0..tilemap.width() {
        let mut up = MatrixIndex { col, row: at_row };
        let mut down = MatrixIndex { col, row: at_row + 1 };
        loop {
            if tilemap[up] != tilemap[down] {
                smudge_count += 1;
                if smudge_count > smudge_total {
                    return false;
                }
            }

            if let (Some(next_up), Some(next_down)) = (tilemap.next_up(up), tilemap.next_down(down))
            {
                up = next_up;
                down = next_down;
            } else {
                break;
            }
        }
    }

    smudge_count == smudge_total
}

fn find_mirrors(tilemap: &VecMatrix<Tile>, smudge_total: usize) -> usize {
    let mut mirror_sum = 0;

    for col in 0..(tilemap.width() - 1) {
        if has_vertical_mirror_at(tilemap, col, smudge_total) {
            mirror_sum += col + 1;
        }
    }

    for row in 0..(tilemap.height() - 1) {
        if has_horizontal_mirror_at(tilemap, row, smudge_total) {
            mirror_sum += 100 * (row + 1);
        }
    }

    mirror_sum
}

pub fn count_mirrors(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut mirror_sum = 0;
    let mut mirror_sum_with_smudge = 0;

    let mut lines = lines.peekable();

    loop {
        let pattern_lines = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>();

        let mut tilemap = VecMatrix::new(pattern_lines[0].len());
        for line in pattern_lines {
            tilemap.extend(line.chars().map(Tile::from));
        }

        mirror_sum += find_mirrors(&tilemap, 0);
        mirror_sum_with_smudge += find_mirrors(&tilemap, 1);

        if lines.peek().is_none() {
            break;
        }
    }

    Ok((mirror_sum, mirror_sum_with_smudge))
}
