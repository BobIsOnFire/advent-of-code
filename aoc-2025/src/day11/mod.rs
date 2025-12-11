use std::collections::{HashMap, VecDeque};

use aoc_common::util;

struct DeviceMap {
    name_to_id: HashMap<String, usize>,
    outputs: Vec<Vec<usize>>,
    topo_order: Vec<usize>,
}

impl DeviceMap {
    fn new() -> Self {
        Self {
            name_to_id: HashMap::new(),
            outputs: vec![],
            topo_order: vec![],
        }
    }

    fn get_or_insert(&mut self, name: &str) -> usize {
        if let Some(id) = self.name_to_id.get(name) {
            return *id;
        }

        self.outputs.push(vec![]);
        self.name_to_id
            .insert(name.to_string(), self.outputs.len() - 1);

        self.outputs.len() - 1
    }

    fn add_output(&mut self, from: &str, to: &str) {
        let from_id = self.get_or_insert(from);
        let to_id = self.get_or_insert(to);

        self.outputs[from_id].push(to_id);
    }

    fn build_topo_order(&mut self) {
        let mut indegrees = vec![0usize; self.outputs.len()];

        for outputs in self.outputs.iter() {
            for out in outputs {
                indegrees[*out] += 1;
            }
        }

        let mut queue = VecDeque::new();
        for (idx, indegree) in indegrees.iter().enumerate() {
            if *indegree == 0 {
                queue.push_back(idx);
            }
        }

        while let Some(next) = queue.pop_front() {
            self.topo_order.push(next);
            for &out in &self.outputs[next] {
                indegrees[out] -= 1;
                if indegrees[out] == 0 {
                    queue.push_back(out);
                }
            }
        }
    }

    fn count_paths(&self, from: &str, to: &str) -> usize {
        let from = *self.name_to_id.get(from).unwrap();
        let to = *self.name_to_id.get(to).unwrap();

        let mut path_count = vec![0; self.outputs.len()];

        path_count[from] = 1;
        for &idx in &self.topo_order {
            if path_count[idx] == 0 {
                continue;
            }

            for &out in &self.outputs[idx] {
                path_count[out] += path_count[idx];
            }

            if idx == to {
                break;
            }
        }

        path_count[to]
    }
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut device_map = DeviceMap::new();

    for line in lines {
        let mut lexer = util::Lexer::of(&line);
        let from = lexer.before_literal(": ")?;
        while let Ok(to) = lexer.before_literal(" ") {
            device_map.add_output(from, to);
        }
        let last_to = lexer.take_rest()?;
        device_map.add_output(from, last_to);
    }
    device_map.build_topo_order();

    let path_count = device_map.count_paths("you", "out");

    let fft_to_dac_count = device_map.count_paths("fft", "dac");
    let full_path_count = if fft_to_dac_count != 0 {
        device_map.count_paths("svr", "fft")
            * fft_to_dac_count
            * device_map.count_paths("dac", "out")
    } else {
        device_map.count_paths("svr", "dac")
            * device_map.count_paths("dac", "fft")
            * device_map.count_paths("fft", "out")
    };

    Ok((path_count, full_path_count))
}
