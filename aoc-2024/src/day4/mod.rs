use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    const fn all() -> [Self; 8] {
        [
            Self::Up,
            Self::UpRight,
            Self::Right,
            Self::DownRight,
            Self::Down,
            Self::DownLeft,
            Self::Left,
            Self::UpLeft,
        ]
    }
}

fn next_idx(matrix: &VecMatrix<char>, idx: MatrixIndex, direction: Direction) -> Option<MatrixIndex> {
    match direction {
        Direction::Up => matrix.next_up(idx),
        Direction::UpRight => matrix.next_up(idx).and_then(|i| matrix.next_right(i)),
        Direction::Right => matrix.next_right(idx),
        Direction::DownRight => matrix.next_down(idx).and_then(|i| matrix.next_right(i)),
        Direction::Down => matrix.next_down(idx),
        Direction::DownLeft => matrix.next_down(idx).and_then(|i| matrix.next_left(i)),
        Direction::Left => matrix.next_left(idx),
        Direction::UpLeft => matrix.next_up(idx).and_then(|i| matrix.next_left(i)),
    }
}

fn check_xmas_word(matrix: &VecMatrix<char>, start_idx: MatrixIndex, direction: Direction) -> bool {
    let letters = ['X', 'M', 'A', 'S'];
    let mut idx = start_idx;

    // skip(1): suppose 'X' is checked somewhere up the stack
    for letter in letters.into_iter().skip(1) {
        if let Some(next) = next_idx(matrix, idx, direction) {
            idx = next;
        } else {
            return false;
        }

        if letter != matrix[idx] {
            return false;
        }
    }

    true
}

fn count_xmas_words(matrix: &VecMatrix<char>, idx: MatrixIndex) -> usize {
    if matrix[idx] != 'X' {
        return 0;
    }

    let mut count = 0;

    for direction in Direction::all() {
        count += usize::from(check_xmas_word(matrix, idx, direction));
    }

    count
}

// Trying to implement nightly .transpose() without needing to allocate anything
fn array_transpose<T, const N: usize>(array: [Option<T>; N]) -> Option<[T; N]> {
    if array.iter().any(Option::is_none) {
        return None;
    }

    // SAFETY: already checked that entire array is Some above
    Some(array.map(|elem| unsafe { elem.unwrap_unchecked() }))
}

fn check_mas_cross(matrix: &VecMatrix<char>, idx: MatrixIndex) -> bool {
    if matrix[idx] != 'A' {
        return false;
    }

    let cross_directions = [Direction::UpRight, Direction::DownRight, Direction::DownLeft, Direction::UpLeft];
    let Some(cross_indices) = array_transpose(cross_directions.map(|d| next_idx(matrix, idx, d))) else {
        return false;
    };

    let cross_chars = cross_indices.map(|idx| matrix[idx]);

    // Note: two other combinations that are left ([M, S, M, S] and [S, M, S, M])
    // give 'MAM' and 'SAS' crosses, which is not suitable for us
    cross_chars == ['M', 'M', 'S', 'S']
        || cross_chars == ['M', 'S', 'S', 'M']
        || cross_chars == ['S', 'M', 'M', 'S']
        || cross_chars == ['S', 'S', 'M', 'M']
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let matrix = {
        let mut data = vec![];
        let mut width = 0;

        for line in lines {
            width = line.len();
            data.extend(line.chars());
        }

        VecMatrix::with_data(data, width)
    };

    let total_xmas = matrix.iter_enumerate().map(|(idx, _)| count_xmas_words(&matrix, idx)).sum();
    let total_crosses = matrix.iter_enumerate().filter(|(idx, _)| check_mas_cross(&matrix, *idx)).count();

    Ok((total_xmas, total_crosses))
}
