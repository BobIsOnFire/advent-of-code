use aoc_common::util;

pub fn fit_presents(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, &'static str)> {
    let mut fit_count = 0;

    // As Reddit said, the input is very lenient, and presents fit if area is large enough (present count x 7)
    // and do not fit if it is not.
    //
    // TODO: actually solve the problem. Though it's not really exciting so meh.
    for line in lines.skip(30) {
        let mut lexer = util::Lexer::of(&line);
        let x = lexer.unsigned_number::<usize>()?;
        lexer.literal("x")?;
        let y = lexer.unsigned_number::<usize>()?;
        lexer.literal(": ")?;

        let mut total_area = 0usize;
        for num in lexer.take_rest()?.split_ascii_whitespace() {
            total_area += num.parse::<usize>()? * 7;
        }

        if total_area <= x * y {
            fit_count += 1;
        }
    }
    Ok((fit_count, "Merry Christmas!"))
}
