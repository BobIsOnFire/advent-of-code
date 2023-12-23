use std::collections::{HashMap, HashSet};

use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::West => Self::East,
            Self::South => Self::North,
            Self::East => Self::West,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::Slope(Direction::North),
            '<' => Self::Slope(Direction::West),
            'v' => Self::Slope(Direction::South),
            '>' => Self::Slope(Direction::East),
            _ => panic!("Unknown tile {}", value),
        }
    }
}

fn next_idx(tilemap: &VecMatrix<Tile>, idx: MatrixIndex, direction: Direction) -> Option<MatrixIndex> {
    match direction {
        Direction::North => tilemap.next_up(idx),
        Direction::West => tilemap.next_left(idx),
        Direction::South => tilemap.next_down(idx),
        Direction::East => tilemap.next_right(idx),
    }
}

struct Edge {
    to: MatrixIndex,
    length: usize,
}

struct Vertex {
    edges: Vec<Edge>,
}

impl Vertex {
    fn new() -> Self {
        Self { edges: vec![] }
    }
}

fn create_edge(tilemap: &VecMatrix<Tile>, start: MatrixIndex, mut direction: Direction) -> Option<Edge> {
    let mut current = start;
    let mut length = 0;

    'outer: loop {
        if let Some(next) = next_idx(tilemap, current, direction) {
            current = next;
        } else {
            return None;
        }
        length += 1;

        match tilemap[current] {
            Tile::Forest => return None,
            Tile::Slope(slope) if slope == direction.opposite() => return None,
            _ => {}
        }

        let mut next_direction = None;
        for dir in [Direction::North, Direction::West, Direction::South, Direction::East] {
            if dir == direction.opposite() {
                continue;
            }

            if let Some(idx) = next_idx(tilemap, current, dir) {
                if tilemap[idx] != Tile::Forest && next_direction.replace(dir).is_some() {
                    // Multiple directions to continue => next vertex is reached
                    break 'outer;
                }
            } else {
                // end of map was reached => either a start or finish
                break 'outer;
            }
        }

        if let Some(next) = next_direction {
            direction = next;
        } else {
            // all surrounding tiles are forest => dead end
            return None;
        }
    }

    Some(Edge { to: current, length })
}

fn create_vertex(tilemap: &VecMatrix<Tile>, start: MatrixIndex) -> Vertex {
    let mut vertex = Vertex::new();

    for direction in [Direction::North, Direction::West, Direction::South, Direction::East] {
        if let Some(edge) = create_edge(tilemap, start, direction) {
            vertex.edges.push(edge);
        }
    }

    vertex
}

fn do_create_vertices(tilemap: &VecMatrix<Tile>, start: MatrixIndex, vertices: &mut HashMap<MatrixIndex, Vertex>) {
    if vertices.contains_key(&start) {
        return;
    }

    vertices.insert(start, create_vertex(tilemap, start));
    let vertex = vertices.get(&start).unwrap();

    let connected_vertices = vertex.edges.iter().map(|e| e.to).collect::<Vec<_>>();
    for next in connected_vertices {
        do_create_vertices(tilemap, next, vertices);
    }
}

fn create_vertices(tilemap: &VecMatrix<Tile>, start: MatrixIndex) -> HashMap<MatrixIndex, Vertex> {
    let mut vertices = HashMap::new();
    do_create_vertices(tilemap, start, &mut vertices);
    vertices
}

fn do_find_max_path(
    vertices: &HashMap<MatrixIndex, Vertex>,
    start: MatrixIndex,
    end: MatrixIndex,
    visited: &mut HashSet<MatrixIndex>,
    current: usize,
) -> usize {
    if start == end {
        return current;
    }

    visited.insert(start);

    let mut max_path = 0;
    for edge in vertices.get(&start).unwrap().edges.iter() {
        if !visited.contains(&edge.to) {
            let path = do_find_max_path(vertices, edge.to, end, visited, current + edge.length);
            max_path = usize::max(max_path, path);
        }
    }

    visited.remove(&start);

    max_path
}

fn find_max_path(tilemap: &VecMatrix<Tile>, start: MatrixIndex, end: MatrixIndex) -> usize {
    let vertices = create_vertices(tilemap, start);

    let mut visited = HashSet::new();
    do_find_max_path(&vertices, start, end, &mut visited, 0)
}

pub fn find_longest_path(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let tilemap = {
        let mut data = vec![];
        let mut width = 0;

        for line in lines {
            width = line.len();
            data.extend(line.chars().map(Tile::from));
        }

        VecMatrix::with_data(data, width)
    };

    let start = MatrixIndex { row: 0, col: 1 };
    let end = MatrixIndex {
        row: tilemap.height() - 1,
        col: tilemap.width() - 2,
    };

    let max_path = find_max_path(&tilemap, start, end);

    let data_no_slopes = tilemap
        .data()
        .iter()
        .map(|tile| match tile {
            Tile::Slope(_) => Tile::Path,
            other => *other,
        })
        .collect::<Vec<_>>();

    let tilemap_no_slopes = VecMatrix::with_data(data_no_slopes, tilemap.width());

    let max_path_no_slopes = find_max_path(&tilemap_no_slopes, start, end);

    Ok((max_path, max_path_no_slopes))
}
