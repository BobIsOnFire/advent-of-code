use aoc_common::util;

fn weird_digit_into_decimal(ch: u8) -> i8 {
    match ch {
        b'2' => 2,
        b'1' => 1,
        b'0' => 0,
        b'-' => -1,
        b'=' => -2,
        _ => panic!("Wtf is this digit: {ch}"),
    }
}

fn decimal_digit_into_weird(num: i8) -> u8 {
    match num {
        2 => b'2',
        1 => b'1',
        0 => b'0',
        -1 => b'-',
        -2 => b'=',
        _ => panic!("Digit is out of bounds: {num}"),
    }
}

fn convert_into_decimal(string: &str) -> i64 {
    string.bytes().map(weird_digit_into_decimal).fold(0, |acc, num| acc * 5 + i64::from(num))
}

fn convert_into_weird(mut number: i64) -> String {
    let mut reversed_digits = vec![];

    while number != 0 {
        let positive_digit = number % 5;
        let digit = if positive_digit > 2 { positive_digit - 5 } else { positive_digit };
        reversed_digits.push(digit as i8);

        number = (number - digit) / 5;
    }

    String::from_utf8(reversed_digits.into_iter().rev().map(decimal_digit_into_weird).collect()).expect("Weird digits should all be ASCII")
}

pub fn translate_numbers(lines: impl Iterator<Item = String>) -> util::GenericResult<(String, usize)> {
    let sum = lines.map(|s| convert_into_decimal(&s)).sum();
    let weird_sum = convert_into_weird(sum);

    Ok((weird_sum, 0))
}
