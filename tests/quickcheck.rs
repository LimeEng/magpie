use lazy_static::lazy_static;
use magpie::othello::{OthelloBoard, OthelloError, PositionExt, Stone};
use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;
use std::{collections::HashSet, convert::TryFrom, iter::successors};

lazy_static! {
    static ref MASKS: Vec<u64> = {
        successors(Some(1_u64), |n| {
            if *n == 1_u64 << 63 {
                None
            } else {
                Some(n << 1)
            }
        })
        .collect()
    };
}

#[quickcheck]
fn legal_moves_should_place(board: ShadowOthelloBoard) {
    // Check so that all legal moves returned can actually be placed
    let board = OthelloBoard::try_from(board).unwrap();
    let stone = Stone::Black;

    let result = board
        .legal_moves_for(stone)
        .positions()
        .map(|pos| board.clone().place_stone(stone, pos))
        .all(|result| result.is_ok());
    assert!(result);
}

#[quickcheck]
fn illegal_moves_should_not_place(board: ShadowOthelloBoard) {
    // Check so that all moves not contained in the set of legal moves
    // cannot actually be placed
    let board = OthelloBoard::try_from(board).unwrap();
    let stone = Stone::Black;

    let legal_positions: HashSet<u64> = board.legal_moves_for(stone).positions().collect();

    let failed = MASKS
        .iter()
        .filter(|pos| !legal_positions.contains(pos))
        .map(|pos| board.clone().place_stone(stone, *pos))
        .any(|result| result.is_ok());

    assert!(!failed);
}

#[quickcheck]
fn legal_moves_should_be_legal(board: ShadowOthelloBoard) {
    // Check so that all legal moves returned can be individually verified as legal
    let board = OthelloBoard::try_from(board).unwrap();
    let stone = Stone::Black;

    let result = board
        .legal_moves_for(stone)
        .positions()
        .map(|pos| board.is_legal_move(stone, pos))
        .all(|result| result);
    assert!(result);
}

#[quickcheck]
fn illegal_moves_should_be_illegal(board: ShadowOthelloBoard) {
    // Check so that all moves not contained in the set of legal moves
    // actually is illegal
    let board = OthelloBoard::try_from(board).unwrap();
    let stone = Stone::Black;

    let legal_positions: HashSet<u64> = board.legal_moves_for(stone).positions().collect();

    let failed = MASKS
        .iter()
        .filter(|pos| !legal_positions.contains(pos))
        .map(|pos| board.is_legal_move(stone, *pos))
        .any(|result| result);

    assert!(!failed);
}

#[derive(Debug, Clone)]
struct ShadowOthelloBoard {
    black_stones: u64,
    white_stones: u64,
}

impl Arbitrary for ShadowOthelloBoard {
    fn arbitrary(g: &mut Gen) -> ShadowOthelloBoard {
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
        ShadowOthelloBoard::try_from((black_stones, white_stones)).unwrap()
    }
}

impl TryFrom<(u64, u64)> for ShadowOthelloBoard {
    type Error = OthelloError;

    fn try_from(stones: (u64, u64)) -> Result<Self, Self::Error> {
        let (black_stones, white_stones) = stones;
        if black_stones & white_stones != 0 {
            return Err(OthelloError::PiecesOverlapping);
        }
        let board = ShadowOthelloBoard {
            black_stones,
            white_stones,
        };
        Ok(board)
    }
}

impl TryFrom<ShadowOthelloBoard> for OthelloBoard {
    type Error = OthelloError;

    fn try_from(board: ShadowOthelloBoard) -> Result<Self, Self::Error> {
        OthelloBoard::try_from((board.black_stones, board.white_stones))
    }
}