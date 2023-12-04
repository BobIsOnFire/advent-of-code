use std::{
    cmp::Ordering,
    ops::{BitAnd, BitOr, RangeInclusive},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NumberRange {
    Empty,
    NonEmpty(i64, i64),
}

impl NumberRange {
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::NonEmpty(from, to) => (to - from) as usize,
        }
    }

    pub fn new(from: i64, to: i64) -> Self {
        if from <= to {
            Self::NonEmpty(from, to)
        } else {
            Self::Empty
        }
    }

    // Total ordering based on the range start. Empty ranges are always equal to each other and less than non-empty
    pub fn started_before(&self, rhs: &NumberRange) -> Ordering {
        match (self, rhs) {
            (Self::Empty, Self::Empty) => Ordering::Equal,
            (Self::Empty, _) => Ordering::Less,
            (_, Self::Empty) => Ordering::Greater,
            (Self::NonEmpty(my_from, _), Self::NonEmpty(other_from, _)) => i64::cmp(my_from, other_from),
        }
    }
}

impl BitAnd for NumberRange {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Empty, _) => rhs,
            (_, Self::Empty) => self,
            (Self::NonEmpty(my_from, my_to), Self::NonEmpty(other_from, other_to)) => {
                let from = Ord::max(my_from, other_from);
                let to = Ord::min(my_to, other_to);

                if from <= to {
                    Self::NonEmpty(from, to)
                } else {
                    Self::Empty
                }
            }
        }
    }
}

impl BitOr for NumberRange {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Empty, _) => rhs,
            (_, Self::Empty) => self,
            (Self::NonEmpty(my_from, my_to), Self::NonEmpty(other_from, other_to)) => {
                // FIXME: (2,3) | (5,6) will generate (2,6), which is not really correct
                Self::NonEmpty(Ord::min(my_from, other_from), Ord::max(my_to, other_to))
            }
        }
    }
}

impl From<NumberRange> for RangeInclusive<i64> {
    fn from(value: NumberRange) -> Self {
        match value {
            #[allow(clippy::reversed_empty_ranges)]
            NumberRange::Empty => 1..=0, // basic empty range
            NumberRange::NonEmpty(from, to) => from..=to,
        }
    }
}

impl IntoIterator for NumberRange {
    type Item = i64;
    type IntoIter = RangeInclusive<i64>;

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}

impl std::fmt::Display for NumberRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "[Empty]"),
            Self::NonEmpty(from, to) => write!(f, "[{},{}]", from, to),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NumberRange::*;

    #[test]
    fn full_contains() {
        assert_eq!(NonEmpty(2, 8) & NonEmpty(3, 7), NonEmpty(3, 7));
        assert_eq!(NonEmpty(3, 7) & NonEmpty(2, 8), NonEmpty(3, 7));
    }

    #[test]
    fn intersects() {
        assert_eq!(NonEmpty(2, 7) & NonEmpty(3, 8), NonEmpty(3, 7));
        assert_eq!(NonEmpty(3, 8) & NonEmpty(2, 7), NonEmpty(3, 7));
    }

    #[test]
    fn no_intersect() {
        assert_eq!(NonEmpty(2, 3) & NonEmpty(7, 8), Empty);
        assert_eq!(NonEmpty(7, 8) & NonEmpty(2, 3), Empty);
    }
}
