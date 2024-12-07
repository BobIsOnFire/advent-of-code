use aoc_common::util;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

const fn perform(lhs: u64, rhs: u64, op: Operation) -> u64 {
    match op {
        Operation::Add => lhs + rhs,
        Operation::Multiply => lhs * rhs,
        Operation::Concatenate => lhs * 10u64.pow(rhs.ilog10() + 1) + rhs,
    }
}

fn do_can_calculate(result: u64, sum: u64, numbers: &[u64], ops: &[Operation]) -> bool {
    if sum > result {
        return false;
    }

    let Some((first, rest)) = numbers.split_first() else {
        return result == sum;
    };

    ops.iter()
        .any(|&op| do_can_calculate(result, perform(sum, *first, op), rest, ops))
}

fn can_calculate(result: u64, numbers: &[u64], ops: &[Operation]) -> bool {
    let Some((first, rest)) = numbers.split_first() else {
        // WTF, there are no numbers in the list?
        return false;
    };

    do_can_calculate(result, *first, rest, ops)
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(u64, u64)> {
    use Operation::{Add, Concatenate, Multiply};

    let mut sum = 0;
    let mut sum_with_concat = 0;

    for line in lines {
        let mut lexer = util::Lexer::of(&line);

        let result = lexer.unsigned_number()?;
        lexer.literal(":")?;

        let numbers = std::iter::from_fn(|| {
            lexer
                .whitespace()
                .and_then(|()| lexer.unsigned_number())
                .ok()
        })
        .collect::<Vec<_>>();

        if can_calculate(result, &numbers, &[Add, Multiply]) {
            sum += result;
        }
        if can_calculate(result, &numbers, &[Add, Multiply, Concatenate]) {
            sum_with_concat += result;
        }
    }

    Ok((sum, sum_with_concat))
}
