use std::collections::{HashMap, HashSet};

use aoc_common::util::iter::ArrayIterators;

use super::data::{BoundsMapping, Coord, Direction, TileMap};

const EDGE_LENGTH: usize = 50;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Vertex {
    row: usize,
    col: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Edge {
    vertex_from: Vertex,
    vertex_to: Vertex,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Face {
    top_left: Vertex,
}

////////////////////////////////////////////////////////////////////////////////

impl Vertex {
    fn neighbour(&self, direction: Direction) -> Option<Self> {
        let Self { row, col } = *self;

        match direction {
            Direction::Right => col.checked_add(EDGE_LENGTH).map(|col| Self { row, col }),
            Direction::Down => row.checked_add(EDGE_LENGTH).map(|row| Self { row, col }),
            Direction::Left => col.checked_sub(EDGE_LENGTH).map(|col| Self { row, col }),
            Direction::Up => row.checked_sub(EDGE_LENGTH).map(|row| Self { row, col }),
        }
    }

    fn get_edge(&self, direction: Direction) -> Option<Edge> {
        self.neighbour(direction).map(|vertex_to| Edge {
            vertex_from: *self,
            vertex_to,
        })
    }
}

impl From<Vertex> for Coord {
    fn from(value: Vertex) -> Self {
        let Vertex { row, col } = value;
        Self { row, col }
    }
}

impl Edge {
    fn reversed(&self) -> Self {
        Self {
            vertex_from: self.vertex_to,
            vertex_to: self.vertex_from,
        }
    }

    fn is_vertical(&self) -> bool {
        self.vertex_from.col == self.vertex_to.col
    }

    fn is_horizontal(&self) -> bool {
        self.vertex_from.row == self.vertex_to.row
    }

    fn get_line_direction(&self) -> Direction {
        use std::cmp::Ordering::*;
        match self.vertex_to.row.cmp(&self.vertex_from.row) {
            Greater => Direction::Down,
            Equal => {
                if self.vertex_from.col < self.vertex_to.col {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Less => Direction::Up,
        }
    }

    fn coord_range(&self) -> impl Iterator<Item = Coord> {
        let direction = self.get_line_direction();
        let &Edge {
            vertex_from: from,
            vertex_to: to,
        } = self;

        let range: Box<dyn Iterator<Item = usize>> = match direction {
            Direction::Right => Box::new(from.col..to.col),
            Direction::Down => Box::new(from.row..to.row),
            Direction::Left => Box::new((to.col..from.col).rev()),
            Direction::Up => Box::new((to.row..from.row).rev()),
        };

        let mapper: Box<dyn Fn(usize) -> Coord> = match direction {
            Direction::Right | Direction::Left => Box::new(move |col| Coord { row: from.row, col }),
            Direction::Down | Direction::Up => Box::new(move |row| Coord { row, col: from.col }),
        };

        range.map(mapper)
    }
}

impl Face {
    fn neighbour(&self, direction: Direction) -> Option<Self> {
        self.top_left
            .neighbour(direction)
            .map(|top_left| Self { top_left })
    }

    fn top_left(&self) -> Vertex {
        self.top_left
    }

    fn top_right(&self) -> Vertex {
        self.top_left().neighbour(Direction::Right).unwrap()
    }

    fn down_left(&self) -> Vertex {
        self.top_left().neighbour(Direction::Down).unwrap()
    }

    fn down_right(&self) -> Vertex {
        self.down_left().neighbour(Direction::Right).unwrap()
    }

    fn vertices_clockwise(&self) -> [Vertex; 4] {
        [
            self.top_left(),
            self.top_right(),
            self.down_right(),
            self.down_left(),
        ]
    }

    fn vertices_counter_clockwise(&self) -> [Vertex; 4] {
        [
            self.top_left(),
            self.down_left(),
            self.down_right(),
            self.top_right(),
        ]
    }

    fn get_edge(&self, direction: Direction) -> Edge {
        let (vertex_from, vertex_to) = match direction {
            Direction::Right => (self.top_right(), self.down_right()),
            Direction::Down => (self.down_left(), self.down_right()),
            Direction::Left => (self.top_left(), self.down_left()),
            Direction::Up => (self.top_left(), self.top_right()),
        };

        Edge {
            vertex_from,
            vertex_to,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum VertexKind {
    Bottom(u8),
    Top(u8),
}

impl VertexKind {
    fn unwrap_bottom(self) -> u8 {
        match self {
            Self::Bottom(num) => num,
            Self::Top(_) => panic!("Should be Bottom!"),
        }
    }
    fn unwrap_top(self) -> u8 {
        match self {
            Self::Top(num) => num,
            Self::Bottom(_) => panic!("Should be Top!"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum FaceKind {
    Bottom,
    Side,
    Top,
}

#[derive(Debug)]
struct CubeNet {
    vertex_colors: HashMap<Vertex, VertexKind>,
    edge_mapping: HashMap<Edge, Edge>,
    set_bounds: HashMap<Edge, Direction>,
}

impl CubeNet {
    fn from_tilemap(tile_map: &TileMap) -> Self {
        let mut result = Self {
            vertex_colors: HashMap::new(),
            edge_mapping: HashMap::new(),
            set_bounds: HashMap::new(),
        };

        result.paint_face_vertices(tile_map);
        result.map_edges();
        result.map_set_bounds();

        result
    }

    fn paint_face_vertices(&mut self, tile_map: &TileMap) {
        let mut start_vertex = Vertex { row: 1, col: 1 };
        while tile_map[start_vertex.into()].is_none() {
            start_vertex = start_vertex.neighbour(Direction::Right).unwrap();
        }

        let face = Face {
            top_left: start_vertex,
        };
        self.vertex_colors
            .insert(start_vertex, VertexKind::Bottom(0));
        self.do_paint_face_vertices(tile_map, face, FaceKind::Bottom);
    }

    fn map_edges(&mut self) {
        let all_edges: Vec<_> = self
            .vertex_colors
            .keys()
            .flat_map(|v| {
                [
                    v.get_edge(Direction::Right).unwrap(),
                    v.get_edge(Direction::Down).unwrap(),
                ]
            })
            .filter(|e| self.vertex_colors.contains_key(&e.vertex_to))
            .collect();

        for &edge in all_edges.iter() {
            let equivalent = all_edges
                .iter()
                .copied()
                .chain(all_edges.iter().map(|e| e.reversed()))
                .find(|&other| edge != other && self.are_edges_equivalent(edge, other));

            self.edge_mapping.insert(edge, equivalent.unwrap_or(edge));
        }
    }

    fn map_set_bounds(&mut self) {
        let vertical_edges = self
            .edge_mapping
            .keys()
            .copied()
            .filter(Edge::is_vertical)
            .collect::<Vec<_>>();

        let edge_starts_rows = vertical_edges
            .iter()
            .map(|e| e.vertex_from.row)
            .collect::<HashSet<_>>();

        for row in edge_starts_rows {
            let row_edges = vertical_edges
                .iter()
                .filter(move |e| e.vertex_from.row == row)
                .copied()
                .collect::<Vec<_>>();

            let leftmost_edge = *row_edges.iter().min_by_key(|e| e.vertex_from.col).unwrap();
            self.set_bounds.insert(leftmost_edge, Direction::Left);
            let rightmost_edge = *row_edges.iter().max_by_key(|e| e.vertex_from.col).unwrap();
            self.set_bounds.insert(rightmost_edge, Direction::Right);
        }

        let horizontal_edges = self
            .edge_mapping
            .keys()
            .copied()
            .filter(Edge::is_horizontal)
            .collect::<Vec<_>>();

        let edge_starts_cols = horizontal_edges
            .iter()
            .map(|e| e.vertex_from.col)
            .collect::<HashSet<_>>();

        for col in edge_starts_cols {
            let col_edges = horizontal_edges
                .iter()
                .filter(move |e| e.vertex_from.col == col)
                .copied()
                .collect::<Vec<_>>();

            let upmost_edge = *col_edges.iter().min_by_key(|e| e.vertex_from.row).unwrap();
            self.set_bounds.insert(upmost_edge, Direction::Up);
            let downmost_edge = *col_edges.iter().max_by_key(|e| e.vertex_from.row).unwrap();
            self.set_bounds.insert(downmost_edge, Direction::Down);
        }
    }

    fn do_paint_face_vertices(&mut self, tile_map: &TileMap, face: Face, kind: FaceKind) {
        if self.are_vertices_colored(face) {
            return;
        }

        self.paint_vertices(face, kind);

        for direction in Direction::all() {
            if let Some(neigh) = face
                .neighbour(direction)
                .filter(|f| tile_map[f.top_left.into()].is_some())
            {
                self.do_paint_face_vertices(
                    tile_map,
                    neigh,
                    self.get_neighbour_kind(face, kind, direction),
                );
            }
        }
    }

    fn are_vertices_colored(&self, face: Face) -> bool {
        face.vertices_clockwise()
            .iter()
            .filter_map(|v| self.vertex_colors.get(v))
            .count()
            == 4
    }

    fn paint_vertices(&mut self, face: Face, kind: FaceKind) {
        match kind {
            FaceKind::Bottom => {
                for vertices in face.vertices_clockwise().windows_cycle().take(4) {
                    if let Some(v) = self.vertex_colors.get(vertices[0]) {
                        let value = v.unwrap_bottom();
                        self.vertex_colors
                            .insert(*vertices[1], VertexKind::Bottom((value + 1) % 4));
                        self.vertex_colors
                            .insert(*vertices[2], VertexKind::Bottom((value + 2) % 4));
                        self.vertex_colors
                            .insert(*vertices[3], VertexKind::Bottom((value + 3) % 4));
                        break;
                    }
                }
            }
            FaceKind::Top => {
                for vertices in face.vertices_counter_clockwise().windows_cycle().take(4) {
                    if let Some(v) = self.vertex_colors.get(vertices[0]) {
                        let value = v.unwrap_top();
                        self.vertex_colors
                            .insert(*vertices[1], VertexKind::Top((value + 1) % 4));
                        self.vertex_colors
                            .insert(*vertices[2], VertexKind::Top((value + 2) % 4));
                        self.vertex_colors
                            .insert(*vertices[3], VertexKind::Top((value + 3) % 4));
                        break;
                    }
                }
            }
            FaceKind::Side => {
                for vertices in face.vertices_clockwise().windows_cycle().take(4) {
                    if let (Some(&v0), Some(&v1)) = (
                        self.vertex_colors.get(vertices[0]),
                        self.vertex_colors.get(vertices[1]),
                    ) {
                        use VertexKind::*;
                        let (v2, v3) = match (v0, v1) {
                            (Bottom(v0), Bottom(v1)) => (Top(v1), Top(v0)),
                            (Bottom(v0), Top(_)) => (Top((v0 + 1) % 4), Bottom((v0 + 1) % 4)),
                            (Top(v0), Bottom(_)) => (Bottom((v0 - 1) % 4), Top((v0 - 1) % 4)),
                            (Top(v0), Top(v1)) => (Bottom(v1), Bottom(v0)),
                        };

                        self.vertex_colors.insert(*vertices[2], v2);
                        self.vertex_colors.insert(*vertices[3], v3);
                        break;
                    }
                }
            }
        }
    }

    fn get_neighbour_kind(&self, face: Face, kind: FaceKind, direction: Direction) -> FaceKind {
        match kind {
            FaceKind::Top | FaceKind::Bottom => FaceKind::Side,
            FaceKind::Side => {
                let Edge {
                    vertex_from,
                    vertex_to,
                } = face.get_edge(direction);

                match (self.get_color(vertex_from), self.get_color(vertex_to)) {
                    (VertexKind::Top(_), VertexKind::Top(_)) => FaceKind::Top,
                    (VertexKind::Bottom(_), VertexKind::Bottom(_)) => FaceKind::Bottom,
                    _ => FaceKind::Side,
                }
            }
        }
    }

    fn get_color(&self, vertex: Vertex) -> VertexKind {
        *self.vertex_colors.get(&vertex).unwrap()
    }

    fn are_edges_equivalent(&self, first: Edge, second: Edge) -> bool {
        self.get_color(first.vertex_from) == self.get_color(second.vertex_from)
            && self.get_color(first.vertex_to) == self.get_color(second.vertex_to)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn cube_mapping(tile_map: &TileMap) -> BoundsMapping {
    let default = (Coord { row: 0, col: 0 }, Direction::Up);
    let mut bounds = BoundsMapping {
        right_side: vec![default; tile_map.height()],
        left_side: vec![default; tile_map.height()],
        down_side: vec![default; tile_map.width()],
        up_side: vec![default; tile_map.width()],
    };

    let cube_net = CubeNet::from_tilemap(tile_map);

    for (edge, direction) in cube_net.set_bounds.iter() {
        let mapping = cube_net.edge_mapping.get(edge).unwrap();
        let mapping_side = *cube_net
            .set_bounds
            .get(mapping)
            .or(cube_net.set_bounds.get(&mapping.reversed()))
            .unwrap();

        for (coord_from, mut coord_to) in edge.coord_range().zip(mapping.coord_range()) {
            let mapping_from = match direction {
                Direction::Right => &mut bounds.right_side[coord_from.row],
                Direction::Down => &mut bounds.down_side[coord_from.col],
                Direction::Left => &mut bounds.left_side[coord_from.row],
                Direction::Up => &mut bounds.up_side[coord_from.col],
            };

            if mapping_side == Direction::Right {
                coord_to.col -= 1;
            }

            if mapping_side == Direction::Down {
                coord_to.row -= 1;
            }

            *mapping_from = (coord_to, mapping_side.opposite());
        }
    }

    bounds
}
