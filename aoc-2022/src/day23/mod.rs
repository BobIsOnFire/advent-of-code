use std::collections::{HashMap, HashSet};

use aoc_common::util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    const fn coords_around(self) -> [Self; 8] {
        [
            Self { row: self.row - 1, col: self.col - 1 },
            Self { row: self.row - 1, col: self.col },
            Self { row: self.row - 1, col: self.col + 1 },
            Self { row: self.row, col: self.col - 1 },
            Self { row: self.row, col: self.col + 1 },
            Self { row: self.row + 1, col: self.col - 1 },
            Self { row: self.row + 1, col: self.col },
            Self { row: self.row + 1, col: self.col + 1 },
        ]
    }

    fn coords_around_at(self, direction: Direction) -> [Self; 3] {
        match direction {
            Direction::North => [-1, 0, 1].map(|inc| Self {
                row: self.row - 1,
                col: self.col + inc,
            }),
            Direction::South => [-1, 0, 1].map(|inc| Self {
                row: self.row + 1,
                col: self.col + inc,
            }),
            Direction::West => [-1, 0, 1].map(|inc| Self {
                row: self.row + inc,
                col: self.col - 1,
            }),
            Direction::East => [-1, 0, 1].map(|inc| Self {
                row: self.row + inc,
                col: self.col + 1,
            }),
        }
    }

    const fn coord_at(self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self { row: self.row - 1, col: self.col },
            Direction::South => Self { row: self.row + 1, col: self.col },
            Direction::West => Self { row: self.row, col: self.col - 1 },
            Direction::East => Self { row: self.row, col: self.col + 1 },
        }
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.row, self.col)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

struct Field {
    elves: HashSet<Coord>,
}

impl Field {
    fn new() -> Self {
        Self { elves: HashSet::new() }
    }

    const fn with_elves(elves: HashSet<Coord>) -> Self {
        Self { elves }
    }

    fn has_around(&self, elf: Coord) -> bool {
        elf.coords_around().iter().any(|other| self.elves.contains(other))
    }

    fn has_around_at(&self, elf: Coord, direction: Direction) -> bool {
        elf.coords_around_at(direction).iter().any(|other| self.elves.contains(other))
    }

    fn get_field_bounds(&self) -> (Coord, Coord) {
        let mut top_left = *self.elves.iter().next().expect("At least one elf should exist in the field");
        let mut bottom_right = top_left;

        for elf in &self.elves {
            top_left.row = top_left.row.min(elf.row);
            top_left.col = top_left.col.min(elf.col);

            bottom_right.row = bottom_right.row.max(elf.row);
            bottom_right.col = bottom_right.col.max(elf.col);
        }

        (top_left, bottom_right)
    }

    #[allow(dead_code)]
    fn draw(&self) {
        let (top_left, bottom_right) = self.get_field_bounds();

        for row in top_left.row..=bottom_right.row {
            for col in top_left.col..=bottom_right.col {
                if self.elves.contains(&Coord { row, col }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut elves = self.elves.iter().map(|e| format!("{e}")).collect::<Vec<_>>();
        elves.sort();
        write!(f, "[{}]", elves.join(", "))
    }
}

const fn direction_order(round: usize) -> [Direction; 4] {
    let initial_order = [Direction::North, Direction::South, Direction::West, Direction::East];

    [
        initial_order[round % 4],
        initial_order[(round + 1) % 4],
        initial_order[(round + 2) % 4],
        initial_order[(round + 3) % 4],
    ]
}

fn play_round(field: &Field, round: usize) -> Field {
    // key is the final location of elf, value is the initial position of elf that moves there
    let mut elf_moves: HashMap<Coord, Coord> = HashMap::new();

    for &elf in &field.elves {
        let elf_move = if field.has_around(elf) {
            direction_order(round)
                .into_iter()
                .find(|&d| !field.has_around_at(elf, d))
                .map_or(elf, |d| elf.coord_at(d))
        } else {
            elf // Not moving
        };

        if let Some(other_elf) = elf_moves.remove(&elf_move) {
            // If someone else wanted to move into this cell, nobody moves there
            elf_moves.insert(elf, elf);
            elf_moves.insert(other_elf, other_elf);
        } else {
            elf_moves.insert(elf_move, elf);
        }
    }

    // I hope we didn't lose any elves here!
    assert_eq!(elf_moves.len(), field.elves.len());

    Field::with_elves(elf_moves.keys().copied().collect())
}

pub fn spread_elves(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut field = Field::new();

    for (row, line) in lines.enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                field.elves.insert(Coord { row: row as i32, col: col as i32 });
            }
        }
    }

    let mut round = 0;
    let mut empty_tiles_at_round_10 = 0;

    loop {
        let new_field = play_round(&field, round);
        round += 1;
        if new_field.elves == field.elves {
            break;
        }

        field = new_field;

        if round == 10 {
            let (top_left, bottom_right) = field.get_field_bounds();

            let total_area = (bottom_right.row - top_left.row + 1) as usize * (bottom_right.col - top_left.col + 1) as usize;
            empty_tiles_at_round_10 = total_area - field.elves.len();
            // break;
        }
    }

    Ok((empty_tiles_at_round_10, round))
}
