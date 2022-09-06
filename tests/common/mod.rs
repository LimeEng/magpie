use magpie::othello::{Bitboard, Board, OthelloError, Position, PositionError};
use quickcheck::{Arbitrary, Gen};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct ShadowBitboard(u64);

impl Arbitrary for ShadowBitboard {
    fn arbitrary(g: &mut Gen) -> Self {
        ShadowBitboard(u64::arbitrary(g))
    }
}

impl From<ShadowBitboard> for Bitboard {
    fn from(bitboard: ShadowBitboard) -> Self {
        Bitboard(bitboard.0)
    }
}

#[derive(Debug, Clone)]
pub struct ShadowPosition(u64);

impl Arbitrary for ShadowPosition {
    fn arbitrary(g: &mut Gen) -> Self {
        let bit = (u8::arbitrary(g) % 63) + 1;
        ShadowPosition(1 << bit)
    }
}

impl TryFrom<ShadowPosition> for Position {
    type Error = PositionError;

    fn try_from(pos: ShadowPosition) -> Result<Self, Self::Error> {
        Position::try_from(pos.0)
    }
}

#[derive(Debug, Clone)]
pub struct ShadowBoard {
    black_stones: u64,
    white_stones: u64,
}

impl Arbitrary for ShadowBoard {
    fn arbitrary(g: &mut Gen) -> Self {
        // Generate a random bitboard
        let bits = u64::arbitrary(g);

        let mut black_stones = 0;
        let mut white_stones = 0;

        // Iterate over all bits
        for i in 0..63 {
            // Extract the next bit
            let next_bit = (bits >> i) & 1;
            // Arbitrarily assign this bit to either black or white
            let assign_black = bool::arbitrary(g);
            if assign_black {
                black_stones |= next_bit << i;
            } else {
                white_stones |= next_bit << i;
            }
        }
        ShadowBoard::try_from((black_stones, white_stones)).unwrap()
    }
}

impl TryFrom<(u64, u64)> for ShadowBoard {
    type Error = OthelloError;

    fn try_from(stones: (u64, u64)) -> Result<Self, Self::Error> {
        let (black_stones, white_stones) = stones;
        if black_stones & white_stones != 0 {
            return Err(OthelloError::PiecesOverlapping);
        }
        let board = ShadowBoard {
            black_stones,
            white_stones,
        };
        Ok(board)
    }
}

impl TryFrom<ShadowBoard> for Board {
    type Error = OthelloError;

    fn try_from(board: ShadowBoard) -> Result<Self, Self::Error> {
        Board::try_from((board.black_stones, board.white_stones))
    }
}
