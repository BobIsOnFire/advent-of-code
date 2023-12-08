use std::collections::HashMap;

use aoc_common::util;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct Node {
    left_id: usize,
    right_id: usize,
}

struct NodeMap {
    name_to_node: HashMap<String, usize>,
    nodes: Vec<Node>,
}

impl NodeMap {
    fn new() -> Self {
        Self {
            name_to_node: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    fn get_or_create_node(&mut self, name: String) -> usize {
        if let Some(id) = self.name_to_node.get(&name) {
            *id
        } else {
            self.nodes.push(Node { left_id: 0, right_id: 0 });
            self.name_to_node.insert(name, self.nodes.len() - 1);
            self.nodes.len() - 1
        }
    }

    fn add_node(&mut self, name: String, left: String, right: String) {
        let node_id = self.get_or_create_node(name);
        let left_id = self.get_or_create_node(left);
        let right_id = self.get_or_create_node(right);

        self.nodes[node_id].left_id = left_id;
        self.nodes[node_id].right_id = right_id;
    }

    fn get_id_by_name(&self, name: &str) -> Option<usize> {
        self.name_to_node.get(name).copied()
    }

    fn get_node_by_id(&self, id: usize) -> Option<&Node> {
        self.nodes.get(id)
    }
}

fn parse_direction(ch: char) -> Result<Direction, String> {
    match ch {
        'L' => Ok(Direction::Left),
        'R' => Ok(Direction::Right),
        ch => Err(format!("Invalid direction char: {}", ch)),
    }
}

fn get_node_period(mut node: usize, move_mapping: &[usize]) -> usize {
    let mut visited_nodes: HashMap<usize, usize> = HashMap::from([(node, 0)]);
    let mut current_step = 0;

    loop {
        current_step += 1;
        node = move_mapping[node];

        if let Some(&visit) = visited_nodes.get(&node) {
            break current_step - visit;
        } else {
            visited_nodes.insert(node, current_step);
        }
    }
}

fn get_lcm(num1: usize, num2: usize) -> usize {
    let gcd = {
        let shift = (num1 | num2).trailing_zeros();
        let mut num1 = num1 >> num1.trailing_zeros();
        let mut num2 = num2 >> num2.trailing_zeros();

        while num1 != num2 {
            if num1 > num2 {
                num1 -= num2;
                num1 >>= num1.trailing_zeros();
            } else {
                num2 -= num1;
                num2 >>= num2.trailing_zeros();
            }
        }

        num1 << shift
    };

    (num1 * num2) / gcd
}

pub fn count_steps(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut map = NodeMap::new();

    let directions = lines
        .next()
        .expect("Input ended too soon")
        .chars()
        .map(parse_direction)
        .collect::<Result<Vec<_>, _>>()?;

    lines.next().expect("Input ended too soon");

    for line in lines {
        let mut lexer = util::Lexer::of(&line);

        let name = lexer.take_while(|ch| ch.is_ascii_alphanumeric())?.to_owned();
        lexer.literal(" = (")?;
        let left = lexer.take_while(|ch| ch.is_ascii_alphanumeric())?.to_owned();
        lexer.literal(", ")?;
        let right = lexer.take_while(|ch| ch.is_ascii_alphanumeric())?.to_owned();
        lexer.literal(")")?;
        lexer.end()?;

        map.add_node(name, left, right);
    }

    /*
     NOTE: There are a couple assumptions about the input:
     - Each starting node can reach exactly one ending node
     - The only time when each ending node is reached is in the end of "directions" loop, which is why
       we can make mapping between loop start and end and completely ignore turning logic afterwards.
       Further on, 'step' means the position of the node after the whole "directions" loop, not just
       single turn.
     - Starting nodes move over a set of other nodes in the same way -- it reaches a unique node after
       first step, after which it goes into the loop and returns to the same node which it got in the
       first step (i.e. it never reaches starting node again)
     - Each starting node reaches its ending node one step before looping over to the first node, which
       means that ending node is visited at steps P, 2P, 3P, ... where P is the period of traversing
       the loop (length of the loop).
     Based on that, number of steps for all starting points to reach all ending points is just the LCM
     of the periods of starting nodes. There should be a way to generalize solution and avoid assumptions
     above, but I just didn't bother.
    */

    let move_mapping: Vec<_> = map
        .nodes
        .iter()
        .map(|mut node| {
            let mut next_id = 0;
            for direction in directions.iter() {
                next_id = match direction {
                    Direction::Left => node.left_id,
                    Direction::Right => node.right_id,
                };
                node = map.get_node_by_id(next_id).unwrap();
            }
            next_id
        })
        .collect();

    let steps_count = get_node_period(map.get_id_by_name("AAA").expect("Node AAA was not added into the map"), &move_mapping);

    let all_steps_count = map
        .name_to_node
        .iter()
        .filter_map(|(name, id)| name.ends_with('A').then_some(*id))
        .map(|node| get_node_period(node, &move_mapping))
        .fold(1, get_lcm);

    Ok((steps_count * directions.len(), all_steps_count * directions.len()))
}
