use aoc_common::util;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum NumericButton {
    Number(u8),
    Confirm,
}

impl NumericButton {
    /*
    +---+---+---+
    | 7 | 8 | 9 |
    +---+---+---+
    | 4 | 5 | 6 |
    +---+---+---+
    | 1 | 2 | 3 |
    +---+---+---+
        | 0 | A |
        +---+---+
    */

    const fn row(self) -> u8 {
        match self {
            Self::Number(num) => 3 - (num + 2) / 3,
            Self::Confirm => 3,
        }
    }

    const fn col(self) -> u8 {
        match self {
            Self::Number(0) => 1,
            Self::Number(num) => (num - 1) % 3,
            Self::Confirm => 2,
        }
    }
}

impl TryFrom<char> for NumericButton {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0'..='9' => Ok(Self::Number(value as u8 - b'0')),
            'A' => Ok(Self::Confirm),
            _ => Err(format!("Invalid numeric button: {value}")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum DirectionButton {
    Left,
    Right,
    Up,
    Down,
    Confirm,
}

impl DirectionButton {
    /*
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+
    */

    const fn row(self) -> u8 {
        match self {
            Self::Up | Self::Confirm => 0,
            _ => 1,
        }
    }

    const fn col(self) -> u8 {
        match self {
            Self::Left => 0,
            Self::Up | Self::Down => 1,
            _ => 2,
        }
    }
}

struct Distance {
    vertical: i8,   // positive => down, negative => up
    horizontal: i8, // positive => right, negative => left
}

impl Distance {
    const fn between_numeric(from: NumericButton, to: NumericButton) -> Self {
        Self {
            vertical: to.row() as i8 - from.row() as i8,
            horizontal: to.col() as i8 - from.col() as i8,
        }
    }

    const fn between_directions(from: DirectionButton, to: DirectionButton) -> Self {
        Self {
            vertical: to.row() as i8 - from.row() as i8,
            horizontal: to.col() as i8 - from.col() as i8,
        }
    }
}

struct NumericKeypad {
    position: NumericButton,
    moves: Vec<DirectionButton>,
}

impl NumericKeypad {
    const fn new() -> Self {
        Self {
            position: NumericButton::Confirm,
            moves: vec![],
        }
    }

    fn add_vertical_moves(&mut self, vertical: i8) {
        if vertical > 0 {
            for _ in 0..vertical {
                self.moves.push(DirectionButton::Down);
            }
        } else {
            for _ in 0..(-vertical) {
                self.moves.push(DirectionButton::Up);
            }
        }
    }

    fn add_horizontal_moves(&mut self, horizontal: i8) {
        if horizontal > 0 {
            for _ in 0..horizontal {
                self.moves.push(DirectionButton::Right);
            }
        } else {
            for _ in 0..(-horizontal) {
                self.moves.push(DirectionButton::Left);
            }
        }
    }

    fn move_to(&mut self, position: NumericButton) {
        let distance = Distance::between_numeric(self.position, position);

        // let vertical_first = self.position.row() == 3 && position.col() == 0;
        let horizontal_first = self.position.col() == 0 && position.row() == 3;

        if !horizontal_first {
            self.add_vertical_moves(distance.vertical);
            self.add_horizontal_moves(distance.horizontal);
        } else {
            self.add_horizontal_moves(distance.horizontal);
            self.add_vertical_moves(distance.vertical);
        }

        self.moves.push(DirectionButton::Confirm);

        self.position = position;
    }
}

struct DirectionKeypad {
    position: DirectionButton,
    moves: Vec<DirectionButton>,
}

impl DirectionKeypad {
    const fn new() -> Self {
        Self {
            position: DirectionButton::Confirm,
            moves: vec![],
        }
    }

    fn add_vertical_moves(&mut self, vertical: i8) {
        if vertical > 0 {
            for _ in 0..vertical {
                self.moves.push(DirectionButton::Down);
            }
        } else {
            for _ in 0..(-vertical) {
                self.moves.push(DirectionButton::Up);
            }
        }
    }

    fn add_horizontal_moves(&mut self, horizontal: i8) {
        if horizontal > 0 {
            for _ in 0..horizontal {
                self.moves.push(DirectionButton::Right);
            }
        } else {
            for _ in 0..(-horizontal) {
                self.moves.push(DirectionButton::Left);
            }
        }
    }

    fn move_to(&mut self, position: DirectionButton) {
        let distance = Distance::between_directions(self.position, position);

        // let horizontal_first = self.position.col() == 0;
        let vertical_first = self.position.row() == 0;

        if !vertical_first {
            self.add_horizontal_moves(distance.horizontal);
            self.add_vertical_moves(distance.vertical);
        } else {
            self.add_vertical_moves(distance.vertical);
            self.add_horizontal_moves(distance.horizontal);
        }

        self.moves.push(DirectionButton::Confirm);

        self.position = position;
    }
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut complexity_total = 0;

    for line in lines {
        let mut value = 0;
        let mut keypad = NumericKeypad::new();
        for ch in line.chars() {
            let button = ch.try_into()?;
            if let NumericButton::Number(num) = button {
                value = value * 10 + num as usize;
            }
            keypad.move_to(button);
        }

        let mut first_directions = DirectionKeypad::new();
        for direction in keypad.moves {
            first_directions.move_to(direction);
        }

        let mut second_directions = DirectionKeypad::new();
        for direction in first_directions.moves {
            second_directions.move_to(direction);
        }

        let move_len = second_directions.moves.len();

        println!("{value} {move_len}");

        complexity_total += value * move_len;
    }

    Ok((complexity_total, 0))
}
