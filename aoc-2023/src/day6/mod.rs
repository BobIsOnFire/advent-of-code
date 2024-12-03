use aoc_common::util;

fn parse_table_row(line: &str, prefix: &str) -> util::lexer::Result<Vec<u64>> {
    let mut lexer = util::Lexer::of(line);
    lexer.literal(prefix)?;
    std::iter::from_fn(|| {
        lexer.end().err()?;
        let _ = lexer.whitespace();
        Some(lexer.unsigned_number())
    })
    .collect()
}

const fn get_distance(hold_time: u64, total_time: u64) -> u64 {
    let speed = hold_time;
    let run_time = total_time - hold_time;
    speed * run_time
}

fn count_winning_ways(total_time: u64, distance_to_beat: u64) -> u64 {
    // running distance reaches its maximum at total_time / 2, so check if distance is actually beatable at that point before bisecting
    if get_distance(total_time / 2, total_time) <= distance_to_beat {
        return 0;
    }

    let first_winning_time = util::bisect(0, total_time / 2, |&time| get_distance(time, total_time) > distance_to_beat);

    total_time - 2 * first_winning_time + 1
}

fn merge_decimal(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .fold(0, |acc, &num| acc * 10u64.pow(num.checked_ilog10().unwrap_or(0) + 1) + num)
}

pub fn get_answer(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(u64, u64)> {
    let times = {
        let line = lines.next().expect("Line with times expected");
        parse_table_row(&line, "Time:")?
    };

    let distances = {
        let line = lines.next().expect("Line with distances expected");
        parse_table_row(&line, "Distance:")?
    };

    let total_ways = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| count_winning_ways(time, distance))
        .product();

    let merged_time = merge_decimal(&times);
    let merged_distance = merge_decimal(&distances);
    let total_ways_merged = count_winning_ways(merged_time, merged_distance);

    Ok((total_ways, total_ways_merged))
}
