use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseType {
    Shape,
    Outcome,
}

impl Display for ParseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let describe = match self {
            Self::Shape => "shape (Rock, Paper or Scissors)",
            Self::Outcome => "match outcome (Win, Lose or Draw)",
        };

        write!(f, "Error while parsing {}", describe)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTurnError {
    parse_type: ParseType,
    ch: char,
}

impl Display for ParseTurnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parse_type)
    }
}

impl std::error::Error for ParseTurnError {}

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn wins(&self, other: &Self) -> bool {
        *other == self.beats()
    }

    fn loses(&self, other: &Self) -> bool {
        *other == self.loses_to()
    }
}

pub struct OpponentTurn {
    shape: Shape,
}

impl OpponentTurn {
    fn new(shape: Shape) -> Self {
        Self { shape }
    }
}

impl TryFrom<char> for OpponentTurn {
    type Error = ParseTurnError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::new(Shape::Rock)),
            'B' => Ok(Self::new(Shape::Paper)),
            'C' => Ok(Self::new(Shape::Scissors)),
            ch => Err(Self::Error { ch, parse_type: ParseType::Shape }),
        }
    }
}

pub struct UserTurn {
    shape: Shape,
}

impl UserTurn {
    fn new(shape: Shape) -> Self {
        Self { shape }
    }

    fn from_opponent_and_outcome(opponent: &OpponentTurn, outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Lose => Self::new(opponent.shape.beats()),
            Outcome::Draw => Self::new(opponent.shape),
            Outcome::Win => Self::new(opponent.shape.loses_to()),
        }
    }
}

impl TryFrom<char> for UserTurn {
    type Error = ParseTurnError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::new(Shape::Rock)),
            'Y' => Ok(Self::new(Shape::Paper)),
            'Z' => Ok(Self::new(Shape::Scissors)),
            ch => Err(Self::Error { ch, parse_type: ParseType::Shape }),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_turns(user: &UserTurn, opponent: &OpponentTurn) -> Self {
        if user.shape.wins(&opponent.shape) {
            Self::Win
        } else if user.shape.loses(&opponent.shape) {
            Self::Lose
        } else {
            Self::Draw
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = ParseTurnError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            ch => Err(Self::Error { ch, parse_type: ParseType::Outcome }),
        }
    }
}

pub struct Match {
    user_turn: UserTurn,
    outcome: Outcome,
}

impl Match {
    pub fn from_turns(opponent_turn: OpponentTurn, user_turn: UserTurn) -> Self {
        let outcome = Outcome::from_turns(&user_turn, &opponent_turn);
        Self { user_turn, outcome }
    }

    pub fn from_opponent_and_outcome(opponent_turn: OpponentTurn, outcome: Outcome) -> Self {
        let user_turn = UserTurn::from_opponent_and_outcome(&opponent_turn, &outcome);
        Self { user_turn, outcome }
    }

    pub fn score(&self) -> usize {
        let user_score = match self.user_turn.shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        let outcome_score = match self.outcome {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };

        user_score + outcome_score
    }
}
