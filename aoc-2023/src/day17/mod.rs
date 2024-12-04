use std::collections::BTreeSet;

use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(Clone, Debug)]
struct Edge {
    to: MatrixIndex,
    weight: u64,
}

fn get_edges(
    tilemap: &VecMatrix<u8>,
    coord: MatrixIndex,
    min_length: usize,
    max_length: usize,
) -> Vec<Edge> {
    #[derive(Clone, Copy)]
    enum Direction {
        Up,
        Left,
        Down,
        Right,
    }

    fn next_idx(
        tilemap: &VecMatrix<u8>,
        coord: MatrixIndex,
        direction: Direction,
    ) -> Option<MatrixIndex> {
        match direction {
            Direction::Up => tilemap.next_up(coord),
            Direction::Left => tilemap.next_left(coord),
            Direction::Down => tilemap.next_down(coord),
            Direction::Right => tilemap.next_right(coord),
        }
    }

    let mut edges = Vec::new();

    for direction in [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ] {
        let mut current = Some(Edge { to: coord, weight: 0 });
        for i in 1..=max_length {
            current = current.and_then(|edge| {
                next_idx(tilemap, edge.to, direction).map(|coord| Edge {
                    to: coord,
                    weight: edge.weight + u64::from(tilemap[coord]),
                })
            });

            if let Some(edge) = current.clone() {
                if i >= min_length {
                    edges.push(edge);
                }
            } else {
                break;
            }
        }
    }

    edges
}

fn get_edges_map(
    tilemap: &VecMatrix<u8>,
    min_length: usize,
    max_length: usize,
) -> VecMatrix<Vec<Edge>> {
    let edges_data = tilemap
        .iter_enumerate()
        .map(|(idx, _)| get_edges(tilemap, idx, min_length, max_length))
        .collect();
    VecMatrix::with_data(edges_data, tilemap.width())
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(PartialEq, Eq, Debug)]
struct Node {
    min_path: u64,
    coord: MatrixIndex,
    orientation: Orientation,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.min_path.cmp(&other.min_path) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        match self.coord.row.cmp(&other.coord.row) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        match self.coord.col.cmp(&other.coord.col) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        self.orientation.cmp(&other.orientation)
    }
}

struct FlowMap {
    edges: VecMatrix<Vec<Edge>>,
    min_paths_horizontal: VecMatrix<u64>,
    min_paths_vertical: VecMatrix<u64>,
    nodes: BTreeSet<Node>,
    end: MatrixIndex,
}

impl FlowMap {
    fn new(edges: VecMatrix<Vec<Edge>>) -> Self {
        let start = MatrixIndex { row: 0, col: 0 };
        let end = MatrixIndex {
            row: edges.height() - 1,
            col: edges.width() - 1,
        };

        let mut min_paths_horizontal =
            VecMatrix::with_data(vec![u64::MAX; edges.len()], edges.width());
        min_paths_horizontal[start] = 0;
        let min_paths_vertical = min_paths_horizontal.clone();

        let nodes = BTreeSet::from([
            Node {
                min_path: 0,
                coord: start,
                orientation: Orientation::Horizontal,
            },
            Node {
                min_path: 0,
                coord: start,
                orientation: Orientation::Vertical,
            },
        ]);

        Self {
            edges,
            min_paths_horizontal,
            min_paths_vertical,
            nodes,
            end,
        }
    }

    fn get_next_node(&self, current: &Node, edge: &Edge) -> Option<Node> {
        let orientation = match current.orientation {
            Orientation::Horizontal => {
                if edge.to.col != current.coord.col {
                    return None;
                }
                Orientation::Vertical
            }
            Orientation::Vertical => {
                if edge.to.row != current.coord.row {
                    return None;
                }
                Orientation::Horizontal
            }
        };

        let min_path = match orientation {
            Orientation::Vertical => self.min_paths_vertical[edge.to],
            Orientation::Horizontal => self.min_paths_horizontal[edge.to],
        };

        Some(Node {
            min_path,
            coord: edge.to,
            orientation,
        })
    }

    fn find_min_path(mut self) -> u64 {
        // Classic Dijkstra
        loop {
            let node = self
                .nodes
                .pop_first()
                .expect("End node should be reachable");
            if node.coord == self.end {
                break;
            }

            for edge in &self.edges[node.coord] {
                let Some(mut to_node) = self.get_next_node(&node, edge) else {
                    continue;
                };
                if node.min_path + edge.weight < to_node.min_path {
                    self.nodes.remove(&to_node);
                    to_node.min_path = node.min_path + edge.weight;

                    match to_node.orientation {
                        Orientation::Horizontal => {
                            self.min_paths_horizontal[to_node.coord] = to_node.min_path
                        }
                        Orientation::Vertical => {
                            self.min_paths_vertical[to_node.coord] = to_node.min_path
                        }
                    }
                    self.nodes.insert(to_node);
                }
            }
        }

        u64::min(
            self.min_paths_horizontal[self.end],
            self.min_paths_vertical[self.end],
        )
    }
}

pub fn find_shortest_paths(lines: impl Iterator<Item = String>) -> util::GenericResult<(u64, u64)> {
    let tilemap = {
        let mut width = 0;
        let mut data = vec![];

        for line in lines {
            width = line.len();
            data.extend(line.bytes().map(|b| b - b'0'));
        }

        VecMatrix::with_data(data, width)
    };

    let edges_short = get_edges_map(&tilemap, 0, 3);
    let min_path_short = FlowMap::new(edges_short).find_min_path();

    let edges_long = get_edges_map(&tilemap, 4, 10);
    let min_path_long = FlowMap::new(edges_long).find_min_path();

    Ok((min_path_short, min_path_long))
}
