use std::collections::HashMap;

use aoc_common::util;
use model::{DirectionButton, KeypadButton, KeypadPosition, NumericButton};

mod model;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Movement<T: KeypadButton> {
    from: T,
    to: T,
}

impl<T: KeypadButton> Movement<T> {
    pub const fn new(from: T, to: T) -> Self {
        Self { from, to }
    }

    pub fn into_higher_level_presses(self, row_first: bool) -> Option<Vec<DirectionButton>> {
        use DirectionButton::{Confirm, Down, Left, Right, Up};

        let from_pos: KeypadPosition = self.from.into();
        let to_pos: KeypadPosition = self.to.into();

        let iter_row = std::iter::repeat_n(
            if to_pos.row > from_pos.row { Down } else { Up },
            usize::abs_diff(to_pos.row, from_pos.row),
        );

        let iter_col = std::iter::repeat_n(
            if to_pos.col > from_pos.col { Right } else { Left },
            usize::abs_diff(to_pos.col, from_pos.col),
        );

        let presses = if row_first { iter_row.chain(iter_col) } else { iter_col.chain(iter_row) }
            .chain(std::iter::once(Confirm))
            .collect();

        let mut current: T = from_pos.try_into().ok()?;
        for &press in &presses {
            let pos: KeypadPosition = current.into();
            let next_pos = match press {
                Left if pos.col == 0 => return None,
                Up if pos.row == 0 => return None,
                Confirm => pos,
                Left => KeypadPosition { row: pos.row, col: pos.col - 1 },
                Right => KeypadPosition { row: pos.row, col: pos.col + 1 },
                Up => KeypadPosition { row: pos.row - 1, col: pos.col },
                Down => KeypadPosition { row: pos.row + 1, col: pos.col },
            };

            current = next_pos.try_into().ok()?;
        }
        assert!(current.into() == to_pos);

        Some(presses)
    }
}

struct KeypadSetCache {
    best_weights: Vec<HashMap<Movement<DirectionButton>, usize>>,
}

impl KeypadSetCache {
    fn new(levels: usize) -> Self {
        Self {
            best_weights: vec![HashMap::new(); levels],
        }
    }

    fn get_movement_weight(&mut self, dirmove: Movement<DirectionButton>, level: usize) -> usize {
        if let Some(best_weight) = self.best_weights[level].get(&dirmove) {
            return *best_weight;
        }

        let best_weight = self.find_best_weight(dirmove, level + 1);
        self.best_weights[level].insert(dirmove, best_weight);

        best_weight
    }

    fn get_presses_weight(&mut self, presses: &[DirectionButton], next_level: usize) -> usize {
        if next_level == self.best_weights.len() {
            presses.len()
        } else {
            let mut prev_press = DirectionButton::Confirm;
            let mut weight = 0;

            for &press in presses {
                let next_move = Movement::new(prev_press, press);
                weight += self.get_movement_weight(next_move, next_level);
                prev_press = press;
            }

            weight
        }
    }

    fn find_best_weight<T: KeypadButton + Copy + std::fmt::Debug>(
        &mut self,
        movement: Movement<T>,
        next_level: usize,
    ) -> usize {
        let presses_vertical = movement.into_higher_level_presses(true);
        let presses_horizontal = movement.into_higher_level_presses(false);

        match (presses_vertical, presses_horizontal) {
            (None, None) => panic!("Cannot perform {movement:?} for some reason"),
            (None, Some(presses)) | (Some(presses), None) => {
                self.get_presses_weight(&presses, next_level)
            }
            (Some(vertical), Some(horizontal)) => {
                let weight_vertical = self.get_presses_weight(&vertical, next_level);
                let weight_horizontal = self.get_presses_weight(&horizontal, next_level);

                usize::min(weight_vertical, weight_horizontal)
            }
        }
    }

    fn get_numeric_weight(&mut self, numeric: &[NumericButton]) -> usize {
        let mut weight_total = 0;
        let mut prev_number = NumericButton::Confirm;

        for &num in numeric {
            weight_total += self.find_best_weight(Movement::new(prev_number, num), 0);
            prev_number = num;
        }

        weight_total
    }
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut complexity_short = 0;
    let mut complexity_long = 0;

    let mut cache_short = KeypadSetCache::new(2);
    let mut cache_long = KeypadSetCache::new(25);

    for line in lines {
        let mut value = 0;
        let mut numeric = vec![];

        for ch in line.chars() {
            let button = ch.try_into()?;
            if let NumericButton::Number(num) = button {
                value = value * 10 + num as usize;
            }
            numeric.push(button);
        }

        let weight_short = cache_short.get_numeric_weight(&numeric);
        let weight_long = cache_long.get_numeric_weight(&numeric);

        complexity_short += value * weight_short;
        complexity_long += value * weight_long;
    }

    Ok((complexity_short, complexity_long))
}
