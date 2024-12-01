use std::collections::HashSet;

use aoc_common::util;

pub fn get_distance(lines: impl Iterator<Item = String>) -> util::GenericResult<(u64, u64)> {
    let mut left: Vec<u64> = vec![];
    let mut right: Vec<u64> = vec![];

    for line in lines {
        let mut lexer = util::Lexer::of(&line);

        left.push(lexer.unsigned_number()?);
        lexer.whitespace()?;
        right.push(lexer.unsigned_number()?);
        lexer.end()?;
    }

    left.sort_unstable();
    right.sort_unstable();

    let differences = left.iter().zip(right.iter()).map(|(&l, &r)| u64::abs_diff(l, r)).sum::<u64>();

    // the problem asks to find number of times left numbers appear in right list, multiplied by the number itself
    // we can reinterpret that to sum of right numbers that also exist in left list to make calculations easier
    let left_set = HashSet::<u64>::from_iter(left.iter().copied());
    let right_sum = right.iter().filter(|v| left_set.contains(v)).sum::<u64>();

    Ok((differences, right_sum))
}
