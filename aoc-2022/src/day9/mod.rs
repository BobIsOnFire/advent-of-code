use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use aoc_common::util::{self, Lexer};

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coords(i64, i64);

impl Sub<Coords> for Coords {
    type Output = Coords;

    fn sub(self, rhs: Coords) -> Self::Output {
        Coords(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<Coords> for Coords {
    type Output = Coords;

    fn add(self, rhs: Coords) -> Self::Output {
        Coords(self.0 + rhs.0, self.1 + rhs.1)
    }
}

struct Rope<const N: usize> {
    knots: [Coords; N],
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self { knots: [Coords(0, 0); N] }
    }

    fn follow(&mut self) {
        for ind in 1..N {
            let diff = self.knots[ind - 1] - self.knots[ind];
            let diff_abs = Coords(diff.0.abs(), diff.1.abs());

            let change_abs = if diff_abs.0 < 2 && diff_abs.1 < 2 {
                Coords(0, 0)
            } else if diff_abs.0 == 2 {
                if diff_abs.1 > 0 {
                    Coords(1, 1)
                } else {
                    Coords(1, 0)
                }
            } else if diff_abs.0 == 1 {
                Coords(1, 1)
            } else {
                Coords(0, 1)
            };

            let change = Coords(change_abs.0 * diff.0.signum(), change_abs.1 * diff.1.signum());

            self.knots[ind] = self.knots[ind] + change;
        }
    }

    fn perform(&mut self, op: char) {
        match op {
            'U' => self.knots[0].1 += 1,
            'D' => self.knots[0].1 -= 1,
            'L' => self.knots[0].0 -= 1,
            'R' => self.knots[0].0 += 1,
            _ => panic!("Invalid operation code"),
        }

        self.follow();
    }
}

pub fn count_unique_positions(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut rope = Rope::<10>::new();

    let mut set_short = HashSet::new();
    let mut set_long = HashSet::new();
    set_short.insert(Coords(0, 0));
    set_long.insert(Coords(0, 0));

    for line in lines {
        let mut lexer = Lexer::of(&line);

        let op = lexer.symbol()?;
        lexer.literal(" ")?;
        let count = lexer.unsigned_number()?;
        lexer.end()?;

        for _ in 0..count {
            rope.perform(op);
            set_short.insert(rope.knots[1]);
            set_long.insert(rope.knots[9]);
        }
    }

    Ok((set_short.len(), set_long.len()))
}
