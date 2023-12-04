use std::collections::{HashMap, HashSet};

use aoc_common::util;

pub fn count_scratchcards(lines: impl Iterator<Item = String>) -> util::GenericResult<(u32, usize)> {
    let mut total_worth = 0;

    let mut card_count: HashMap<usize, usize> = HashMap::new();

    for line in lines {
        let mut lexer = util::Lexer::of(&line);
        lexer.literal("Card")?;
        lexer.take_while(|ch| ch.is_ascii_whitespace())?;

        let id: usize = lexer.unsigned_number()?;
        lexer.literal(":")?;

        let mut winning_set: HashSet<u32> = HashSet::new();
        loop {
            lexer.take_while(|ch| ch.is_ascii_whitespace())?;
            if lexer.literal("|").is_ok() {
                break;
            }
            winning_set.insert(lexer.unsigned_number()?);
        }

        let mut card_set: HashSet<u32> = HashSet::new();
        loop {
            lexer.take_while(|ch| ch.is_ascii_whitespace())?;
            if lexer.end().is_ok() {
                break;
            }
            card_set.insert(lexer.unsigned_number()?);
        }

        let current_count = *card_count.entry(id).or_insert(1);

        let winning_numbers_count = winning_set.intersection(&card_set).count();
        if winning_numbers_count > 0 {
            total_worth += 2u32.pow(winning_numbers_count as u32 - 1);

            for i in (id + 1)..(id + 1 + winning_numbers_count) {
                *card_count.entry(i).or_insert(1) += current_count;
            }
        }
    }

    let total_count = card_count.values().sum();

    Ok((total_worth, total_count))
}
