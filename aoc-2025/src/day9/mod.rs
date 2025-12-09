use std::collections::HashMap;

use aoc_common::util::{self, iter::IteratorExtended};

#[derive(Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

fn parse_coord(line: &str) -> util::lexer::Result<Coord> {
    let mut lexer = util::Lexer::of(line);
    let col = lexer.unsigned_number()?;
    lexer.literal(",")?;
    let row = lexer.unsigned_number()?;
    lexer.end()?;

    Ok(Coord { row, col })
}

fn rectangle_area(first: Coord, second: Coord) -> usize {
    (first.row.abs_diff(second.row) + 1) * (first.col.abs_diff(second.col) + 1)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Forward,
    Backward,
}

fn dedup_bounds(bounds: &mut Vec<(usize, Direction)>) {
    bounds.sort_unstable_by_key(|(num, _)| *num);
    let init_direction = bounds[0].1;
    let mut i = 0;
    let mut insert = 0;
    while i < bounds.len() {
        let mut next = i;
        while next < bounds.len() && bounds[next].1 == bounds[i].1 {
            next += 1;
        }
        if bounds[i].1 == init_direction {
            bounds.swap(insert, i);
        } else {
            bounds.swap(insert, next - 1);
        }
        i = next;
        insert += 1;
    }
    bounds.resize(insert, (0, Direction::Forward));
}

fn is_line_in_bounds(bounds: &[(usize, Direction)], from: usize, to: usize) -> bool {
    let (from, to) = (usize::min(from, to), usize::max(from, to));

    for [(bound_start, _), (bound_end, _)] in bounds.iter().copied().groups::<2>() {
        let range = bound_start..=bound_end;
        if range.contains(&from) {
            return range.contains(&to);
        }
    }

    false
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let coords = lines
        .map(|line| parse_coord(&line))
        .collect::<Result<Vec<_>, _>>()?;

    let mut row_bounds: HashMap<usize, Vec<(usize, Direction)>> = HashMap::new();
    let mut col_bounds: HashMap<usize, Vec<(usize, Direction)>> = HashMap::new();

    for i in 0..coords.len() {
        let from = coords[i];
        let to = coords.get(i + 1).copied().unwrap_or(coords[0]);

        let (from, to, direction) = if from.row > to.row || from.col > to.col {
            (to, from, Direction::Backward)
        } else {
            (from, to, Direction::Forward)
        };

        if from.row == to.row {
            // horizontal line
            for col in from.col..=to.col {
                col_bounds
                    .entry(col)
                    .or_default()
                    .push((from.row, direction));
            }
        } else {
            // vertical line
            for row in from.row..=to.row {
                row_bounds
                    .entry(row)
                    .or_default()
                    .push((from.col, direction));
            }
        }
    }

    for v in row_bounds.values_mut() {
        dedup_bounds(v);
    }
    for v in col_bounds.values_mut() {
        dedup_bounds(v);
    }

    let mut max_total = 0;
    let mut max_in_shape = 0;
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let (first, second) = (coords[i], coords[j]);
            let area = rectangle_area(first, second);

            max_total = max_total.max(area);
            if is_line_in_bounds(&row_bounds[&first.row], first.col, second.col)
                && is_line_in_bounds(&row_bounds[&second.row], first.col, second.col)
                && is_line_in_bounds(&col_bounds[&first.col], first.row, second.row)
                && is_line_in_bounds(&col_bounds[&second.col], first.row, second.row)
            {
                max_in_shape = max_in_shape.max(area);
            }
        }
    }

    Ok((max_total, max_in_shape))
}
