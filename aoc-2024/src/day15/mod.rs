use std::collections::HashSet;

use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[allow(dead_code)] // linter warns on fields, we use them only to print in Debug/Display impl
#[derive(Debug)]
pub struct CharParseError {
    expected: String,
    actual: char,
}

impl std::fmt::Display for CharParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CharParseError {}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Tile {
    Empty,
    Box,
    Wall,
    Robot,
}

impl TryFrom<char> for Tile {
    type Error = CharParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            'O' => Ok(Self::Box),
            '#' => Ok(Self::Wall),
            '@' => Ok(Self::Robot),
            ch => Err(CharParseError {
                expected: "Tile ('.', 'O', '#', '@')".to_string(),
                actual: ch,
            }),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = CharParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            v => Err(CharParseError {
                expected: "Direction [^, >, v, <]".into(),
                actual: v,
            }),
        }
    }
}

struct Map {
    tilemap: VecMatrix<Tile>,
    robot_position: MatrixIndex,
}

impl Map {
    fn new(mut tilemap: VecMatrix<Tile>) -> Result<Self, String> {
        let robot_position = tilemap
            .iter_enumerate()
            .find_map(|(idx, tile)| matches!(tile, Tile::Robot).then_some(idx))
            .ok_or("Robot is missing")?;
        tilemap[robot_position] = Tile::Empty;

        Ok(Self { tilemap, robot_position })
    }

    fn clone_to_wide(&self) -> Self {
        let mut data = vec![];

        for tile in &self.tilemap {
            match tile {
                Tile::Robot => panic!("More than one robot on the map"),
                Tile::Empty => data.extend([Tile::Empty, Tile::Empty]),
                Tile::Box => data.extend([Tile::Box, Tile::Empty]),
                Tile::Wall => data.extend([Tile::Wall, Tile::Wall]),
            }
        }

        Self {
            tilemap: VecMatrix::with_data(data, self.tilemap.width() * 2),
            robot_position: MatrixIndex {
                row: self.robot_position.row,
                col: self.robot_position.col * 2,
            },
        }
    }

    const fn next_position(
        &self,
        position: MatrixIndex,
        direction: Direction,
    ) -> Option<MatrixIndex> {
        match direction {
            Direction::Up => self.tilemap.next_up(position),
            Direction::Right => self.tilemap.next_right(position),
            Direction::Down => self.tilemap.next_down(position),
            Direction::Left => self.tilemap.next_left(position),
        }
    }

    const fn next_position_checked(
        &self,
        position: MatrixIndex,
        direction: Direction,
    ) -> MatrixIndex {
        self.next_position(position, direction)
            .expect("Cannot move out of bounds")
    }

    fn move_robot(&mut self, direction: Direction) {
        let forward = self.next_position_checked(self.robot_position, direction);

        match self.tilemap[forward] {
            Tile::Robot => panic!("More than one robot on the map"),
            // Tile forward is empty, we can move without issues
            Tile::Empty => self.robot_position = forward,
            // Tile forward is a wall, this is a move failure
            Tile::Wall => (),
            // There's a box forward to us, need to move it. Find out if it's even possible to move them
            Tile::Box => {
                let mut next_empty = forward;
                loop {
                    next_empty = self.next_position_checked(next_empty, direction);

                    match self.tilemap[next_empty] {
                        Tile::Robot => panic!("More than one robot on the map"),
                        Tile::Box => {}
                        // line of boxes ends with a wall, this is a move failure
                        Tile::Wall => break,
                        // line of boxes ends with an empty tile, move boxes and then move ourselves
                        Tile::Empty => {
                            self.tilemap[next_empty] = Tile::Box;
                            self.tilemap[forward] = Tile::Empty;
                            self.robot_position = forward;
                            break;
                        }
                    }
                }
            }
        }
    }

    fn move_wide_boxes_vertical(
        &mut self,
        box_position: MatrixIndex,
        direction: Direction,
    ) -> bool {
        use Direction::{Left, Right};
        use Tile::{Box, Empty, Wall};

        let mut boxes = HashSet::new();
        let mut stack = vec![box_position];

        while let Some(box_position) = stack.pop() {
            assert!(self.tilemap[box_position] == Box);
            if boxes.contains(&box_position) {
                continue;
            }
            boxes.insert(box_position);

            let forward = self.next_position_checked(box_position, direction);
            let forward_left = self.next_position_checked(forward, Left);
            let forward_right = self.next_position_checked(forward, Right);

            if self.tilemap[forward_left] != Box
                && self.tilemap[forward] == Empty
                && self.tilemap[forward_right] == Empty
            {
                continue;
            }

            if self.tilemap[forward] == Wall || self.tilemap[forward_right] == Wall {
                return false;
            }

            for tile in [forward_left, forward, forward_right] {
                if self.tilemap[tile] == Box {
                    stack.push(tile);
                }
            }
        }

        // Remove boxes from the map
        for &box_position in &boxes {
            self.tilemap[box_position] = Tile::Empty;
        }

        // Add them back at forward positions
        for &box_position in &boxes {
            let forward = self.next_position_checked(box_position, direction);
            self.tilemap[forward] = Tile::Box;
        }

        true
    }

    fn move_wide_boxes_horizontal(
        &mut self,
        box_position: MatrixIndex,
        direction: Direction,
    ) -> bool {
        use Direction::{Left, Right};
        use Tile::{Box, Empty, Wall};

        #[derive(Clone)]
        struct WideBox(MatrixIndex, MatrixIndex);
        fn get_next_box(this: &Map, current: &WideBox, direction: Direction) -> WideBox {
            if direction == Right {
                let left = this.next_position_checked(current.1, Right);
                let right = this.next_position_checked(left, Right);

                WideBox(left, right)
            } else {
                let right = this.next_position_checked(current.0, Left);
                let left = this.next_position_checked(right, Left);

                WideBox(left, right)
            }
        }

        let current_box = WideBox(
            box_position,
            self.next_position_checked(box_position, Right),
        );

        let mut next_box = current_box.clone();
        let first_empty_box = loop {
            next_box = get_next_box(self, &next_box, direction);

            if self.tilemap[next_box.0] == Box {
                continue;
            }

            let tile_to_check = if direction == Right { next_box.0 } else { next_box.1 };
            if self.tilemap[tile_to_check] == Wall {
                return false;
            }

            break next_box;
        };

        let mut current_box = current_box;
        loop {
            let next_box = get_next_box(self, &current_box, direction);

            let moved_tile = if direction == Right { current_box.1 } else { next_box.1 };
            self.tilemap[current_box.0] = Empty;
            self.tilemap[moved_tile] = Box;

            if next_box.0 == first_empty_box.0 {
                break;
            }

            current_box = next_box;
        }

        true
    }

    fn move_wide_boxes(&mut self, box_position: MatrixIndex, direction: Direction) -> bool {
        match direction {
            Direction::Up | Direction::Down => {
                self.move_wide_boxes_vertical(box_position, direction)
            }
            Direction::Right | Direction::Left => {
                self.move_wide_boxes_horizontal(box_position, direction)
            }
        }
    }

    fn move_robot_wide(&mut self, direction: Direction) {
        let forward = self.next_position_checked(self.robot_position, direction);
        let forward_left = self.next_position_checked(forward, Direction::Left);

        match (self.tilemap[forward_left], self.tilemap[forward]) {
            (_, Tile::Robot) => panic!("More than one robot on the map"),
            (_, Tile::Box) => {
                if self.move_wide_boxes(forward, direction) {
                    self.robot_position = forward;
                }
            }
            (Tile::Box, _) => {
                if self.move_wide_boxes(forward_left, direction) {
                    self.robot_position = forward;
                }
            }
            (_, Tile::Empty) => self.robot_position = forward,
            (_, Tile::Wall) => (),
        }
    }

    #[allow(unused)]
    fn draw(&self) {
        for (idx, tile) in self.tilemap.iter_enumerate() {
            if self.robot_position == idx {
                print!("@");
            } else {
                match tile {
                    Tile::Empty => print!("."),
                    Tile::Box => print!("O"),
                    Tile::Wall => print!("#"),
                    Tile::Robot => panic!("???"),
                }
            }

            if idx.col == self.tilemap.width() - 1 {
                println!();
            }
        }
    }

    fn get_box_gps(&self) -> usize {
        self.tilemap
            .iter_enumerate()
            .filter_map(|(idx, tile)| matches!(tile, Tile::Box).then_some(idx))
            .map(|idx| 100 * idx.row + idx.col)
            .sum()
    }
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let (tilemap, movements) = {
        let mut lines = lines.peekable();

        let width = lines.by_ref().peek().map_or(0, String::len);
        let data: Vec<Tile> = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .flat_map(String::into_bytes)
            .map(|byte| byte as char)
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;

        let movements: Vec<Direction> = lines
            .flat_map(String::into_bytes)
            .map(|byte| byte as char)
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;

        (VecMatrix::with_data(data, width), movements)
    };

    let mut map = Map::new(tilemap)?;
    let mut wide_map = map.clone_to_wide();
    // wide_map.draw();

    for movement in movements {
        map.move_robot(movement);
        wide_map.move_robot_wide(movement);
        // wide_map.draw();
    }

    let gps_sum = map.get_box_gps();
    let gps_sum_wide = wide_map.get_box_gps();

    Ok((gps_sum, gps_sum_wide))
}
