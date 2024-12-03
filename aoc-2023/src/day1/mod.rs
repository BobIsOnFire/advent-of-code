use aoc_common::util;

fn parse_digit_as_text(lexer: &mut util::Lexer) -> Option<u8> {
    let literals = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for (idx, literal) in literals.into_iter().enumerate() {
        if lexer.literal(literal).is_ok() {
            return Some(idx as u8);
        }
    }

    None
}

fn parse_digit_char(lexer: &mut util::Lexer) -> Option<u8> {
    let mut lexer = lexer.clone();
    let ch = lexer.symbol().ok()?;
    if !ch.is_ascii_digit() {
        return None;
    }

    Some(ch as u8 - b'0')
}

pub fn calibrate(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut calibration = 0;
    let mut calibration_with_text = 0;

    for line in lines {
        let mut first = None;
        let mut last = None;

        let mut first_with_text = None;
        let mut last_with_text = None;

        for i in 0..line.len() {
            let mut lexer = util::Lexer::of(&line[i..]);
            if let Some(ch) = parse_digit_char(&mut lexer) {
                first.get_or_insert(ch);
                last = Some(ch);

                first_with_text.get_or_insert(ch);
                last_with_text = Some(ch);
            } else if let Some(ch) = parse_digit_as_text(&mut lexer) {
                first_with_text.get_or_insert(ch);
                last_with_text = Some(ch);
            }
        }

        let first = first.unwrap_or_else(|| panic!("{line}: at least one digit is expected"));
        let last = last.unwrap_or(first);
        calibration += (first * 10 + last) as usize;

        let first_with_text = first_with_text.unwrap_or_else(|| panic!("{line}: at least one digit or digit string is expected"));
        let last_with_text = last_with_text.unwrap_or(first_with_text);
        calibration_with_text += (first_with_text * 10 + last_with_text) as usize;
    }
    Ok((calibration, calibration_with_text))
}
