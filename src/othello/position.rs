use crate::othello::{
    bitboard::Bitboard,
    constants::{FILES, MASKS, POSITIONS_AS_NOTATION, RANKS},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Position(Bitboard);

impl Position {
    fn new_unchecked(bitboard: Bitboard) -> Self {
        Self(bitboard)
    }

    pub fn rank(&self) -> u8 {
        (self.0.raw().leading_zeros() / 8).try_into().unwrap()
    }

    pub fn file(&self) -> u8 {
        (self.0.raw().leading_zeros() % 8).try_into().unwrap()
    }

    pub fn as_bitboard(&self) -> Bitboard {
        self.0
    }

    pub fn to_notation(&self) -> String {
        POSITIONS_AS_NOTATION[self.0.raw().leading_zeros() as usize].to_string()
    }
}

impl From<Bitboard> for Position {
    fn from(bitboard: Bitboard) -> Self {
        Position(bitboard)
    }
}

impl From<Bitboard> for PositionSet {
    fn from(bitboard: Bitboard) -> Self {
        PositionSet(bitboard)
    }
}

impl From<Position> for Bitboard {
    fn from(position: Position) -> Self {
        position.0
    }
}

impl From<PositionSet> for Bitboard {
    fn from(set: PositionSet) -> Self {
        set.0
    }
}

impl TryFrom<(u8, u8)> for Position {
    type Error = PositionError;

    fn try_from(pair: (u8, u8)) -> Result<Self, Self::Error> {
        let (rank, file) = pair;
        if rank > 7 || file > 7 {
            Err(PositionError::InvalidPosition)
        } else {
            let bitboard = RANKS[rank as usize] & FILES[file as usize];
            Ok(Position::new_unchecked(bitboard.into()))
        }
    }
}

impl TryFrom<String> for Position {
    type Error = PositionError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        let text = text.to_lowercase();
        let bitboard = POSITIONS_AS_NOTATION
            .iter()
            .position(|position| position == &text)
            .map(|index| MASKS[index])
            .ok_or(PositionError::InvalidPosition)?;
        Ok(Position::new_unchecked(bitboard.into()))
    }
}

impl TryFrom<u64> for Position {
    type Error = PositionError;

    fn try_from(bitboard: u64) -> Result<Self, Self::Error> {
        if bitboard.count_ones() == 1 {
            Ok(Position::new_unchecked(bitboard.into()))
        } else {
            Err(PositionError::MultipleBitsSet)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum PositionError {
    MultipleBitsSet,
    InvalidPosition,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PositionSet(Bitboard);

impl ExactSizeIterator for PositionSet {
    fn len(&self) -> usize {
        self.0.count_set() as usize
    }
}

impl Iterator for PositionSet {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        if self.0.is_empty() {
            return None;
        }

        let position: u64 = 1 << self.0.raw().trailing_zeros();
        let position: Bitboard = position.into();
        self.0 ^= position;

        Some(Position::new_unchecked(position))
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
