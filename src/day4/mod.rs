use crate::util::{
    self,
    lexer::{self, Lexer},
    NumberRange,
};

#[derive(Default)]
struct AssignmentLine {
    first: (i64, i64),
    second: (i64, i64),
}

fn parse_assignment_line(s: &str) -> lexer::Result<AssignmentLine> {
    let mut lexer = Lexer::of(s);

    let mut line = AssignmentLine::default();

    line.first.0 = lexer.unsigned_number()?;
    lexer.literal("-")?;
    line.first.1 = lexer.unsigned_number()?;
    lexer.literal(",")?;
    line.second.0 = lexer.unsigned_number()?;
    lexer.literal("-")?;
    line.second.1 = lexer.unsigned_number()?;
    lexer.end()?;

    Ok(line)
}

#[derive(Default)]
pub struct Answers {
    pub contains: usize,
    pub overlaps: usize,
}

pub fn count_overlaps(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut answers = Answers::default();

    for line in lines {
        let line: AssignmentLine = parse_assignment_line(&line)?;

        let first = NumberRange::new(line.first.0, line.first.1);
        let second = NumberRange::new(line.second.0, line.second.1);
        let overlap = first & second;

        if overlap == first || overlap == second {
            answers.contains += 1
        }

        if !overlap.is_empty() {
            answers.overlaps += 1
        }
    }

    Ok((answers.contains, answers.overlaps))
}
