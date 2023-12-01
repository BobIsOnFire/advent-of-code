use crate::util;

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(i32, usize)> {
    let mut data = Vec::new();

    for line in lines {
        let mut lexer = util::Lexer::of(&line);
        let num: i32 = lexer.number()?;
        data.push(num);
        lexer.end()?;
    }

    Ok((0, 0))
}
