use std::ops::BitAnd;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NumberRange {
    Empty,
    NonEmpty(i64, i64),
}

impl NumberRange {
    pub fn is_empty(self) -> bool {
        self == Self::Empty
    }

    pub fn new(from: i64, to: i64) -> Self {
        Self::NonEmpty(from, to)
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
