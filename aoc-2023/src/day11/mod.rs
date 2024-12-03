use aoc_common::util::{self, MatrixIndex};

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut cells = Vec::new();

    for (row, line) in lines.enumerate() {
        let cell_cols = line.chars().enumerate().filter_map(|(col, ch)| (ch == '#').then_some(col));

        cells.extend(cell_cols.map(|col| MatrixIndex { row, col }));
    }

    cells.sort_unstable_by_key(|cell| cell.row);
    let mut empty_rows = 0;
    let mut prev = cells[0];
    for cell in &mut cells {
        empty_rows += (cell.row - prev.row).saturating_sub(1);
        prev = *cell;
        cell.row += empty_rows * 999_999;
    }

    cells.sort_unstable_by_key(|cell| cell.col);
    let mut empty_cols = 0;
    let mut prev = cells[0];
    for cell in &mut cells {
        empty_cols += (cell.col - prev.col).saturating_sub(1);
        prev = *cell;
        cell.col += empty_cols * 999_999;
    }

    let mut distance_sum = 0;
    for cell in &cells {
        for other in &cells {
            if cell == other {
                continue;
            }
            distance_sum += usize::abs_diff(cell.row, other.row) + usize::abs_diff(cell.col, other.col);
        }
    }

    Ok((distance_sum / 2, 0))
}
