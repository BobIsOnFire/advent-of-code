use std::{collections::HashMap, mem};

use aoc_common::util;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stripe {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl From<char> for Stripe {
    fn from(value: char) -> Self {
        match value {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            ch => panic!("Expected stripe, got {ch}"),
        }
    }
}

struct PatternTrie {
    nodes: Vec<[Option<usize>; 5]>,
    is_end_node: Vec<bool>,
}

impl PatternTrie {
    fn new() -> Self {
        Self {
            nodes: vec![[None; 5]],
            is_end_node: vec![false],
        }
    }

    fn add_pattern(&mut self, pattern: impl IntoIterator<Item = Stripe>) {
        let mut node = 0;
        for stripe in pattern {
            if let Some(next_node) = self.nodes[node][stripe as usize] {
                node = next_node;
            } else {
                let next_node = self.nodes.len();

                self.nodes.push([None; 5]);
                self.is_end_node.push(false);

                self.nodes[node][stripe as usize] = Some(next_node);
                node = next_node;
            }
        }
        self.is_end_node[node] = true;
    }

    fn check_pattern(&self, pattern: impl IntoIterator<Item = Stripe>) -> usize {
        let mut nodes = HashMap::new();
        nodes.insert(0, 1);

        for stripe in pattern {
            for (node, count) in mem::take(&mut nodes) {
                if let Some(next_node) = self.nodes[node][stripe as usize] {
                    *nodes.entry(next_node).or_default() += count;
                    if self.is_end_node[next_node] {
                        *nodes.entry(0).or_default() += count;
                    }
                }
            }
        }

        nodes
            .into_iter()
            .filter_map(|(node, count)| (node == 0).then_some(count))
            .sum()
    }
}

pub fn get_answer(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut trie = PatternTrie::new();

    let towel_line = lines.next().ok_or("Input ended too early")?;
    towel_line
        .split(", ")
        .map(|t| t.chars().map(Into::into))
        .for_each(|it| trie.add_pattern(it));

    lines.next().ok_or("Input ended too early")?;

    let mut valid_patterns = 0;
    let mut total_ways = 0;

    for line in lines {
        let count = trie.check_pattern(line.chars().map(Into::into));
        if count > 0 {
            valid_patterns += 1;
        }
        total_ways += count;
    }

    Ok((valid_patterns, total_ways))
}
