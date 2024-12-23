use std::collections::{HashMap, HashSet};

use aoc_common::util;

struct Network {
    names: Vec<String>,
    name_to_id: HashMap<String, usize>,
    links: HashSet<[usize; 2]>,
}

impl Network {
    fn new() -> Self {
        Self {
            names: vec![],
            name_to_id: HashMap::new(),
            links: HashSet::new(),
        }
    }

    fn get_or_insert(&mut self, name: &str) -> usize {
        if let Some(id) = self.name_to_id.get(name) {
            return *id;
        }

        self.names.push(name.to_string());
        self.name_to_id
            .insert(name.to_string(), self.names.len() - 1);

        self.names.len() - 1
    }

    fn add_link(&mut self, from: &str, to: &str) {
        let from_id = self.get_or_insert(from);
        let to_id = self.get_or_insert(to);

        let (from_id, to_id) = if from_id < to_id { (from_id, to_id) } else { (to_id, from_id) };

        #[allow(clippy::tuple_array_conversions)]
        self.links.insert([from_id, to_id]);
    }

    fn get_triangles(&self) -> Vec<[usize; 3]> {
        let mut triangles = vec![];

        for &[first, second] in &self.links {
            let third_start = usize::max(first, second) + 1;
            for third in third_start..self.names.len() {
                if self.links.contains(&[first, third]) && self.links.contains(&[second, third]) {
                    triangles.push([first, second, third]);
                }
            }
        }

        triangles
    }

    fn get_bigger_groups<const MAX: usize>(
        &self,
        len: usize,
        groups: &[[usize; MAX]],
    ) -> Vec<[usize; MAX]> {
        assert!(len < MAX);
        let mut next_groups = vec![];

        for group in groups {
            let extra_start = group[len - 1] + 1;
            for extra in extra_start..self.names.len() {
                if group[..len]
                    .iter()
                    .all(|&node| self.links.contains(&[node, extra]))
                {
                    let mut next_group = *group;
                    next_group[len] = extra;

                    next_groups.push(next_group);
                }
            }
        }

        next_groups
    }

    fn find_biggest_group<const MAX: usize>(&self) -> Vec<usize> {
        let triangles = self.get_triangles();

        let mut current = triangles
            .into_iter()
            .map(|tr| {
                let mut array = [0; MAX];
                for (dst, src) in array.iter_mut().zip(tr) {
                    *dst = src;
                }
                array
            })
            .collect::<Vec<_>>();

        for len in 3..MAX {
            let next = self.get_bigger_groups(len, &current);
            if next.is_empty() {
                assert!(current.len() == 1);
                return current[0][..len].to_vec();
            }
            current = next;
        }

        panic!("Array limit ({MAX}) exceeded");
    }
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, String)> {
    let mut network = Network::new();

    for line in lines {
        let mut lexer = util::Lexer::of(&line);

        let from = lexer.before_literal("-")?;
        let to = lexer.take_rest()?;
        lexer.end()?;

        network.add_link(from, to);
    }

    let mut triangle_count = 0;
    for triangle in network.get_triangles() {
        if triangle
            .into_iter()
            .any(|node| network.names[node].starts_with('t'))
        {
            triangle_count += 1;
        }
    }

    let biggest_group = network.find_biggest_group::<15>();
    let mut biggest_names = biggest_group
        .into_iter()
        .map(|node| network.names[node].clone())
        .collect::<Vec<_>>();

    biggest_names.sort_unstable();
    let password = biggest_names.join(",");

    Ok((triangle_count, password))
}
