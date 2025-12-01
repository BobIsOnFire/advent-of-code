use std::ops::{BitAnd, BitAndAssign};

#[derive(Copy, Clone)]
pub struct BitSet {
    mask: u64,
}

impl BitSet {
    #[must_use]
    pub const fn new() -> Self {
        Self { mask: 0 }
    }

    pub const fn insert(&mut self, key: u64) {
        self.mask |= 1u64 << key;
    }

    #[must_use]
    pub const fn contains(&self, key: u64) -> bool {
        (1u64 << key) & self.mask != 0
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> {
        let mask = self.mask;
        (0..u64::BITS.into()).filter(move |key| (1u64 << key) & mask != 0)
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.mask == 0
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.mask.count_ones() as usize
    }
}

impl Default for BitSet {
    fn default() -> Self {
        Self::new()
    }
}

impl BitAnd for BitSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self { mask: self.mask & rhs.mask }
    }
}

impl BitAndAssign for BitSet {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl FromIterator<u64> for BitSet {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        let mut bitset = Self { mask: 0 };
        iter.into_iter().for_each(|num| bitset.insert(num));
        bitset
    }
}
