use aoc_common::util;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Lock {
    pins: [usize; 5],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    pins: [usize; 5],
}

fn are_overlapping(lock: &Lock, key: &Key) -> bool {
    for i in 0..5 {
        if lock.pins[i] + key.pins[i] > 5 {
            return true;
        }
    }
    false
}

pub fn get_answer(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, String)> {
    let mut locks = vec![];
    let mut keys = vec![];

    while let Some(first) = lines.next() {
        let is_lock = first == "#####";
        let mut pin_lines = [(); 5].map(|()| lines.next().expect("Pin line"));

        let _ = lines.next(); // Last line in the pattern, safe to ignore
        let _ = lines.next(); // Empty line delimiter between patterns

        let mut pins = [0usize; 5];

        if !is_lock {
            pin_lines.reverse();
        }

        for line in pin_lines {
            for (pin, ch) in line.chars().enumerate() {
                if ch == '#' {
                    pins[pin] += 1;
                }
            }
        }

        if is_lock {
            locks.push(Lock { pins });
        } else {
            keys.push(Key { pins });
        }
    }

    let fit_count = locks
        .iter()
        .flat_map(|lock| keys.iter().map(move |key| (lock, key)))
        .filter(|(lock, key)| !are_overlapping(lock, key))
        .count();

    Ok((fit_count, "Merry Christmas!".to_string()))
}
