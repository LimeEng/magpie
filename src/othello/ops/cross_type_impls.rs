use crate::othello::{Bitboard, Position};
use std::{
    cmp::Ordering,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign},
};

// =====[ Cross-type comparisons ]=====

impl PartialEq<Position> for Bitboard {
    fn eq(&self, other: &Position) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Bitboard> for Position {
    fn eq(&self, other: &Bitboard) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<Position> for Bitboard {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialOrd<Bitboard> for Position {
    fn partial_cmp(&self, other: &Bitboard) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

// =====[ Position OP Position => Bitboard (singleton sets promote to set) ]=====

impl BitAnd<Position> for Position {
    type Output = Bitboard;
    fn bitand(self, rhs: Position) -> Bitboard {
        Bitboard::from(self.0 & rhs.0)
    }
}

impl BitOr<Position> for Position {
    type Output = Bitboard;
    fn bitor(self, rhs: Position) -> Bitboard {
        Bitboard::from(self.0 | rhs.0)
    }
}

impl BitXor<Position> for Position {
    type Output = Bitboard;
    fn bitxor(self, rhs: Position) -> Bitboard {
        Bitboard::from(self.0 ^ rhs.0)
    }
}

// =====[ Bitboard OP Position => Bitboard (set operations with singleton) ]=====

impl BitAnd<Position> for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Position) -> Bitboard {
        Bitboard::from(self.0 & rhs.0)
    }
}

impl BitOr<Position> for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Position) -> Bitboard {
        Bitboard::from(self.0 | rhs.0)
    }
}

impl BitXor<Position> for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Position) -> Bitboard {
        Bitboard::from(self.0 ^ rhs.0)
    }
}

impl BitAndAssign<Position> for Bitboard {
    fn bitand_assign(&mut self, rhs: Position) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign<Position> for Bitboard {
    fn bitor_assign(&mut self, rhs: Position) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign<Position> for Bitboard {
    fn bitxor_assign(&mut self, rhs: Position) {
        self.0 ^= rhs.0;
    }
}

// =====[ Position OP Bitboard => Bitboard (symmetric operations) ]=====

impl BitAnd<Bitboard> for Position {
    type Output = Bitboard;
    fn bitand(self, rhs: Bitboard) -> Bitboard {
        Bitboard::from(self.0 & rhs.0)
    }
}

impl BitOr<Bitboard> for Position {
    type Output = Bitboard;
    fn bitor(self, rhs: Bitboard) -> Bitboard {
        Bitboard::from(self.0 | rhs.0)
    }
}

impl BitXor<Bitboard> for Position {
    type Output = Bitboard;
    fn bitxor(self, rhs: Bitboard) -> Bitboard {
        Bitboard::from(self.0 ^ rhs.0)
    }
}
