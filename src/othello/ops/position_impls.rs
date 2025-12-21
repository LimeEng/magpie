use crate::othello::{Bitboard, Position};
use std::{
    cmp::Ordering,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, Shr},
};

// =====[ Comparison with u64 ]=====

impl PartialEq<u64> for Position {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u64> for Position {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

// =====[ u64 OP Position ]=====

impl BitAnd<Position> for u64 {
    type Output = u64;
    fn bitand(self, rhs: Position) -> u64 {
        self & rhs.0
    }
}

impl BitOr<Position> for u64 {
    type Output = u64;
    fn bitor(self, rhs: Position) -> u64 {
        self | rhs.0
    }
}

impl BitXor<Position> for u64 {
    type Output = u64;
    fn bitxor(self, rhs: Position) -> u64 {
        self ^ rhs.0
    }
}

impl BitAndAssign<Position> for u64 {
    fn bitand_assign(&mut self, rhs: Position) {
        *self &= rhs.0;
    }
}

impl BitOrAssign<Position> for u64 {
    fn bitor_assign(&mut self, rhs: Position) {
        *self |= rhs.0;
    }
}

impl BitXorAssign<Position> for u64 {
    fn bitxor_assign(&mut self, rhs: Position) {
        *self ^= rhs.0;
    }
}

impl PartialEq<Position> for u64 {
    fn eq(&self, other: &Position) -> bool {
        *self == other.0
    }
}

impl PartialOrd<Position> for u64 {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

// =====[ Shift operations ]=====

macro_rules! impl_shifts {
    ($($t:ty),+) => {$(
        impl Shl<$t> for Position {
            type Output = Bitboard;
            fn shl(self, rhs: $t) -> Bitboard {
                Bitboard::from(self.0 << rhs)
            }
        }

        impl Shr<$t> for Position {
            type Output = Bitboard;
            fn shr(self, rhs: $t) -> Bitboard {
                Bitboard::from(self.0 >> rhs)
            }
        }
    )+};
}

impl_shifts!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
