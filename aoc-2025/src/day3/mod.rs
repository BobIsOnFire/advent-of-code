use aoc_common::util;

fn find_max_joltage(line: &[u8], count: usize) -> usize {
    let mut joltage_twelve = 0usize;
    let mut start_idx = 0;
    for step in 0..count {
        let mut max = line[start_idx];
        let mut max_i = start_idx;

        for (ch, i) in line[start_idx..line.len() - (count - step - 1)]
            .iter()
            .copied()
            .zip(start_idx..)
        {
            if ch > max {
                (max, max_i) = (ch, i);
            }
        }

        joltage_twelve = joltage_twelve * 10 + (max - b'0') as usize;
        start_idx = max_i + 1;
    }

    joltage_twelve
}

pub fn get_total_joltage(
    lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let mut total_twelve: usize = 0;
    let mut total_two: usize = 0;
    for line in lines {
        let line = line.as_bytes();

        total_two += find_max_joltage(line, 2);
        total_twelve += find_max_joltage(line, 12);
    }
    Ok((total_two, total_twelve))
}
