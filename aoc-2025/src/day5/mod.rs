use std::cmp::Ordering;

use aoc_common::util::{self, NumberRange};

fn sort_and_merge(ranges: &mut Vec<NumberRange>) {
    ranges.sort_unstable_by(NumberRange::started_before);

    let mut merged_ranges: Vec<NumberRange> = Vec::with_capacity(ranges.len());
    merged_ranges.push(NumberRange::Empty);

    for &range in ranges.iter() {
        if range.is_empty() {
            continue;
        }
        let last = merged_ranges.last_mut().unwrap();
        if *last & range == NumberRange::Empty {
            // Two closest ranges don't intersect
            if (*last | range).len() == last.len() + range.len() + 1 {
                // They can still be one after another (e.g. [1,2] and [3,4]), we can merge them
                *last = *last | range;
            } else {
                // No common ground (e.g. [1,2] and [5,6]), create new range
                merged_ranges.push(range);
            }
        } else {
            // Merge new range into previous one
            *last = *last | range;
        }
    }

    *ranges = merged_ranges;
}

fn is_in_range(product: i64, ranges: &[NumberRange]) -> bool {
    // TODO: binary search?
    for r in ranges {
        match r.contains(product) {
            Ordering::Less => return false,
            Ordering::Equal => return true,
            Ordering::Greater => {}
        }
    }

    false
}

pub fn get_answer(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut ranges: Vec<NumberRange> = vec![];

    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let (from, to) = line.split_once('-').ok_or("Range expected")?;
        ranges.push(NumberRange::NonEmpty(from.parse()?, to.parse()?));
    }

    sort_and_merge(&mut ranges);

    let total = ranges.iter().map(|r| r.len() + 1).sum::<usize>();

    let mut count = 0;
    for line in lines {
        let product = line.parse::<i64>()?;
        if is_in_range(product, &ranges) {
            count += 1;
        }
    }

    Ok((count, total))
}
