use crate::othello::Position;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Bitboard(pub(crate) u64);

impl Bitboard {
    pub fn raw(self) -> u64 {
        self.0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn count_set(self) -> u8 {
        self.0.count_ones().try_into().unwrap()
    }

    pub fn count_empty(self) -> u8 {
        self.0.count_zeros().try_into().unwrap()
    }

    pub fn is_power_of_two(self) -> bool {
        self.count_set() == 1
    }

    pub fn bits(self) -> BitsIntoIterator {
        let bits = Bits {
            remaining: 64,
            bitboard: self,
        };
        bits.into_iter()
    }

    pub fn hot_bits(self) -> HotBitsIntoIterator {
        let positions = HotBits {
            remaining: self.count_set(),
            bitboard: self,
        };
        positions.into_iter()
    }
}

#[derive(Clone, Debug)]
pub struct Bits {
    remaining: usize,
    bitboard: Bitboard,
}

#[derive(Clone, Debug)]
pub struct BitsIntoIterator {
    remaining: usize,
    bitboard: Bitboard,
}

impl IntoIterator for Bits {
    type Item = Bitboard;
    type IntoIter = BitsIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitsIntoIterator {
            remaining: self.remaining,
            bitboard: self.bitboard,
        }
    }
}

impl Iterator for BitsIntoIterator {
    type Item = Bitboard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let mask = 1u64 << (self.remaining - 1);
            let bit = self.bitboard & mask;
            self.remaining -= 1;
            Some(bit)
        }
    }
}

impl ExactSizeIterator for BitsIntoIterator {
    fn len(&self) -> usize {
        self.remaining
    }
}

#[derive(Clone, Debug)]
pub struct HotBits {
    remaining: u8,
    bitboard: Bitboard,
}

#[derive(Clone, Debug)]
pub struct HotBitsIntoIterator {
    remaining: u8,
    bitboard: Bitboard,
}

impl IntoIterator for HotBits {
    type Item = Position;
    type IntoIter = HotBitsIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        HotBitsIntoIterator {
            remaining: self.remaining,
            bitboard: self.bitboard,
        }
    }
}

impl Iterator for HotBitsIntoIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bitboard.is_empty() {
            None
        } else {
            self.remaining -= 1;
            let position = 1 << (63 - self.bitboard.raw().leading_zeros());
            self.bitboard ^= position;

            Some(Position::new_unchecked(position))
        }
    }
}

impl ExactSizeIterator for HotBitsIntoIterator {
    fn len(&self) -> usize {
        self.remaining.into()
    }
}

impl From<u64> for Bitboard {
    fn from(bitboard: u64) -> Self {
        Bitboard(bitboard)
    }
}

impl From<Bitboard> for u64 {
    fn from(bitboard: Bitboard) -> Self {
        bitboard.0
    }
}
