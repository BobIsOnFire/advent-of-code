use std::fmt::Debug;

use aoc_common::util::{self, BitSet, iter::IteratorExtended};

const fn get_priority(ch: char) -> u64 {
    match ch {
        'a'..='z' => (ch as u64) - ('a' as u64) + 1,
        'A'..='Z' => (ch as u64) - ('A' as u64) + 27,
        _ => 0,
    }
}

fn bitset_from_contents(contents: impl AsRef<str>) -> BitSet {
    contents.as_ref().chars().map(get_priority).collect()
}

fn find_misplaced<const N: usize>(
    contents: &[impl AsRef<str> + Debug; N],
) -> util::GenericResult<u64> {
    let mut set = bitset_from_contents(
        contents
            .first()
            .expect("At least one content string expected"),
    );

    for cont in &contents[1..] {
        set &= bitset_from_contents(cont);
    }

    if set.len() == 1 {
        Ok(set.iter().next().unwrap())
    } else {
        Err(format!(
            "Expected exactly 1 mismatched item, found {}:\n{:#?}",
            set.len(),
            contents
        )
        .into())
    }
}

pub struct Misplacings {
    pub compartments: u64,
    pub groups: u64,
}

pub fn get_misplacings<const N: usize>(
    iter: impl Iterator<Item = String>,
) -> util::GenericResult<(u64, u64)> {
    let mut answers = Misplacings { compartments: 0, groups: 0 };

    for group in iter.groups::<N>() {
        answers.groups += find_misplaced(&group)?;

        for line in group {
            let contents = line.split_at(line.len() / 2).into();
            answers.compartments += find_misplaced(&contents)?;
        }
    }

    Ok((answers.compartments, answers.groups))
}
