use aoc_common::util::lexer::{self, Lexer};

#[derive(Default)]
pub struct StackOperation {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

pub fn parse_stack_level<const LEN: usize>(s: &str) -> lexer::Result<[Option<char>; LEN]> {
    let mut lexer = Lexer::of(s);

    let mut res = [None; LEN];

    for (ind, elem) in res.iter_mut().enumerate() {
        if lexer.literal("   ").is_ok() {
            *elem = None;
        } else {
            let mut ch = '?';
            lexer.chain().literal("[")?.symbol(&mut ch)?.literal("]")?;
            *elem = Some(ch);
        }

        if ind != LEN - 1 {
            lexer.literal(" ")?;
        }
    }

    lexer.end()?;

    Ok(res)
}

pub fn parse_stack_separator<const LEN: usize>(s: &str) -> lexer::Result<()> {
    let mut lexer = Lexer::of(s);

    for ind in 1..=LEN {
        lexer.literal(&format!(" {} ", ind))?;

        if ind != LEN {
            lexer.literal(" ")?;
        }
    }
    lexer.end()?;

    Ok(())
}

pub fn parse_stack_operation(s: &str) -> lexer::Result<StackOperation> {
    let mut lexer = Lexer::of(s);
    let mut op = StackOperation::default();

    lexer
        .chain()
        .literal("move ")?
        .unsigned_number(&mut op.count)?
        .literal(" from ")?
        .unsigned_number(&mut op.from)?
        .literal(" to ")?
        .unsigned_number(&mut op.to)?
        .end()?;

    Ok(op)
}
