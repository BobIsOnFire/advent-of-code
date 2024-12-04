use aoc_common::util::{self, VecMatrix};

#[derive(Debug)]
struct Tile {
    distance: usize,
    height: u8,
    is_start: bool,
    is_end: bool,
    visited: bool,
}

impl Tile {
    const fn of(ch: char) -> Self {
        let height_ch = match ch {
            'S' => 'a',
            'E' => 'z',
            ch => ch,
        };

        Self {
            distance: usize::MAX,
            height: (height_ch as u8) - b'a',
            is_start: ch == 'S',
            is_end: ch == 'E',
            visited: false,
        }
    }

    const fn can_cross(&self, other: &Self) -> bool {
        self.height < other.height || (self.height - other.height) <= 1
    }
}

fn walk_paths(mat: &mut VecMatrix<Tile>) -> util::GenericResult<()> {
    let start = mat
        .iter_enumerate()
        .find_map(|(idx, tile)| tile.is_end.then_some(idx))
        .ok_or("Start element not found")?;
    let mut current = vec![start];
    let mut next = vec![];
    mat[start].distance = 0;
    mat[start].visited = true;

    loop {
        for &idx in &current {
            let neighbours = [
                mat.next_up(idx),
                mat.next_left(idx),
                mat.next_down(idx),
                mat.next_right(idx),
            ];

            for neigh in neighbours.into_iter().flatten() {
                if !mat[neigh].visited && mat[idx].can_cross(&mat[neigh]) {
                    mat[neigh].distance = mat[idx].distance + 1;
                    mat[neigh].visited = true;
                    next.push(neigh);
                }
            }
        }

        if next.is_empty() {
            break;
        }

        std::mem::swap(&mut current, &mut next);
        next.clear();
    }

    Ok(())
}

pub fn find_shortest_distance(
    lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let mut lines = lines.peekable();

    let width = lines
        .peek()
        .ok_or("At least one line expected")?
        .as_bytes()
        .len();

    let mut mat = VecMatrix::new(width);
    lines.for_each(|line| mat.extend(line.chars().map(Tile::of)));

    walk_paths(&mut mat)?;

    let mut start_distance = usize::MAX;
    let mut min_distance = usize::MAX;

    for (_, tile) in mat.iter_enumerate() {
        if tile.is_start {
            start_distance = tile.distance;
        }

        if tile.height == 0 {
            min_distance = usize::min(min_distance, tile.distance);
        }
    }

    Ok((start_distance, min_distance))
}
