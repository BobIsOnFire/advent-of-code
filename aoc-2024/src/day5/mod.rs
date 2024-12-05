use std::{cmp::Ordering, collections::HashSet, str::FromStr};

use aoc_common::util;

#[derive(PartialEq, Eq, Hash)]
struct OrderingRule(u8, u8);

impl FromStr for OrderingRule {
    type Err = util::lexer::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lexer = util::Lexer::of(s);

        let first = lexer.unsigned_number()?;
        lexer.literal("|")?;
        let second = lexer.unsigned_number()?;
        lexer.end()?;

        Ok(Self(first, second))
    }
}

struct OrderingRules {
    rules: HashSet<OrderingRule>,
}

impl FromIterator<OrderingRule> for OrderingRules {
    fn from_iter<T: IntoIterator<Item = OrderingRule>>(iter: T) -> Self {
        Self { rules: iter.into_iter().collect() }
    }
}

impl OrderingRules {
    fn into_comparator(self) -> impl Fn(&u8, &u8) -> Ordering {
        move |&before, &after| {
            if self.rules.contains(&OrderingRule(before, after)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

pub fn order_updates(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(u64, u64)> {
    let comparator = lines
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|s| s.parse())
        .collect::<Result<OrderingRules, _>>()?
        .into_comparator();

    let mut sum_unchanged = 0;
    let mut sum_changed = 0;

    for line in lines {
        let mut pages = line
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        let middle_idx = pages.len() / 2;

        if pages.is_sorted_by(|a, b| comparator(a, b).is_lt()) {
            sum_unchanged += u64::from(pages[middle_idx]);
        } else {
            // Since we're summarizing middle elements from each line, we're not interested in sorting entire array
            let (_, middle_elem, _) = pages.select_nth_unstable_by(middle_idx, &comparator);
            sum_changed += u64::from(*middle_elem);
        }
    }

    Ok((sum_unchanged, sum_changed))
}
