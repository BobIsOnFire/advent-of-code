use std::collections::BTreeMap;

use aoc_common::util::{self, MatrixIndex, VecMatrix};

const MAP_SIZE: usize = 71;
const BYTES_TO_CHECK: usize = 1024;

fn find_fastest_path(corrupted: &VecMatrix<bool>, start: MatrixIndex, end: MatrixIndex) -> usize {
    let mut visited = VecMatrix::with_data(vec![false; corrupted.len()], corrupted.width());
    let mut lowest_cost =
        VecMatrix::with_data(vec![usize::MAX; corrupted.len()], corrupted.width());

    let mut to_visit: BTreeMap<usize, Vec<MatrixIndex>> = BTreeMap::new();
    to_visit.insert(0, vec![start]);

    while let Some((cost, nodes)) = to_visit.pop_first() {
        for node in nodes {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            lowest_cost[node] = cost;

            if node == end {
                break;
            }

            for next in std::iter::empty()
                .chain(corrupted.next_up(node))
                .chain(corrupted.next_left(node))
                .chain(corrupted.next_down(node))
                .chain(corrupted.next_right(node))
            {
                if !corrupted[next] && !visited[next] {
                    to_visit.entry(cost + 1).or_default().push(next);
                }
            }
        }
    }

    lowest_cost[end]
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, String)> {
    let mut bytes = vec![];

    for line in lines {
        let mut lexer = util::Lexer::of(&line);
        let col = lexer.unsigned_number()?;
        lexer.literal(",")?;
        let row = lexer.unsigned_number()?;
        lexer.end()?;

        bytes.push(MatrixIndex { row, col });
    }

    let mut corrupted = VecMatrix::with_data(vec![false; MAP_SIZE * MAP_SIZE], MAP_SIZE);
    for byte in &bytes[..BYTES_TO_CHECK] {
        corrupted[*byte] = true;
    }

    let fastest = find_fastest_path(
        &corrupted,
        MatrixIndex { row: 0, col: 0 },
        MatrixIndex { row: MAP_SIZE - 1, col: MAP_SIZE - 1 },
    );

    let mut corrupted = VecMatrix::with_data(vec![false; MAP_SIZE * MAP_SIZE], MAP_SIZE);
    let mut first_corrupt = MatrixIndex { row: usize::MAX, col: usize::MAX };
    for byte in bytes {
        corrupted[byte] = true;
        let fastest = find_fastest_path(
            &corrupted,
            MatrixIndex { row: 0, col: 0 },
            MatrixIndex { row: MAP_SIZE - 1, col: MAP_SIZE - 1 },
        );
        if fastest == usize::MAX {
            first_corrupt = byte;
            break;
        }
    }

    Ok((
        fastest,
        format!("{},{}", first_corrupt.col, first_corrupt.row),
    ))
}
