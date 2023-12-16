use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mirror {
    Forward,
    Backward,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Splitter {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Mirror(Mirror),
    Splitter(Splitter),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::Mirror(Mirror::Forward),
            '\\' => Self::Mirror(Mirror::Backward),
            '|' => Self::Splitter(Splitter::Vertical),
            '-' => Self::Splitter(Splitter::Horizontal),
            _ => panic!("Unknown tile: {}", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn bounce(self, mirror: Mirror) -> Self {
        use Direction::*;
        use Mirror::*;
        match (self, mirror) {
            (Left, Backward) | (Right, Forward) => Up,
            (Up, Backward) | (Down, Forward) => Left,
            (Right, Backward) | (Left, Forward) => Down,
            (Down, Backward) | (Up, Forward) => Right,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Beam {
    location: MatrixIndex,
    direction: Direction,
}

struct VisitData {
    visits: [bool; 4],
}

impl VisitData {
    fn visit(&mut self, direction: Direction) {
        self.visits[direction as usize] = true;
    }

    fn is_visited(&self, direction: Direction) -> bool {
        self.visits[direction as usize]
    }

    fn any_visited(&self) -> bool {
        self.visits != [false; 4]
    }
}

struct BeamMap<'a> {
    tilemap: &'a VecMatrix<Tile>,
    visits: VecMatrix<VisitData>,
    beams: Vec<Beam>,
}

impl<'a> BeamMap<'a> {
    fn new(tilemap: &'a VecMatrix<Tile>, start_beam: Beam) -> Self {
        let visits_data = tilemap.data().iter().map(|_| VisitData { visits: [false; 4] }).collect();
        let mut visits = VecMatrix::with_data(visits_data, tilemap.width());

        visits[start_beam.location].visit(start_beam.direction);

        Self {
            tilemap,
            visits,
            beams: vec![start_beam],
        }
    }

    fn add_next_beam(&mut self, beam: Beam, direction: Direction) {
        let maybe_location = match direction {
            Direction::Up => self.tilemap.next_up(beam.location),
            Direction::Left => self.tilemap.next_left(beam.location),
            Direction::Down => self.tilemap.next_down(beam.location),
            Direction::Right => self.tilemap.next_right(beam.location),
        };

        if let Some(location) = maybe_location {
            let beam = Beam { location, direction };
            let beam_visits = &mut self.visits[beam.location];
            if !beam_visits.is_visited(beam.direction) {
                beam_visits.visit(beam.direction);
                self.beams.push(beam);
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.beams.is_empty()
    }

    fn count_visited(&self) -> usize {
        self.visits.iter().filter(|visits| visits.any_visited()).count()
    }

    fn forward(&mut self) {
        for beam in std::mem::take(&mut self.beams) {
            match self.tilemap[beam.location] {
                Tile::Empty => self.add_next_beam(beam, beam.direction),
                Tile::Mirror(mirror) => self.add_next_beam(beam, beam.direction.bounce(mirror)),
                Tile::Splitter(splitter) => {
                    use Direction::*;
                    use Splitter::*;
                    match (beam.direction, splitter) {
                        (Left | Right, Horizontal) | (Up | Down, Vertical) => self.add_next_beam(beam, beam.direction),
                        (Left | Right, Vertical) => {
                            self.add_next_beam(beam, Up);
                            self.add_next_beam(beam, Down);
                        }
                        (Up | Down, Horizontal) => {
                            self.add_next_beam(beam, Left);
                            self.add_next_beam(beam, Right);
                        }
                    }
                }
            }
        }
    }
}

fn all_edge_beams(width: usize, height: usize) -> impl Iterator<Item = Beam> {
    let top_beams = (0..width).map(move |col| Beam {
        location: MatrixIndex { row: 0, col },
        direction: Direction::Down,
    });
    let bottom_beams = (0..width).map(move |col| Beam {
        location: MatrixIndex { row: height - 1, col },
        direction: Direction::Up,
    });
    let left_beams = (0..height).map(move |row| Beam {
        location: MatrixIndex { row, col: 0 },
        direction: Direction::Right,
    });
    let right_beams = (0..height).map(move |row| Beam {
        location: MatrixIndex { row, col: width - 1 },
        direction: Direction::Left,
    });

    top_beams.chain(bottom_beams).chain(left_beams).chain(right_beams)
}

fn count_visited(tilemap: &VecMatrix<Tile>, start_beam: Beam) -> usize {
    let mut beam_map = BeamMap::new(tilemap, start_beam);
    while !beam_map.is_empty() {
        beam_map.forward();
    }
    beam_map.count_visited()
}

pub fn count_shining_tiles(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let tilemap = {
        let mut width = 0;
        let mut data = vec![];

        for line in lines {
            width = line.len();
            data.extend(line.chars().map(Tile::from))
        }
        VecMatrix::with_data(data, width)
    };

    let top_left_visits = count_visited(
        &tilemap,
        Beam {
            location: MatrixIndex { row: 0, col: 0 },
            direction: Direction::Right,
        },
    );

    let max_visits = all_edge_beams(tilemap.width(), tilemap.height())
        .map(|start_beam| count_visited(&tilemap, start_beam))
        .max()
        .unwrap_or(0);

    Ok((top_left_visits, max_visits))
}
