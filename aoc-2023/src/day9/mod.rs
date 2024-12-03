use aoc_common::util;

fn get_sequence_element(values: &[i64], at: i64) -> i64 {
    let mut multiplier = 1;
    let mut count = 0;

    let mut element = 0;

    // I don't want to convert types here, it will look messy
    #[allow(clippy::explicit_counter_loop)]
    for value in values {
        element += value * multiplier;

        multiplier = (multiplier * (at - count)) / (count + 1);
        count += 1;
    }

    element
}

pub fn extrapolate_sequence(lines: impl Iterator<Item = String>) -> util::GenericResult<(i64, i64)> {
    let mut next_values_sum = 0;
    let mut prev_values_sum = 0;

    for line in lines {
        let mut lexer = util::Lexer::of(&line);
        let numbers = std::iter::from_fn(|| {
            if lexer.end().is_ok() {
                None
            } else {
                Some(lexer.whitespace().and_then(|()| lexer.number()))
            }
        })
        .collect::<Result<Vec<i64>, _>>()?;

        #[rustfmt::skip]
        let first_elems = std::iter::successors(
            Some(numbers.clone()),
            |seq| Some(seq.windows(2).map(|win| win[1] - win[0]).collect())
        )
        .take_while(|seq| seq.iter().any(|&num| num != 0))
        .map(|seq| seq[0])
        .collect::<Vec<_>>();

        next_values_sum += get_sequence_element(&first_elems, numbers.len() as i64);
        prev_values_sum += get_sequence_element(&first_elems, -1);
    }

    Ok((next_values_sum, prev_values_sum))
}
