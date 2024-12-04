use std::collections::{HashMap, HashSet};

use aoc_common::util;

fn do_count_connected(
    graph: &HashMap<String, HashSet<String>>,
    current: &str,
    visited: &mut HashSet<String>,
) -> usize {
    if visited.contains(current) {
        return 0;
    }
    visited.insert(current.to_string());

    let count = graph
        .get(current)
        .unwrap()
        .iter()
        .map(|next| do_count_connected(graph, next, visited))
        .sum::<usize>();

    count + 1
}

fn count_connected_nodes(graph: &HashMap<String, HashSet<String>>, node: &str) -> usize {
    let mut visited = HashSet::new();
    do_count_connected(graph, node, &mut visited)
}

pub fn disconnect_nodes(
    lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for line in lines {
        let mut lexer = util::Lexer::of(&line);
        let left = lexer.take_while(|ch| ch.is_ascii_alphabetic())?;
        lexer.literal(":")?;
        while lexer.end().is_err() {
            lexer.whitespace()?;
            let right = lexer.take_while(|ch| ch.is_ascii_alphabetic())?;

            // println!("    {} -- {}", left, right);
            graph
                .entry(left.to_string())
                .or_default()
                .insert(right.to_string());
            graph
                .entry(right.to_string())
                .or_default()
                .insert(left.to_string());
        }
    }

    // Edges to remove. Determined visually by looking at `neato` output 👀
    let edges = [("sdv", "mxv"), ("vbk", "gqr"), ("klj", "scr")];

    for (left, right) in edges {
        graph.get_mut(left).unwrap().remove(right);
        graph.get_mut(right).unwrap().remove(left);
    }

    let connected = count_connected_nodes(&graph, "sdv");
    let disconnected = graph.len() - connected;

    Ok((connected * disconnected, 0))
}
