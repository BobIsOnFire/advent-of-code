use aoc_common::util;

const DELTA: i64 = 10_000_000_000_000;

fn parse_numbers(line: &str, prefix: &str) -> util::lexer::Result<(i64, i64)> {
    let mut lexer = util::Lexer::of(line);
    lexer.literal(prefix)?;
    lexer.literal(": X")?;
    lexer.symbol()?;
    let x = lexer.unsigned_number()?;
    lexer.literal(", Y")?;
    lexer.symbol()?;
    let y = lexer.unsigned_number()?;
    lexer.end()?;

    Ok((x, y))
}

fn solve(a_x: i64, a_y: i64, b_x: i64, b_y: i64, prize_x: i64, prize_y: i64) -> Option<(i64, i64)> {
    // Working equation:
    // a_x * A + b_x * B = prize_x
    // a_y * A + b_y * B = prize_y

    // ===[ Calculate A ]===
    // [ multiply 1st equation by b_y, 2nd equation by b_x ]
    // (a_x * b_y) * A + (b_x * b_y) * B = prize_x * b_y
    // (a_y * b_x) * A + (b_x * b_y) * B = prize_y * b_x

    // [ substract 2nd equation from 1st ]
    // (a_x * b_y - a_y * b_x) * A = prize_x * b_y - prize_y * b_x

    // [ extract value of A ]
    // A = (prize_x * b_y - prize_y * b_x) / (a_x * b_y - a_y * b_x)
    let delim = a_x * b_y - a_y * b_x;
    let result = prize_x * b_y - prize_y * b_x;

    // If `delim` is zero, there's an infinite number of solutions, and we ideally need to find the
    // most optimal solution by minimizing S = 3A + B.
    // There's no such case in my input, so I didn't need to handle it :)
    if delim == 0 {
        unimplemented!("Optimizing solution is missing");
    }

    // If `delim` is non-zero, there's exactly one solution. Since we're counting number of button
    // presses, we should additionally check that solution pair is whole, non-negative numbers.

    if result % delim != 0 {
        return None;
    }

    let a = result / delim;

    if a < 0 {
        return None;
    }

    // ===[ Calculate B ]===
    // a_x * A + b_x * B = prize_x

    // [ extract value of B ]
    // B = (prize_x - a_x * A) / b_x
    let b_result = prize_x - a_x * a;

    // Same checks on whole non-negative for B

    if b_result % b_x != 0 {
        return None;
    }

    let b = b_result / b_x;

    if b < 0 {
        return None;
    }

    Some((a, b))
}

pub fn get_answer(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(i64, i64)> {
    let mut total = 0;
    let mut total_with_delta = 0;

    loop {
        let line_a = lines.next().ok_or("Button A line is missing")?;
        let (a_x, a_y) = parse_numbers(&line_a, "Button A")?;

        let line_b = lines.next().ok_or("Button B line is missing")?;
        let (b_x, b_y) = parse_numbers(&line_b, "Button B")?;

        let line_prize = lines.next().ok_or("Prize line is missing")?;
        let (prize_x, prize_y) = parse_numbers(&line_prize, "Prize")?;

        if let Some((a, b)) = solve(a_x, a_y, b_x, b_y, prize_x, prize_y) {
            total += 3 * a + b;
        }

        if let Some((a, b)) = solve(a_x, a_y, b_x, b_y, prize_x + DELTA, prize_y + DELTA) {
            total_with_delta += 3 * a + b;
        }

        if lines.next().is_none() {
            break;
        }
    }

    Ok((total, total_with_delta))
}
