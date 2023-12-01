use aoc_common::util::{self, iter::ResultIteratorExtended, lexer, Lexer};

fn parse_food_line(s: &str) -> lexer::Result<Option<usize>> {
    let mut lexer = Lexer::of(s);

    let res = lexer.end().map(|_| None);
    if res.is_ok() {
        return res;
    }

    // If `res` is Err (i.e. `s` is not empty), try parsing a calorie number from the line
    let num = lexer.unsigned_number()?;
    lexer.end()?;
    Ok(Some(num))
}

pub fn get_n_highest<const N: usize>(
    lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let mut elf_sums = Vec::new();
    let mut food_lines = lines.map(|s| parse_food_line(&s)).end_on_error();

    loop {
        let sum = food_lines.by_ref().map_while(|num| num).sum();
        // No more elves (or an error occured)
        if sum == 0 {
            break;
        }
        elf_sums.push(sum);
    }

    if let Some(err) = food_lines.into_error() {
        return Err(err.into());
    }

    let len = elf_sums.len();
    assert!(N < len, "N is too big");

    let (_, _, highest) = elf_sums.select_nth_unstable(len - N - 1);
    let mut highest: [usize; N] = highest
        .try_into()
        .expect("This should be guaranteed by select algo");

    highest.sort_unstable();
    Ok((highest[N - 1], highest.iter().sum()))
}
