use aoc_common::util;

fn is_invalid_halves(num: usize) -> bool {
    let num_digits = num.ilog10() + 1;
    if num_digits.is_multiple_of(2) {
        let exp = 10usize.pow(num_digits / 2);
        let rep = num % exp;
        let invalid = rep * exp + rep;
        if num == invalid {
            return true;
        }
    }

    false
}

fn is_invalid(num: usize) -> bool {
    let num_digits = num.ilog10() + 1;
    for n in 1..=num_digits / 2 {
        if num_digits.is_multiple_of(n) {
            let exp = 10usize.pow(n);
            let rep = num % exp;
            let invalid = (0..num_digits / n).fold(0, |acc, _| acc * exp + rep);
            if num == invalid {
                return true;
            }
        }
    }

    false
}

pub fn find_invalid_numbers(
    mut lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let mut invalid_sum_halves = 0;
    let mut invalid_sum = 0;

    let line = lines.next().ok_or("There should be exactly one line")?;
    for range in line.split(',') {
        let (from, to) = range.split_once('-').ok_or("Invalid range format")?;
        let (from, to) = (from.parse::<usize>()?, to.parse::<usize>()?);

        invalid_sum_halves += (from..=to)
            .filter(|num| is_invalid_halves(*num))
            .sum::<usize>();
        invalid_sum += (from..=to).filter(|num| is_invalid(*num)).sum::<usize>();
    }

    Ok((invalid_sum_halves, invalid_sum))
}
