use aoc_common::util;

enum Transition {
    Repeat,
    Next,
    Drop,
}

struct State {
    if_operational: Transition,
    if_damaged: Transition,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn parse_spring(ch: char) -> Spring {
    match ch {
        '.' => Spring::Operational,
        '#' => Spring::Damaged,
        '?' => Spring::Unknown,
        _ => panic!("Unknown char: {}", ch),
    }
}

fn create_state_chain(damaged_groups: Vec<usize>) -> Vec<State> {
    let mut chain = Vec::new();

    for group in damaged_groups {
        // Drain all operational springs
        chain.push(State {
            if_operational: Transition::Repeat,
            if_damaged: Transition::Next,
        });

        // Take exactly `group` count of damaged springs
        for _ in 1..group {
            chain.push(State {
                if_operational: Transition::Drop,
                if_damaged: Transition::Next,
            });
        }

        // Once damaged springs are taken, make sure that no more damaged springs are taken
        // before looping to the next group
        chain.push(State {
            if_operational: Transition::Next,
            if_damaged: Transition::Drop,
        });
    }

    // There's nowhere to move in last state, so it should drain operational springs forever
    chain.last_mut().unwrap().if_operational = Transition::Repeat;

    chain
}

fn advance(spring: Spring, current_states: Vec<usize>, state_chain: &[State]) -> Vec<usize> {
    let mut new_states = vec![0; current_states.len()];

    for (state_idx, state_count) in current_states.into_iter().enumerate() {
        if spring == Spring::Operational || spring == Spring::Unknown {
            match state_chain[state_idx].if_operational {
                Transition::Repeat => new_states[state_idx] += state_count,
                Transition::Next => new_states[state_idx + 1] += state_count,
                Transition::Drop => {}
            }
        }

        if spring == Spring::Damaged || spring == Spring::Unknown {
            match state_chain[state_idx].if_damaged {
                Transition::Repeat => new_states[state_idx] += state_count,
                Transition::Next => new_states[state_idx + 1] += state_count,
                Transition::Drop => {}
            }
        }
    }

    new_states
}

pub fn count_possible_states(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut count_sum = 0;

    for line in lines {
        let mut lexer = util::Lexer::of(&line);

        let springs = {
            let springs_str = lexer.take_while(|ch| ['.', '#', '?'].contains(&ch))?;

            let mut springs: Vec<_> = springs_str.chars().map(parse_spring).collect();
            let spring_len = springs.len();

            // Repeat 5 times
            for _ in 1..5 {
                springs.push(Spring::Unknown);
                springs.extend_from_within(..spring_len);
            }
            springs
        };

        lexer.whitespace()?;

        let groups = {
            let mut groups: Vec<usize> = vec![];
            groups.push(lexer.unsigned_number()?);

            while lexer.end().is_err() {
                lexer.literal(",")?;
                groups.push(lexer.unsigned_number()?);
            }

            // Repeat 5 times
            let group_len = groups.len();
            for _ in 1..5 {
                groups.extend_from_within(..group_len);
            }

            groups
        };

        let state_chain = create_state_chain(groups);

        let mut current_states = vec![0; state_chain.len()];
        current_states[0] = 1;

        for spring in springs {
            current_states = advance(spring, current_states, &state_chain);
            // println!("{:?}", current_states);
        }

        count_sum += current_states.last().unwrap();
    }

    Ok((0, count_sum))
}
