use aoc_common::util::{self, Lexer, lexer};

#[derive(Debug)]
enum Outcome {
    Correct,
    Incorrect,
    Undetermined,
}

impl Outcome {
    const fn invert(self) -> Self {
        match self {
            Self::Correct => Self::Incorrect,
            Self::Incorrect => Self::Correct,
            Self::Undetermined => Self::Undetermined,
        }
    }

    const fn is_correct(&self) -> bool {
        matches!(self, Self::Correct)
    }

    const fn is_determined(&self) -> bool {
        !matches!(self, Self::Undetermined)
    }
}

fn exhaust_array(array: &mut Lexer) -> lexer::Result<()> {
    let mut open_count = 1;
    let mut close_count = 0;

    while let Ok(txt) = array.before_literal("]") {
        open_count += txt.chars().filter(|&ch| ch == '[').count();
        close_count += 1;

        if open_count == close_count {
            return Ok(());
        }
    }

    // Cannot find the end of array, drop error
    Err(array
        .before_literal("]")
        .expect_err("Closing brackets should have been exhausted"))
}

fn compare_integers(left: i64, right: i64) -> Outcome {
    match Ord::cmp(&left, &right) {
        std::cmp::Ordering::Less => Outcome::Correct,
        std::cmp::Ordering::Equal => Outcome::Undetermined,
        std::cmp::Ordering::Greater => Outcome::Incorrect,
    }
}

fn compare_int_and_array(int: i64, array: &mut Lexer) -> lexer::Result<Outcome> {
    if array.literal("]").is_ok() {
        eprintln!("Comparing integer to array: array is empty");
        return Ok(Outcome::Incorrect);
    }

    let outcome = if let Ok(num) = array.number() {
        compare_integers(int, num)
    } else {
        array.literal("[")?;
        compare_int_and_array(int, array)?
    };

    if array.literal("]").is_err() {
        exhaust_array(array)?;
        return Ok(Outcome::Correct);
    }

    Ok(outcome)
}

fn compare_arrays(left: &mut Lexer, right: &mut Lexer) -> lexer::Result<Outcome> {
    // Early return if one of the arrays is empty
    match (left.literal("]"), right.literal("]")) {
        (Ok(()), Ok(())) => {
            eprintln!("Both arrays are empty");
            return Ok(Outcome::Undetermined);
        }
        (Ok(()), Err(_)) => {
            exhaust_array(right)?;
            eprintln!("Left array is empty");
            return Ok(Outcome::Correct);
        }
        (Err(_), Ok(())) => {
            exhaust_array(right)?;
            eprintln!("Right array is empty");
            return Ok(Outcome::Incorrect);
        }
        _ => (),
    }

    loop {
        eprintln!("Comparing two elements");
        let outcome = compare_data(left, right)?;

        if outcome.is_determined() {
            return Ok(outcome);
        }

        // Are there still elements to compare in both arrays?
        match (left.literal(","), right.literal(",")) {
            (Ok(()), Ok(())) => {}
            (Ok(()), Err(_)) => {
                exhaust_array(left)?;
                exhaust_array(right)?;
                eprintln!("No more elements in the right array");
                return Ok(Outcome::Incorrect);
            }
            (Err(_), Ok(())) => {
                exhaust_array(left)?;
                exhaust_array(right)?;
                eprintln!("No more elements in the left array");
                return Ok(Outcome::Correct);
            }
            _ => {
                exhaust_array(left)?;
                exhaust_array(right)?;
                eprintln!("Both arrays are exhausted");
                return Ok(Outcome::Undetermined);
            }
        }
    }
}

fn compare_data(left: &mut Lexer, right: &mut Lexer) -> lexer::Result<Outcome> {
    match (left.number(), right.number()) {
        (Ok(num_left), Ok(num_right)) => Ok(compare_integers(num_left, num_right)),
        (Ok(num_left), _) => {
            right.literal("[")?;
            eprintln!("Left is an integer, right is an array");
            compare_int_and_array(num_left, right)
        }
        (_, Ok(num_right)) => {
            left.literal("[")?;
            eprintln!("Left is an array, right is an integer");
            compare_int_and_array(num_right, left).map(Outcome::invert)
        }
        _ => {
            left.literal("[")?;
            right.literal("[")?;
            eprintln!("Both elements are arrays");
            compare_arrays(left, right)
        }
    }
}

pub fn determine_order(
    mut lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let mut sum = 0;
    let mut idx = 0;

    loop {
        let first = lines.next().ok_or("Missing line of data")?;
        let second = lines.next().ok_or("Missing line of data")?;
        idx += 1;

        if compare_data(&mut Lexer::of(&first), &mut Lexer::of(&second))?.is_correct() {
            sum += idx;
        }

        eprintln!();

        if lines.next().is_none() {
            break;
        }
    }

    Ok((sum, 0))
}
