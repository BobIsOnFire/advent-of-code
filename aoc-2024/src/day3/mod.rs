use aoc_common::util;

fn parse_number_tuple(lexer: &mut util::Lexer) -> util::lexer::Result<(u64, u64)> {
    lexer.literal("(")?;
    let first = lexer.unsigned_number()?;
    lexer.literal(",")?;
    let second = lexer.unsigned_number()?;
    lexer.literal(")")?;

    Ok((first, second))
}

fn get_all_multiplies(line: &str) -> u64 {
    let mut result = 0;
    let mut lexer = util::Lexer::of(line);

    while lexer.before_literal("mul").is_ok() {
        let Ok((first, second)) = parse_number_tuple(&mut lexer) else {
            continue;
        };
        result += first * second;
    }

    result
}

pub fn parse_corrupted_data(
    lines: impl Iterator<Item = String>,
) -> util::GenericResult<(u64, u64)> {
    let input = lines.collect::<String>();

    // Lexer that just considers "mul" commands
    let result = get_all_multiplies(&input);

    // Lexer that also considers "do" and "don't" commands
    let mut result_with_conditions = 0;
    let mut lexer = util::Lexer::of(&input);

    while let Ok(block) = lexer
        .before_literal("don't()")
        .or_else(|_| lexer.take_rest())
    {
        result_with_conditions += get_all_multiplies(block);
        // Skip everything until next "do", exit loop if it does not exist
        if lexer.before_literal("do()").is_err() {
            break;
        }
    }

    Ok((result, result_with_conditions))
}
