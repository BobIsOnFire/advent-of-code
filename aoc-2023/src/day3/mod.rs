use std::collections::{HashMap, HashSet};

use aoc_common::util::{self, MatrixIndex};

type Coord = MatrixIndex;

#[derive(Debug)]
struct Number {
    value: u32,
    from: Coord,
    to: Coord,
}

impl Number {
    fn surrounding_tiles(&self) -> impl Iterator<Item = Coord> {
        let top_left = Coord {
            row: self.from.row.saturating_sub(1),
            col: self.from.col.saturating_sub(1),
        };
        let bottom_right = Coord {
            row: self.to.row.saturating_add(1),
            col: self.to.col.saturating_add(1),
        };

        let row_range = top_left.row..=bottom_right.row;
        let col_range = top_left.col..=bottom_right.col;

        row_range.flat_map(move |row| col_range.clone().map(move |col| Coord { row, col }))
    }
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(u32, u32)> {
    let mut symbols: HashSet<Coord> = HashSet::new();
    let mut gears: HashSet<Coord> = HashSet::new();
    let mut numbers: Vec<Number> = Vec::new();

    for (row, line) in lines.enumerate() {
        let mut lexer = util::Lexer::of(&line);
        while lexer.end().is_err() {
            let char_coord = Coord {
                row,
                col: lexer.position(),
            };

            if let Ok(num) = lexer.unsigned_number() {
                let end_coord = Coord {
                    row,
                    col: lexer.position() - 1,
                };

                numbers.push(Number {
                    value: num,
                    from: char_coord,
                    to: end_coord,
                });
            } else {
                let ch = lexer.symbol()?;
                if ch != '.' {
                    symbols.insert(char_coord);
                }
                if ch == '*' {
                    gears.insert(char_coord);
                }
            }
        }
    }

    let part_numbers_sum = numbers
        .iter()
        .filter(|num| num.surrounding_tiles().any(|tile| symbols.contains(&tile)))
        .map(|num| num.value)
        .sum();

    let mut gear_candidates: HashMap<Coord, Vec<u32>> = HashMap::new();

    for num in numbers {
        num.surrounding_tiles()
            .filter(|coord| gears.contains(coord))
            .for_each(|gear| gear_candidates.entry(gear).or_default().push(num.value));
    }

    let gear_ratio_sum = gear_candidates
        .values()
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0] * numbers[1])
        .sum();

    Ok((part_numbers_sum, gear_ratio_sum))
}
