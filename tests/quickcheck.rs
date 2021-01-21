use lazy_static::lazy_static;
use magpie::othello::{OthelloBoard, OthelloError, SquareExt, Stone, StoneExt};
use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;
use std::{convert::TryFrom, iter::successors};

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
        .moves_for(stone)
        .stones()
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

    let legal_positions = board.moves_for(stone);

    let failed = MASKS
        .iter()
        .filter(|pos| *pos & legal_positions == 0)
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
        .moves_for(stone)
        .stones()
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

    let legal_positions = board.moves_for(stone);

    let failed = MASKS
        .iter()
        .filter(|pos| *pos & legal_positions == 0)
        .map(|pos| board.is_legal_move(stone, *pos))
        .any(|result| result);

    assert!(!failed);
}

#[quickcheck]
fn bits_should_be_consistent(board: ShadowOthelloBoard) {
    // Check that black and white stones do not overlap with each other or the
    // empty set
    let board = OthelloBoard::try_from(board).unwrap();

    let black = board.bits_for(Stone::Black);
    let white = board.bits_for(Stone::White);
    let empty = board.empty_squares();

    assert!(black & white == 0);
    assert!((black | white) & empty == 0);
}

#[quickcheck]
fn stone_at_consistency(board: ShadowOthelloBoard, rand_pos: u64) {
    // Check that stone_at returns the correct stones
    let board = OthelloBoard::try_from(board).unwrap();

    let black = board.bits_for(Stone::Black);
    let white = board.bits_for(Stone::White);
    let empty = board.empty_squares();

    // Test all board positions and one random element, that may have multiple
    // bits set
    let mut positions = MASKS.iter().chain(std::iter::once(&rand_pos));

    let success = positions.all(|pos| {
        board
            .stone_at(*pos)
            .map(|stone| match stone {
                Stone::Black => pos & black != 0,
                Stone::White => pos & white != 0,
            })
            .unwrap_or_else(|| {
                if pos.count_ones() != 1 {
                    // If the position has multiple bits set, stone_at should
                    // return None
                    true
                } else {
                    pos & empty != 0
                }
            })
    });

    assert!(success);
}

#[quickcheck]
fn squares_bit_count(rand_bitboard: u64) {
    let bit_at = |index: usize| rand_bitboard & (1 << index);
    let success = rand_bitboard
        .squares()
        .enumerate()
        .all(|(index, pos)| bit_at(63 - index) == pos);

    assert!(success);
}

#[quickcheck]
fn stones_bit_count(rand_bitboard: u64) {
    let expected = rand_bitboard.count_ones();
    let result = rand_bitboard.stones().filter(|pos| *pos != 0).count();

    assert_eq!(expected as usize, result);
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
