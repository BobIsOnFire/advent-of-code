mod data;
use data::Match;

use aoc_common::util::{self, lexer, Lexer};

fn parse_match_line(s: &str) -> lexer::Result<(char, char)> {
    let mut lexer = Lexer::of(s);
    let mut res = ('?', '?');

    lexer
        .chain()
        .symbol(&mut res.0)?
        .whitespace()?
        .symbol(&mut res.1)?
        .end()?;

    Ok(res)
}

#[derive(Default)]
pub struct Scores {
    pub two_turns: usize,
    pub outcome: usize,
}

pub fn get_total_scores(iter: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut answers = Scores::default();

    for s in iter {
        let (first, second) = parse_match_line(&s)?;

        let two_turns_match = Match::from_turns(&first.try_into()?, second.try_into()?);
        answers.two_turns += two_turns_match.score();

        let turn_and_outcome_match =
            Match::from_opponent_and_outcome(&first.try_into()?, second.try_into()?);
        answers.outcome += turn_and_outcome_match.score();
    }

    Ok((answers.two_turns, answers.outcome))
}
