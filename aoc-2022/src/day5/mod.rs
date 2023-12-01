mod errors;
use errors::CrateInputError::{self, *};

mod parser;

use aoc_common::util::{self, lexer::Lexer, ArrayStack};

fn get_initial_stacks<const N: usize>(
    lines: &mut impl Iterator<Item = String>,
) -> Result<[ArrayStack<char, 56>; N], CrateInputError> {
    let mut stacks = [(); N].map(|_| ArrayStack::new());

    for line in lines.take_while(|l| parser::parse_stack_separator::<N>(l).is_err()) {
        for (stack, opt) in parser::parse_stack_level::<N>(&line)?
            .into_iter()
            .enumerate()
        {
            if opt.is_none() && !stacks[stack].is_empty() {
                return Err(EmptySpaceUnderCrate {
                    line,
                    stack_num: stack + 1,
                });
            }

            if let Some(ch) = opt {
                stacks[stack].push(ch);
            }
        }
    }

    // Iterator over input lines is putting elements from top to bottom, i.e.
    // last element is the one on the very bottom. Need to reverse this shit
    stacks.iter_mut().for_each(|s| s.reverse());

    Ok(stacks)
}

fn get_pair_mut<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    let (first, second) = (Ord::min(i, j), Ord::max(i, j));

    // Can panic if (i == j) or one of them is >= slice.len()
    let (x, y) = slice.split_at_mut(second);
    let first_ref = &mut x[first];
    let second_ref = &mut y[0];

    if i < j {
        (first_ref, second_ref)
    } else {
        (second_ref, first_ref)
    }
}

fn move_elements<T, const S: usize>(
    from: &mut ArrayStack<T, S>,
    to: &mut ArrayStack<T, S>,
    count: usize,
) -> Option<()> {
    for _ in 0..count {
        to.push(from.pop()?);
    }
    Some(())
}

fn collect_stack_tops<const N: usize>(stacks: [ArrayStack<char, 56>; N]) -> String {
    stacks
        .into_iter()
        .map(|s| s.top().copied().unwrap_or(' '))
        .collect()
}

pub fn reorder_stacks<const N: usize>(
    mut lines: impl Iterator<Item = String>,
) -> util::GenericResult<(String, String)> {
    let stacks = get_initial_stacks::<N>(lines.by_ref())?;

    let mut stacks_by_one = stacks;
    let mut stacks_multiple = stacks_by_one.clone();

    let mut aux_stack = ArrayStack::<char, 56>::new();

    // Skip empty line
    Lexer::of(&lines.next().ok_or(NoEmptyLineAfterSeparator)?).end()?;

    for line in lines {
        let op = parser::parse_stack_operation(&line)?;

        if op.from == op.to {
            return Err(IdenticalStackNumbers { line }.into());
        }

        if op.from > N || op.to > N {
            return Err(StackNumbersTooBig { line }.into());
        }

        {
            let (from, to) = get_pair_mut(&mut stacks_by_one, op.from - 1, op.to - 1);
            if move_elements(from, to, op.count).is_none() {
                return Err(NotEnoughCrates { line }.into());
            }
        }

        {
            let (from, to) = get_pair_mut(&mut stacks_multiple, op.from - 1, op.to - 1);
            if move_elements(from, &mut aux_stack, op.count).is_none() {
                return Err(NotEnoughCrates { line }.into());
            }
            move_elements(&mut aux_stack, to, op.count).unwrap();
        }
    }

    Ok((
        collect_stack_tops(stacks_by_one),
        collect_stack_tops(stacks_multiple),
    ))
}
