use std::iter::{Product, Sum};

use aoc_common::util;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

impl TryFrom<char> for Operation {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Multiply),
            _ => Err("Expected '+' or '*'".to_string()),
        }
    }
}

fn fold_by_operation<T: IntoIterator>(it: T, op: Operation) -> usize
where
    usize: Sum<T::Item> + Product<T::Item>,
{
    match op {
        Operation::Add => it.into_iter().sum(),
        Operation::Multiply => it.into_iter().product(),
    }
}

pub fn do_maths(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let (num_lines, ops_line) = {
        let mut lines = lines.collect::<Vec<_>>();
        let ops_line = lines.pop().unwrap();

        (lines, ops_line)
    };

    let mut ops = ops_line.chars().filter(|ch| !ch.is_ascii_whitespace());

    let mut numbers_rows: Vec<usize> = vec![0; num_lines.len()];
    let mut numbers_cols: Vec<usize> = vec![];

    let mut result_rows = 0;
    let mut result_cols = 0;

    for i in 0..num_lines[0].len() {
        let mut num = 0;
        let mut all_whitespace = true;
        for (idx, line) in num_lines.iter().enumerate() {
            if !line.as_bytes()[i].is_ascii_whitespace() {
                let digit = (line.as_bytes()[i] - b'0') as usize;

                num = num * 10 + digit;
                numbers_rows[idx] = numbers_rows[idx] * 10 + digit;

                all_whitespace = false;
            }
        }
        if all_whitespace {
            let op: Operation = ops.next().ok_or("Not enough ops")?.try_into()?;
            result_cols += fold_by_operation(numbers_cols.drain(..), op);
            result_rows += fold_by_operation(numbers_rows.iter(), op);

            numbers_rows.fill(0);
        } else {
            numbers_cols.push(num);
        }
    }
    let op: Operation = ops.next().ok_or("Not enough ops")?.try_into()?;
    result_cols += fold_by_operation(numbers_cols, op);
    result_rows += fold_by_operation(numbers_rows, op);

    Ok((result_rows, result_cols))
}
