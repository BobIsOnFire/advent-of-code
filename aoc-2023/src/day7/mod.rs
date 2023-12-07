use std::cmp::Ordering;

use aoc_common::util;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Card(u8);

impl Card {
    fn from_char_and_joker(ch: char, joker: Option<char>) -> Result<Self, String> {
        if joker.is_some_and(|j| j == ch) {
            return Ok(Self(1));
        }

        match ch {
            '2'..='9' => Ok(Self(ch as u8 - b'0')),
            'T' => Ok(Self(10)),
            'J' => Ok(Self(11)),
            'Q' => Ok(Self(12)),
            'K' => Ok(Self(13)),
            'A' => Ok(Self(14)),
            _ => Err(format!("Unknown card type: {}", ch)),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Combination {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn get_combination(cards: &[Card; 5], joker: Option<char>) -> Combination {
    let mut card_value_count = [0usize; 15];
    for &card in cards {
        card_value_count[card.0 as usize] += 1;
    }

    let joker_count = {
        if let Some(ch) = joker {
            let joker_card = Card::from_char_and_joker(ch, Some(ch)).unwrap();
            let joker_count = card_value_count[joker_card.0 as usize];
            card_value_count[joker_card.0 as usize] = 0;
            joker_count
        } else {
            0
        }
    };

    let mut groups = card_value_count.into_iter().filter(|&count| count > 0).collect::<Vec<_>>();
    groups.sort();
    if let Some(last) = groups.last_mut() {
        // The best way to use jokers is to make them all same as the current biggest group, making it even bigger
        *last += joker_count;
    } else {
        // All five cards are jokers
        groups.push(5);
    }

    match groups[..] {
        [5] => Combination::FiveOfKind,
        [1, 4] => Combination::FourOfKind,
        [2, 3] => Combination::FullHouse,
        [1, 1, 3] => Combination::ThreeOfKind,
        [1, 2, 2] => Combination::TwoPairs,
        [1, 1, 1, 2] => Combination::OnePair,
        _ => Combination::HighCard,
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    combination: Combination,
    bid: u64,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match Combination::cmp(&self.combination, &other.combination) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_hand(line: &str, joker: Option<char>) -> util::GenericResult<Hand> {
    let mut lexer = util::Lexer::of(line);
    let hand = lexer.take(5)?;
    lexer.whitespace()?;
    let bid = lexer.unsigned_number()?;
    lexer.end()?;

    let cards = hand
        .chars()
        .map(|ch| Card::from_char_and_joker(ch, joker))
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .unwrap();

    let combination = get_combination(&cards, joker);

    Ok(Hand { combination, cards, bid })
}

pub fn play_poker(lines: impl Iterator<Item = String>) -> util::GenericResult<(u64, u64)> {
    let mut hands_plain = vec![];
    let mut hands_joker = vec![];

    for line in lines {
        hands_plain.push(parse_hand(&line, None)?);
        hands_joker.push(parse_hand(&line, Some('J'))?);
    }

    hands_plain.sort();
    let winnings_plain = hands_plain.into_iter().enumerate().map(|(idx, hand)| (idx as u64 + 1) * hand.bid).sum();

    hands_joker.sort();
    let winnings_joker = hands_joker.into_iter().enumerate().map(|(idx, hand)| (idx as u64 + 1) * hand.bid).sum();

    Ok((winnings_plain, winnings_joker))
}
