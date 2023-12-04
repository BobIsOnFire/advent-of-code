use std::ops::{BitAnd, BitAndAssign};

#[derive(Copy, Clone)]
pub struct BitSet {
    mask: u64,
}

impl BitSet {
    pub fn insert(&mut self, key: u64) {
        self.mask |= 1u64 << key;
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> {
        let mask = self.mask;
        (0..u64::BITS.into()).filter(move |key| (1u64 << key) & mask != 0)
    }

    pub fn is_empty(&self) -> bool {
        self.mask == 0
    }

    pub fn len(&self) -> usize {
        self.mask.count_ones() as usize
    }
}

impl BitAnd for BitSet {
    type Output = BitSet;

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
