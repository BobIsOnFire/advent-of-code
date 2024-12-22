#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct KeypadPosition {
    pub row: usize,
    pub col: usize,
}

pub trait KeypadButton: TryFrom<KeypadPosition> + Into<KeypadPosition> {}
impl<T: TryFrom<KeypadPosition> + Into<KeypadPosition>> KeypadButton for T {}

// NUMERIC KEYPAD
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NumericButton {
    Number(u8),
    Confirm,
}

impl TryFrom<KeypadPosition> for NumericButton {
    type Error = ();

    fn try_from(pos: KeypadPosition) -> Result<Self, Self::Error> {
        match (pos.row, pos.col) {
            (0, 0) => Ok(Self::Number(7)),
            (0, 1) => Ok(Self::Number(8)),
            (0, 2) => Ok(Self::Number(9)),
            (1, 0) => Ok(Self::Number(4)),
            (1, 1) => Ok(Self::Number(5)),
            (1, 2) => Ok(Self::Number(6)),
            (2, 0) => Ok(Self::Number(1)),
            (2, 1) => Ok(Self::Number(2)),
            (2, 2) => Ok(Self::Number(3)),
            (3, 1) => Ok(Self::Number(0)),
            (3, 2) => Ok(Self::Confirm),
            _ => Err(()),
        }
    }
}

impl From<NumericButton> for KeypadPosition {
    fn from(button: NumericButton) -> Self {
        let row = match button {
            NumericButton::Number(7..=9) => 0,
            NumericButton::Number(4..=6) => 1,
            NumericButton::Number(1..=3) => 2,
            NumericButton::Number(0) | NumericButton::Confirm => 3,
            NumericButton::Number(num) => panic!("Invalid keypad number: {num}"),
        };

        let col = match button {
            NumericButton::Number(1 | 4 | 7) => 0,
            NumericButton::Number(0 | 2 | 5 | 8) => 1,
            NumericButton::Confirm | NumericButton::Number(3 | 6 | 9) => 2,
            NumericButton::Number(num) => panic!("Invalid keypad number: {num}"),
        };

        Self { row, col }
    }
}

impl TryFrom<char> for NumericButton {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0'..='9' => Ok(Self::Number(value as u8 - b'0')),
            'A' => Ok(Self::Confirm),
            _ => Err(format!("Invalid keypad: {value}")),
        }
    }
}

// DIRECTIONAL KEYPAD
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DirectionButton {
    Left,
    Right,
    Up,
    Down,
    Confirm,
}

impl TryFrom<KeypadPosition> for DirectionButton {
    type Error = ();

    fn try_from(pos: KeypadPosition) -> Result<Self, Self::Error> {
        match (pos.row, pos.col) {
            (0, 1) => Ok(Self::Up),
            (0, 2) => Ok(Self::Confirm),
            (1, 0) => Ok(Self::Left),
            (1, 1) => Ok(Self::Down),
            (1, 2) => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

impl From<DirectionButton> for KeypadPosition {
    fn from(button: DirectionButton) -> Self {
        let (row, col) = match button {
            DirectionButton::Up => (0, 1),
            DirectionButton::Confirm => (0, 2),
            DirectionButton::Left => (1, 0),
            DirectionButton::Down => (1, 1),
            DirectionButton::Right => (1, 2),
        };

        Self { row, col }
    }
}
