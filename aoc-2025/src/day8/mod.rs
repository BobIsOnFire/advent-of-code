use aoc_common::util;

struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

fn parse_coord(line: &str) -> Result<Coord, util::lexer::Error> {
    let mut lexer = util::Lexer::of(line);
    let x = lexer.unsigned_number()?;
    lexer.literal(",")?;
    let y = lexer.unsigned_number()?;
    lexer.literal(",")?;
    let z = lexer.unsigned_number()?;
    lexer.end()?;

    Ok(Coord { x, y, z })
}

fn distance_squared(first: &Coord, second: &Coord) -> usize {
    first.x.abs_diff(second.x).pow(2)
        + first.y.abs_diff(second.y).pow(2)
        + first.z.abs_diff(second.z).pow(2)
}

struct DisjointSet {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    component_count: usize,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            sizes: vec![1; size],
            component_count: size,
        }
    }

    fn find_root(&mut self, id: usize) -> usize {
        let mut root = id;
        while self.parents[root] != root {
            root = self.parents[root]
        }

        // compress path for next searches to be O(1)
        let mut current = id;
        while self.parents[current] != root {
            let parent = self.parents[current];
            self.parents[current] = root;
            current = parent;
        }

        root
    }

    fn merge_sets(&mut self, first: usize, second: usize) {
        let first_root = self.find_root(first);
        let second_root = self.find_root(second);

        if first_root == second_root {
            return;
        }

        self.component_count -= 1;

        if self.sizes[first_root] < self.sizes[second_root] {
            self.parents[first_root] = second_root;
            self.sizes[second_root] += self.sizes[first_root];
        } else {
            self.parents[second_root] = first_root;
            self.sizes[first_root] += self.sizes[second_root];
        }
    }
}

struct Distance {
    edge: (usize, usize),
    distance: usize,
}

pub fn connect_boxes(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let coords = lines
        .map(|line| parse_coord(&line))
        .collect::<Result<Vec<_>, _>>()?;

    let mut distances = (0..coords.len())
        .flat_map(|i| ((i + 1)..coords.len()).map(move |j| (i, j)))
        .map(|(i, j)| Distance {
            edge: (i, j),
            distance: distance_squared(&coords[i], &coords[j]),
        })
        .collect::<Vec<_>>();

    // very slow... is it possible not to sort 1M distances?...
    distances.sort_unstable_by_key(|d| d.distance);

    let mut final_edge = 0;
    let mut three_largest_at_1k = 0;

    let mut disjoint_set = DisjointSet::new(coords.len());

    for (idx, d) in distances.into_iter().enumerate() {
        let (from, to) = d.edge;
        disjoint_set.merge_sets(from, to);

        if idx == 1000 {
            let mut groups = (0..coords.len())
                .map(|i| disjoint_set.find_root(i))
                .collect::<Vec<_>>();
            groups.sort_unstable_by_key(|&i| disjoint_set.sizes[i]);
            groups.dedup();
            three_largest_at_1k = groups
                .into_iter()
                .rev()
                .take(3)
                .map(|i| disjoint_set.sizes[i])
                .product();
        }

        if disjoint_set.component_count == 1 {
            final_edge = coords[from].x * coords[to].x;
            break;
        }
    }

    Ok((three_largest_at_1k, final_edge))
}
