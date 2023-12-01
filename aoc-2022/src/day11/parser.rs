use aoc_common::util::lexer::{self, Lexer};

pub fn parse_monkey_num(s: &str) -> lexer::Result<usize> {
    let mut lexer = Lexer::of(s);
    lexer.literal("Monkey ")?;
    let num = lexer.unsigned_number()?;
    lexer.literal(":")?;
    lexer.end()?;

    Ok(num)
}

pub fn parse_starting_items(s: &str) -> lexer::Result<Vec<u64>> {
    let mut lexer = Lexer::of(s);
    let mut res = vec![];
    lexer.literal("  Starting items: ")?;
    res.push(lexer.unsigned_number()?);

    while lexer.end().is_err() {
        lexer.literal(", ")?;
        res.push(lexer.unsigned_number()?);
    }

    Ok(res)
}

pub fn parse
