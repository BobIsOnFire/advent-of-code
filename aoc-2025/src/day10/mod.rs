use std::collections::HashMap;

use aoc_common::util;

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut total_presses_lights = 0;
    let mut total_presses_joltages = 0;

    for (idx, line) in lines.enumerate() {
        let mut split = line.split_ascii_whitespace();

        let lights = split.next().ok_or("Lights expected")?;
        let lights = lights[1..(lights.len() - 1)]
            .bytes()
            .rev()
            .fold(0usize, |acc, b| (acc << 1) + (b == b'#') as usize);

        let joltages = split.next_back().ok_or("Joltages expected")?;
        let joltages = joltages[1..(joltages.len() - 1)]
            .split(',')
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()?;

        let mut current_lights = HashMap::new();
        current_lights.insert(0usize, 0usize);

        let mut current_joltages = HashMap::new();
        current_joltages.insert(vec![0usize; joltages.len()], 0usize);

        // eprintln!("{0:0b} ({0})", lights);

        for buttons in split {
            let button_presses = buttons[1..(buttons.len() - 1)]
                .split(',')
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()?;
            let button_press = button_presses.iter().fold(0, |acc, p| acc | (1 << p));

            let prev = current_lights
                .iter()
                .map(|(k, v)| (*k, *v))
                .collect::<Vec<_>>();

            for (light, presses) in prev {
                let next = light ^ button_press;
                current_lights
                    .entry(next)
                    .and_modify(|c| *c = usize::min(*c, presses + 1))
                    .or_insert(presses + 1);
            }

            let prev = current_joltages
                .iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect::<Vec<_>>();

            for (mut jolt, mut presses) in prev {
                'outer: loop {
                    presses += 1;
                    for p in button_presses.iter() {
                        if jolt[*p] >= joltages[*p] {
                            break 'outer;
                        }
                        jolt[*p] += 1;
                    }
                    current_joltages
                        .entry(jolt.clone())
                        .and_modify(|c| *c = usize::min(*c, presses))
                        .or_insert(presses);
                }
            }

            // eprintln!("  {0:0b} ({0}) {1:?}", button_press, current_lights);
            // eprintln!("  {0:0b} ({0}) {1:?}", button_press, current_joltages);
        }

        total_presses_lights += current_lights
            .get(&lights)
            .ok_or("Configuration unreachable")?;
        total_presses_joltages += current_joltages
            .get(&joltages)
            .ok_or("Joltage configuration unreachable")?;
        eprintln!("{} done", idx)
    }
    Ok((total_presses_lights, total_presses_joltages))
}
