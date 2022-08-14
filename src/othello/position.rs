use crate::othello::{
    bitboard::Bitboard,
    constants::{FILES, MASKS, POSITIONS_AS_NOTATION, RANKS},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Position {
    bitboard: Bitboard,
}

impl Position {
    fn new_unchecked(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn rank(&self) -> u8 {
        (self.bitboard.raw().leading_zeros() / 8)
            .try_into()
            .unwrap()
    }

    pub fn file(&self) -> u8 {
        (self.bitboard.raw().leading_zeros() % 8)
            .try_into()
            .unwrap()
    }

    pub fn as_bitboard(&self) -> u64 {
        self.bitboard.raw()
    }

    pub fn to_notation(&self) -> String {
        POSITIONS_AS_NOTATION[self.bitboard.raw().leading_zeros() as usize].to_string()
    }
}

impl From<Bitboard> for Position {
    fn from(bitboard: Bitboard) -> Self {
        Position::new_unchecked(bitboard)
    }
}

impl From<Bitboard> for PositionSet {
    fn from(bitboard: Bitboard) -> Self {
        PositionSet { bitboard }
    }
}

impl From<Position> for Bitboard {
    fn from(position: Position) -> Self {
        position.bitboard
    }
}

impl From<PositionSet> for Bitboard {
    fn from(set: PositionSet) -> Self {
        set.bitboard
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
pub struct PositionSet {
    bitboard: Bitboard,
}

impl ExactSizeIterator for PositionSet {
    fn len(&self) -> usize {
        self.bitboard.count_set() as usize
    }
}

impl Iterator for PositionSet {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        if self.bitboard.is_empty() {
            return None;
        }

        let position: u64 = 1 << self.bitboard.raw().trailing_zeros();
        let position: Bitboard = position.into();
        self.bitboard ^= position;

        Some(Position::new_unchecked(position))
    }
}
