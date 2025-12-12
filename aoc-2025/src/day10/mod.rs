use std::collections::HashMap;

use aoc_common::util;

fn get_reachable_masks(pattern: usize, button_presses: &[usize]) -> impl Iterator<Item = usize> {
    (0usize..2usize.pow(button_presses.len() as u32)).filter(move |mask| {
        let pressed = button_presses
            .iter()
            .copied()
            .enumerate()
            .filter(|(idx, _press)| (1 << *idx) & *mask != 0)
            .fold(0usize, |acc, (_idx, press)| acc ^ press);
        pressed == pattern
    })
}

struct Solver {
    button_presses: Vec<usize>,
    press_cache: Vec<Vec<i32>>,
    cache: HashMap<Vec<i32>, Option<usize>>,
}

impl Solver {
    fn new(button_presses: Vec<usize>, joltages_len: usize) -> Self {
        let press_cache = (0usize..2usize.pow(button_presses.len() as u32))
            .map(|mask| {
                let mut joltages = vec![0; joltages_len];
                for press in button_presses
                    .iter()
                    .copied()
                    .enumerate()
                    .filter_map(|(idx, press)| (1 << idx & mask != 0).then_some(press))
                {
                    for (idx, joltage) in joltages.iter_mut().enumerate() {
                        if 1 << idx & press != 0 {
                            *joltage += 1;
                        }
                    }
                }
                joltages
            })
            .collect::<Vec<_>>();
        Self {
            button_presses,
            press_cache,
            cache: HashMap::new(),
        }
    }

    fn solve(mut self, joltages: Vec<i32>) -> Option<usize> {
        self.do_solve(&joltages)
    }

    fn do_solve(&mut self, joltages: &[i32]) -> Option<usize> {
        if joltages.iter().any(|&j| j < 0) {
            return None;
        }
        if joltages.iter().all(|&j| j == 0) {
            return Some(0);
        }

        let pattern = joltages
            .iter()
            .rev()
            .fold(0usize, |acc, j| (acc << 1) + (j % 2) as usize);

        let masks = get_reachable_masks(pattern, &self.button_presses).collect::<Vec<_>>();

        masks
            .into_iter()
            .filter_map(|mask| {
                let mut new_joltages = Vec::from(joltages);
                for (idx, update) in self.press_cache[mask].iter().enumerate() {
                    new_joltages[idx] = (new_joltages[idx] - update) / 2;
                }

                let child_res = if self.cache.contains_key(&new_joltages) {
                    self.cache[&new_joltages]
                } else {
                    let res = self.do_solve(&new_joltages);
                    self.cache.insert(new_joltages, res);
                    res
                };
                child_res.map(|r| 2 * r + mask.count_ones() as usize)
            })
            .min()
    }
}

pub fn count_button_presses(
    lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let mut total_presses_lights = 0;
    let mut total_presses_joltages = 0;

    for line in lines {
        let mut split = line.split_ascii_whitespace();

        let lights = split.next().ok_or("Lights expected")?;
        let lights = lights[1..(lights.len() - 1)]
            .bytes()
            .rev()
            .fold(0usize, |acc, b| (acc << 1) + (b == b'#') as usize);

        let joltages = split.next_back().ok_or("Joltages expected")?;
        let joltages = joltages[1..(joltages.len() - 1)]
            .split(',')
            .map(str::parse::<i32>)
            .collect::<Result<Vec<_>, _>>()?;

        let button_presses = split
            .map(|buttons| {
                buttons[1..(buttons.len() - 1)]
                    .split(',')
                    .map(str::parse::<usize>)
                    .try_fold(0usize, |acc, p_result| p_result.map(|p| acc | (1 << p)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        total_presses_lights += get_reachable_masks(lights, &button_presses)
            .map(|mask| mask.count_ones())
            .min()
            .ok_or("Configuration unreachable")? as usize;

        let solver = Solver::new(button_presses, joltages.len());
        total_presses_joltages += solver.solve(joltages).ok_or("Configuration unreachable")?;
    }
    Ok((total_presses_lights, total_presses_joltages))
}
