use aoc_common::util;

pub fn crack_the_safe(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut current = 50i32;

    let mut point_zeroes = 0;
    let mut all_zeroes = 0;

    for line in lines {
        let direction_right = line.starts_with('R');
        let turns = dbg!(line)[1..].parse::<i32>()?;

        let old = current;
        if direction_right {
            current += turns;
            // Count transitions between 99 and 0 (going forward)
            all_zeroes += (current.div_euclid(100) - old.div_euclid(100)) as usize;
        } else {
            current -= turns;
            // Count transitions between 1 and 0 (going backward)
            all_zeroes += ((old - 1).div_euclid(100) - (current - 1).div_euclid(100)) as usize;
        }

        if current % 100 == 0 {
            point_zeroes += 1;
        }

        dbg!(current);
        dbg!(all_zeroes);
    }

    Ok((point_zeroes, all_zeroes))
}
