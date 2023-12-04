use std::collections::HashSet;

use aoc_common::util::{self, MatrixIndex};

type Coord = MatrixIndex;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Wind {
    coord: Coord,
    direction: Direction,
}

struct Field {
    winds: Vec<Wind>,
    taken_by_wind: HashSet<Coord>,
    height: usize,
    width: usize,
}

impl Field {
    fn new(winds: Vec<Wind>, height: usize, width: usize) -> Self {
        let taken_by_wind = winds.iter().map(|w| w.coord).collect();
        Self { winds, taken_by_wind, height, width }
    }

    fn move_winds(&mut self) {
        let mut new_winds = Vec::with_capacity(self.winds.len());

        for mut wind in self.winds.drain(..) {
            match wind.direction {
                Direction::Up => wind.coord.row = if wind.coord.row > 1 { wind.coord.row - 1 } else { self.height - 2 },
                Direction::Down => wind.coord.row = if wind.coord.row < self.height - 2 { wind.coord.row + 1 } else { 1 },
                Direction::Left => wind.coord.col = if wind.coord.col > 1 { wind.coord.col - 1 } else { self.width - 2 },
                Direction::Right => wind.coord.col = if wind.coord.col < self.width - 2 { wind.coord.col + 1 } else { 1 },
            }

            new_winds.push(wind);
        }

        self.winds = new_winds;
        self.taken_by_wind = self.winds.iter().map(|w| w.coord).collect();
    }

    fn can_move_into(&self, coord: Coord) -> bool {
        !self.taken_by_wind.contains(&coord)
    }
}

fn generate_states(field: &Field, states: HashSet<Coord>) -> HashSet<Coord> {
    let mut new_states = HashSet::new();
    for state in states {
        let Coord { row, col } = state;
        let possible_moves = [
            Some(state),                                                                   // stay
            if row > 1 { Some(Coord { row: row - 1, col }) } else { None },                // up
            if row < field.height - 2 { Some(Coord { row: row + 1, col }) } else { None }, // down
            if col > 1 { Some(Coord { row, col: col - 1 }) } else { None },                // left
            if col < field.width - 2 { Some(Coord { row, col: col + 1 }) } else { None },  // right
        ];

        new_states.extend(possible_moves.into_iter().flatten().filter(|&m| field.can_move_into(m)));
    }

    new_states
}

fn rounds_to_reach(field: &mut Field, start: Coord, end: Coord) -> usize {
    let mut rounds_passed = 0;
    let mut states = HashSet::new();

    loop {
        field.move_winds();
        rounds_passed += 1;

        states = generate_states(field, states);
        if field.can_move_into(start) {
            states.insert(start);
        }
        // print_states(&states);

        if states.iter().any(|&s| s == end) {
            break;
        }
    }

    // +1 round to get out of the field into safety
    field.move_winds();
    rounds_passed += 1;

    rounds_passed
}

#[allow(dead_code)]
fn print_states(states: &HashSet<Coord>) {
    let mut formatted = states.iter().map(|c| format!("[{},{}]", c.row, c.col)).collect::<Vec<_>>();
    formatted.sort();
    println!("[{}]", formatted.join(", "))
}

pub fn count_path_minutes(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut height = 0;
    let mut width = 0;

    let mut winds = Vec::new();

    for (row, line) in lines.enumerate() {
        height += 1;
        width = line.len();
        for (col, ch) in line.chars().enumerate() {
            let direction = match ch {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => continue,
            };

            winds.push(Wind { coord: Coord { row, col }, direction });
        }
    }

    let start = Coord { row: 1, col: 1 };
    let end = Coord { row: height - 2, col: width - 2 };

    let mut field = Field::new(winds, height, width);

    let mut rounds_total = rounds_to_reach(&mut field, start, end);
    let rounds_once = rounds_total;

    rounds_total += rounds_to_reach(&mut field, end, start);
    rounds_total += rounds_to_reach(&mut field, start, end);

    Ok((rounds_once, rounds_total))
}
