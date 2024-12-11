use std::collections::HashMap;

use aoc_common::util;

fn next_step(stones_count: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut next_count = HashMap::new();

    for (stone, count) in stones_count {
        if stone == 0 {
            *next_count.entry(1u64).or_default() += count;
            continue;
        }

        let len = stone.ilog10() + 1;
        if len % 2 == 0 {
            let split = 10u64.pow(len / 2);
            *next_count.entry(stone / split).or_default() += count;
            *next_count.entry(stone % split).or_default() += count;
        } else {
            *next_count.entry(stone * 2024).or_default() += count;
        }
    }

    next_count
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let stones = lines
        .into_iter()
        .next()
        .ok_or("Input is empty")?
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()?;

    let mut stones_count: HashMap<u64, usize> = HashMap::new();

    for stone in stones {
        *stones_count.entry(stone).or_default() += 1;
    }

    for _ in 1..=25 {
        stones_count = next_step(stones_count);
    }
    let twenty_five_count = stones_count.values().sum::<usize>();

    for _ in 26..=75 {
        stones_count = next_step(stones_count);
    }
    let seventy_five_count = stones_count.values().sum::<usize>();

    Ok((twenty_five_count, seventy_five_count))
}
