use crate::othello::{Bitboard, Position};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
};
macro_rules! shl_impl {
    ($wrapped:ty, $num:ty) => {
        impl Shl<$num> for $wrapped {
            type Output = $wrapped;

            fn shl(self, other: $num) -> Self::Output {
                Self(self.raw() << other)
            }
        }

        impl Shl<$wrapped> for $num {
            type Output = $num;

            fn shl(self, other: $wrapped) -> Self::Output {
                self << other.raw()
            }
        }
    };
}

macro_rules! shr_impl {
    ($wrapped:ty, $num:ty) => {
        impl Shr<$num> for $wrapped {
            type Output = $wrapped;

            fn shr(self, other: $num) -> Self::Output {
                Self(self.raw() >> other)
            }
        }

        impl Shr<$wrapped> for $num {
            type Output = $num;

            fn shr(self, other: $wrapped) -> Self::Output {
                self >> other.raw()
            }
        }
    };
}

macro_rules! shl_assign_impl {
    ($wrapped:ty, $num:ty) => {
        impl ShlAssign<$num> for $wrapped {
            fn shl_assign(&mut self, rhs: $num) {
                self.0 <<= rhs;
            }
        }

        impl ShlAssign<$wrapped> for $num {
            fn shl_assign(&mut self, rhs: $wrapped) {
                *self <<= rhs.0;
            }
        }
    };
}

macro_rules! shr_assign_impl {
    ($wrapped:ty, $num:ty) => {
        impl ShrAssign<$num> for $wrapped {
            fn shr_assign(&mut self, rhs: $num) {
                self.0 >>= rhs;
            }
        }

        impl ShrAssign<$wrapped> for $num {
            fn shr_assign(&mut self, rhs: $wrapped) {
                *self >>= rhs.0;
            }
        }
    };
}

macro_rules! bitshift_impl_all {
    ($($t:ty)*) => ($(
        shl_impl! { Bitboard, $t }
        shl_assign_impl! { Bitboard, $t }
        shr_impl! { Bitboard, $t }
        shr_assign_impl! { Bitboard, $t }
        shl_impl! { Position, $t }
        shl_assign_impl! { Position, $t }
        shr_impl! { Position, $t }
        shr_assign_impl! { Position, $t }
    )*)
}

macro_rules! not_impl {
    ($wrapped:ty) => {
        impl Not for $wrapped {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }
    };
}

macro_rules! and_impl {
    ($wrapped:ty) => {
        impl BitAnd<u64> for $wrapped {
            type Output = Self;

            fn bitand(self, rhs: u64) -> Self::Output {
                Self(self.0 & rhs)
            }
        }

        impl BitAndAssign<u64> for $wrapped {
            fn bitand_assign(&mut self, rhs: u64) {
                self.0 &= rhs;
            }
        }

        impl BitAnd<$wrapped> for u64 {
            type Output = Self;

            fn bitand(self, rhs: $wrapped) -> Self::Output {
                self & rhs.0
            }
        }

        impl BitAndAssign<$wrapped> for u64 {
            fn bitand_assign(&mut self, rhs: $wrapped) {
                *self &= rhs.0;
            }
        }

        impl BitAnd for $wrapped {
            type Output = Self;

            fn bitand(self, rhs: $wrapped) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl BitAndAssign for $wrapped {
            fn bitand_assign(&mut self, rhs: $wrapped) {
                self.0 &= rhs.0;
            }
        }
    };
}

macro_rules! or_impl {
    ($wrapped:ty) => {
        impl BitOr<u64> for $wrapped {
            type Output = Self;

            fn bitor(self, rhs: u64) -> Self::Output {
                Self(self.0 | rhs)
            }
        }

        impl BitOrAssign<u64> for $wrapped {
            fn bitor_assign(&mut self, rhs: u64) {
                self.0 |= rhs;
            }
        }

        impl BitOr<$wrapped> for u64 {
            type Output = Self;

            fn bitor(self, rhs: $wrapped) -> Self::Output {
                self | rhs.0
            }
        }

        impl BitOrAssign<$wrapped> for u64 {
            fn bitor_assign(&mut self, rhs: $wrapped) {
                *self |= rhs.0;
            }
        }

        impl BitOr for $wrapped {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $wrapped {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
    };
}

macro_rules! xor_impl {
    ($wrapped:ty) => {
        impl BitXor<u64> for $wrapped {
            type Output = Self;

            fn bitxor(self, rhs: u64) -> Self::Output {
                Self(self.0 ^ rhs)
            }
        }

        impl BitXorAssign<u64> for $wrapped {
            fn bitxor_assign(&mut self, rhs: u64) {
                self.0 ^= rhs;
            }
        }

        impl BitXor<$wrapped> for u64 {
            type Output = Self;

            fn bitxor(self, rhs: $wrapped) -> Self::Output {
                self ^ rhs.0
            }
        }

        impl BitXorAssign<$wrapped> for u64 {
            fn bitxor_assign(&mut self, rhs: $wrapped) {
                *self ^= rhs.0;
            }
        }

        impl BitXor for $wrapped {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl BitXorAssign for $wrapped {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }
    };
}

macro_rules! bitwise_impl_all {
    () => {
        not_impl! {Bitboard}
        and_impl! {Bitboard}
        or_impl! {Bitboard}
        xor_impl! {Bitboard}
    };
}

macro_rules! eq_hash_impl {
    ($wrapped:ty) => {
        impl Eq for $wrapped {}

        impl Hash for $wrapped {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl PartialEq for $wrapped {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl PartialEq<u64> for $wrapped {
            fn eq(&self, other: &u64) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<$wrapped> for u64 {
            fn eq(&self, other: &$wrapped) -> bool {
                *self == other.0
            }
        }
    };
}

macro_rules! ord_impl {
    ($wrapped:ty) => {
        impl Ord for $wrapped {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl PartialOrd for $wrapped {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl PartialOrd<u64> for $wrapped {
            fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl PartialOrd<$wrapped> for u64 {
            fn partial_cmp(&self, other: &$wrapped) -> Option<Ordering> {
                self.partial_cmp(&other.0)
            }
        }
    };
}

macro_rules! compare_impl_all {
    () => {
        eq_hash_impl! {Bitboard}
        ord_impl! {Bitboard}
        eq_hash_impl! {Position}
        ord_impl! {Position}
    };
}

bitshift_impl_all! {u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize}
bitwise_impl_all! {}
compare_impl_all! {}

impl BitAnd<Bitboard> for Position {
    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAnd<Position> for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Position) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign<Bitboard> for Position {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOr<Bitboard> for Position {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOr<Position> for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Position) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign<Position> for Bitboard {
    fn bitor_assign(&mut self, rhs: Position) {
        self.0 |= rhs.0;
    }
}

impl BitXor<Bitboard> for Position {
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXor<Position> for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Position) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign<Position> for Bitboard {
    fn bitxor_assign(&mut self, rhs: Position) {
        self.0 ^= rhs.0;
    }
}
