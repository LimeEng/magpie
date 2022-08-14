use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Bitboard(u64);

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

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

macro_rules! shl_impl {
    ($t:ty) => {
        impl Shl<$t> for Bitboard {
            type Output = Self;

            fn shl(self, other: $t) -> Self::Output {
                Self(self.0 << other)
            }
        }

        impl Shl<Bitboard> for $t {
            type Output = $t;

            fn shl(self, other: Bitboard) -> Self::Output {
                self << other.0
            }
        }
    };
}

macro_rules! shl_assign_impl {
    ($t:ty) => {
        impl ShlAssign<$t> for Bitboard {
            fn shl_assign(&mut self, rhs: $t) {
                self.0 <<= rhs;
            }
        }

        impl ShlAssign<Bitboard> for $t {
            fn shl_assign(&mut self, rhs: Bitboard) {
                *self <<= rhs.0;
            }
        }
    };
}

macro_rules! shr_impl {
    ($t:ty) => {
        impl Shr<$t> for Bitboard {
            type Output = Self;

            fn shr(self, other: $t) -> Self::Output {
                Self(self.0 >> other)
            }
        }

        impl Shr<Bitboard> for $t {
            type Output = $t;

            fn shr(self, other: Bitboard) -> Self::Output {
                self >> other.0
            }
        }
    };
}

macro_rules! shr_assign_impl {
    ($t:ty) => {
        impl ShrAssign<$t> for Bitboard {
            fn shr_assign(&mut self, rhs: $t) {
                self.0 >>= rhs;
            }
        }

        impl ShrAssign<Bitboard> for $t {
            fn shr_assign(&mut self, rhs: Bitboard) {
                *self >>= rhs.0;
            }
        }
    };
}

macro_rules! bit_impl_all {
    ($($t:ty)*) => ($(
        shl_impl! { $t }
        shl_assign_impl! { $t }
        shr_impl! { $t }
        shr_assign_impl! { $t }
    )*)
}

bit_impl_all! {u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
