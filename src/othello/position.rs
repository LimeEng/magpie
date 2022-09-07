use crate::othello::{
    constants::{FILES, POSITIONS, POSITIONS_AS_NOTATION, RANKS},
    Bitboard,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Position(pub u64);

impl Position {
    pub(crate) fn new_unchecked(bitboard: u64) -> Self {
        Self(bitboard)
    }

    pub fn raw(self) -> u64 {
        self.0
    }

    pub fn rank(self) -> u8 {
        (self.0.leading_zeros() / 8).try_into().unwrap()
    }

    pub fn file(self) -> u8 {
        (self.0.leading_zeros() % 8).try_into().unwrap()
    }

    pub fn to_notation(self) -> String {
        POSITIONS_AS_NOTATION[self.0.leading_zeros() as usize].to_string()
    }
}

impl From<Position> for Bitboard {
    fn from(position: Position) -> Self {
        Bitboard::from(position.0)
    }
}

impl From<Position> for u64 {
    fn from(position: Position) -> Self {
        position.0
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
            Ok(Position::new_unchecked(bitboard))
        }
    }
}

impl TryFrom<String> for Position {
    type Error = PositionError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        Position::try_from(text.as_ref())
    }
}

impl TryFrom<&str> for Position {
    type Error = PositionError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let text = text.to_lowercase();
        let bitboard = POSITIONS_AS_NOTATION
            .iter()
            .position(|position| position == &text)
            .map(|index| POSITIONS[index])
            .ok_or(PositionError::InvalidPosition)?;
        Ok(Position::new_unchecked(bitboard))
    }
}

impl TryFrom<u64> for Position {
    type Error = PositionError;

    fn try_from(bitboard: u64) -> Result<Self, Self::Error> {
        if bitboard.count_ones() == 1 {
            Ok(Position::new_unchecked(bitboard))
        } else {
            Err(PositionError::MultipleBitsSet)
        }
    }
}

impl TryFrom<Bitboard> for Position {
    type Error = PositionError;

    fn try_from(bitboard: Bitboard) -> Result<Self, Self::Error> {
        if bitboard.count_set() == 1 {
            Ok(Position::new_unchecked(bitboard.0))
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
