use super::constants::{FILES, MASKS, POSITIONS_AS_NOTATION, RANKS};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    bitboard: u64,
}

impl Position {
    fn new(bitboard: u64) -> Self {
        Self { bitboard }
    }

    pub fn rank(&self) -> u8 {
        (self.bitboard.leading_zeros() / 8).try_into().unwrap()
    }

    pub fn file(&self) -> u8 {
        (self.bitboard.leading_zeros() % 8).try_into().unwrap()
    }

    pub fn as_bitboard(&self) -> u64 {
        self.bitboard
    }

    pub fn to_notation(&self) -> String {
        POSITIONS_AS_NOTATION[self.bitboard.leading_zeros() as usize].to_string()
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
            Ok(Position::new(bitboard))
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
        Ok(Position::new(bitboard))
    }
}

impl TryFrom<u64> for Position {
    type Error = PositionError;

    fn try_from(bitboard: u64) -> Result<Self, Self::Error> {
        if bitboard.count_ones() == 1 {
            Ok(Position::new(bitboard))
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
