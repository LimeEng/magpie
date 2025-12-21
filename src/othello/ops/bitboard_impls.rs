use crate::othello::Bitboard;
use std::{
    cmp::Ordering,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
};

// =====[ Comparison with u64 ]=====

impl PartialEq<u64> for Bitboard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u64> for Bitboard {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

// =====[ Bitboard bitwise operations ]=====

// Bitboard OP Bitboard

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.0 ^= rhs.0;
    }
}

// Bitboard OP u64

impl BitAnd<u64> for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: u64) -> Bitboard {
        Bitboard(self.0 & rhs)
    }
}

impl BitOr<u64> for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: u64) -> Bitboard {
        Bitboard(self.0 | rhs)
    }
}

impl BitXor<u64> for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: u64) -> Bitboard {
        Bitboard(self.0 ^ rhs)
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs;
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitXorAssign<u64> for Bitboard {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.0 ^= rhs;
    }
}

// u64 OP Bitboard => u64

impl BitAnd<Bitboard> for u64 {
    type Output = u64;
    fn bitand(self, rhs: Bitboard) -> u64 {
        self & rhs.0
    }
}

impl BitOr<Bitboard> for u64 {
    type Output = u64;
    fn bitor(self, rhs: Bitboard) -> u64 {
        self | rhs.0
    }
}

impl BitXor<Bitboard> for u64 {
    type Output = u64;
    fn bitxor(self, rhs: Bitboard) -> u64 {
        self ^ rhs.0
    }
}

impl BitAndAssign<Bitboard> for u64 {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        *self &= rhs.0;
    }
}

impl BitOrAssign<Bitboard> for u64 {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        *self |= rhs.0;
    }
}

impl BitXorAssign<Bitboard> for u64 {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        *self ^= rhs.0;
    }
}

impl PartialEq<Bitboard> for u64 {
    fn eq(&self, other: &Bitboard) -> bool {
        *self == other.0
    }
}

impl PartialOrd<Bitboard> for u64 {
    fn partial_cmp(&self, other: &Bitboard) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

// =====[ Shift operations ]=====

macro_rules! impl_shifts {
    ($($t:ty),+) => {$(
        impl Shl<$t> for Bitboard {
            type Output = Bitboard;
            fn shl(self, rhs: $t) -> Bitboard {
                Bitboard(self.0 << rhs)
            }
        }

        impl Shr<$t> for Bitboard {
            type Output = Bitboard;
            fn shr(self, rhs: $t) -> Bitboard {
                Bitboard(self.0 >> rhs)
            }
        }

        impl ShlAssign<$t> for Bitboard {
            fn shl_assign(&mut self, rhs: $t) {
                self.0 <<= rhs;
            }
        }

        impl ShrAssign<$t> for Bitboard {
            fn shr_assign(&mut self, rhs: $t) {
                self.0 >>= rhs;
            }
        }
    )+};
}

impl_shifts!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
