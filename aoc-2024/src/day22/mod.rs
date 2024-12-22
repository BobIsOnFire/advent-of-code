use std::collections::HashMap;

use aoc_common::util;

type Sequence = [i8; 4];

const fn get_next_number(num: u64) -> u64 {
    let num = (num ^ (num * 64)) % 16_777_216;
    let num = (num ^ (num / 32)) % 16_777_216;
    (num ^ (num * 2048)) % 16_777_216
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(u64, u64)> {
    let mut secrets_sum = 0;
    let mut all_prices: Vec<HashMap<Sequence, u64>> = vec![];

    for line in lines {
        let mut num = line.parse()?;
        let mut prev_digit = num % 10;

        let mut sequence = [0; 4];
        let mut prices = HashMap::new();
        for step in 0..2000 {
            num = get_next_number(num);

            let digit = num % 10;
            sequence = [
                sequence[1],
                sequence[2],
                sequence[3],
                digit as i8 - prev_digit as i8,
            ];
            if step >= 3 {
                prices.entry(sequence).or_insert(digit);
            }
            prev_digit = digit;
        }

        secrets_sum += num;
        all_prices.push(prices);
    }

    let mut sequences_sum: HashMap<[i8; 4], u64> = HashMap::new();
    for prices in all_prices {
        for (seq, price) in prices {
            *sequences_sum.entry(seq).or_default() += price;
        }
    }

    let max_profit = sequences_sum.into_values().max().unwrap_or(0);

    Ok((secrets_sum, max_profit))
}
