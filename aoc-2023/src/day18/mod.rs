use aoc_common::util;

#[derive(Clone, Copy)]
struct Coord {
    row: i64,
    col: i64,
}

impl Coord {
    fn next(&self, direction: Direction, moves: usize) -> Self {
        let moves = moves as i64;
        match direction {
            Direction::Left => Coord { row: self.row, col: self.col - moves },
            Direction::Right => Coord { row: self.row, col: self.col + moves },
            Direction::Up => Coord { row: self.row - moves, col: self.col },
            Direction::Down => Coord { row: self.row + moves, col: self.col },
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_letter(ch: char) -> Self {
        match ch {
            'R' => Self::Right,
            'D' => Self::Down,
            'L' => Self::Left,
            'U' => Self::Up,
            _ => panic!("Unknown direction letter: {}", ch),
        }
    }

    fn from_digit(ch: char) -> Self {
        match ch {
            '0' => Self::Right,
            '1' => Self::Down,
            '2' => Self::Left,
            '3' => Self::Up,
            _ => panic!("Unknown direction digit: {}", ch),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct HorizontalBorder {
    row: i64,
    col_from: i64,
    col_to: i64,
}

impl HorizontalBorder {
    fn width(&self) -> usize {
        (self.col_to - self.col_from + 1) as usize
    }

    fn contains(&self, other: &HorizontalBorder) -> bool {
        self.col_from <= other.col_from && self.col_to >= other.col_to
    }

    fn is_extended_right(&self, other: &HorizontalBorder) -> bool {
        self.col_to == other.col_from
    }

    fn is_extended_left(&self, other: &HorizontalBorder) -> bool {
        self.col_from == other.col_to
    }

    fn intersects(&self, other: &HorizontalBorder) -> bool {
        (self.col_from..=self.col_to).contains(&other.col_from)
            || (self.col_from..=self.col_to).contains(&other.col_to)
            || (other.col_from..=other.col_to).contains(&self.col_from)
            || (other.col_from..=other.col_to).contains(&self.col_to)
    }
}

fn get_covered_area(moves_data: Vec<(Direction, usize)>) -> usize {
    let mut current = Coord { row: 0, col: 0 };
    let mut borders = vec![];

    for (direction, moves) in moves_data {
        let next = current.next(direction, moves);

        if current.row == next.row {
            borders.push(HorizontalBorder {
                row: current.row,
                col_from: i64::min(current.col, next.col),
                col_to: i64::max(current.col, next.col),
            });
        }

        current = next;
    }

    borders.sort_unstable();

    let mut current: Vec<HorizontalBorder> = vec![];
    let mut area = 0;

    for border in borders {
        current.sort_unstable_by_key(|b| b.col_from);

        if !current.iter().any(|b| b.intersects(&border)) {
            current.push(border);
            continue;
        }

        if let Some(idx) = (0..current.len()).find(|idx| current[*idx].contains(&border)) {
            let container = current.swap_remove(idx);
            let mut width_left = container.width();

            if container.col_from < border.col_from {
                let left = HorizontalBorder {
                    row: border.row,
                    col_from: container.col_from,
                    col_to: border.col_from,
                };
                width_left -= left.width();
                current.push(left);
            }

            if container.col_to > border.col_to {
                let right = HorizontalBorder {
                    row: border.row,
                    col_from: border.col_to,
                    col_to: container.col_to,
                };
                width_left -= right.width();
                current.push(right);
            }

            area += container.width() * (border.row - container.row) as usize + width_left;

            continue;
        }

        if let Some(idx) = (0..current.len()).find(|idx| current[*idx].is_extended_left(&border)) {
            if idx != 0 && current[idx - 1].is_extended_right(&border) {
                let right = current.swap_remove(idx);
                let left = current.swap_remove(idx - 1);
                area += left.width() * (border.row - left.row) as usize;
                area += right.width() * (border.row - right.row) as usize;

                let merged = HorizontalBorder {
                    row: border.row,
                    col_from: left.col_from,
                    col_to: right.col_to,
                };
                current.push(merged);
                continue;
            } else {
                let extended = &mut current[idx];
                area += extended.width() * (border.row - extended.row) as usize;
                extended.row = border.row;
                extended.col_from = border.col_from;
                continue;
            }
        }

        if let Some(idx) = (0..current.len()).find(|idx| current[*idx].is_extended_right(&border)) {
            let extended = &mut current[idx];
            area += extended.width() * (border.row - extended.row) as usize;
            extended.row = border.row;
            extended.col_to = border.col_to;
            continue;
        }
    }

    area
}

pub fn dig_lagoon(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut moves_data = vec![];
    let mut moves_data_colored = vec![];

    for line in lines {
        let mut lexer = util::Lexer::of(&line);

        let direction = Direction::from_letter(lexer.symbol()?);
        lexer.whitespace()?;
        let moves = lexer.number::<usize>()?;
        lexer.whitespace()?;

        moves_data.push((direction, moves));

        lexer.literal("(#")?;
        let moves = usize::from_str_radix(lexer.take(5)?, 16)?;
        let direction = Direction::from_digit(lexer.symbol()?);
        lexer.literal(")")?;
        lexer.end()?;

        moves_data_colored.push((direction, moves))
    }

    let area = get_covered_area(moves_data);
    let area_colored = get_covered_area(moves_data_colored);

    Ok((area, area_colored))
}
