use magpie::othello::{Bitboard, Board, Position, Stone};
use quickcheck_macros::quickcheck;
use std::convert::TryFrom;

mod common;

use common::{ShadowBitboard, ShadowBoard};

#[quickcheck]
fn legal_moves_should_place(board: ShadowBoard) {
    // Check so that all legal moves returned can actually be placed
    let board = Board::try_from(board).unwrap();
    let stone = Stone::Black;

    let result = board
        .moves_for(stone)
        .hot_bits()
        .map(|pos| board.clone().play(stone, pos))
        .all(|result| result.is_ok());
    assert!(result);
}

#[quickcheck]
fn illegal_moves_should_not_place(board: ShadowBoard) {
    // Check so that all moves not contained in the set of legal moves
    // cannot actually be placed
    let board = Board::try_from(board).unwrap();
    let stone = Stone::Black;

    let legal_positions = board.moves_for(stone);

    let failed = Bitboard::from(u64::MAX)
        .bits()
        .filter(|pos| *pos & legal_positions == 0)
        .filter_map(|pos| Position::try_from(pos).ok())
        .map(|pos| board.clone().play(stone, pos))
        .any(|result| result.is_ok());

    assert!(!failed);
}

#[quickcheck]
fn legal_moves_should_be_legal(board: ShadowBoard) {
    // Check so that all legal moves returned can be individually verified as legal
    let board = Board::try_from(board).unwrap();
    let stone = Stone::Black;

    let result = board
        .moves_for(stone)
        .hot_bits()
        .map(|pos| board.is_legal_move(stone, pos))
        .all(|result| result);
    assert!(result);
}

#[quickcheck]
fn illegal_moves_should_be_illegal(board: ShadowBoard) {
    // Check so that all moves not contained in the set of legal moves
    // actually is illegal
    let board = Board::try_from(board).unwrap();
    let stone = Stone::Black;

    let legal_positions = board.moves_for(stone);

    let failed = Bitboard::from(u64::MAX)
        .bits()
        .filter(|pos| *pos & legal_positions == 0)
        .filter_map(|pos| Position::try_from(pos).ok())
        .map(|pos| board.is_legal_move(stone, pos))
        .any(|result| result);

    assert!(!failed);
}

#[quickcheck]
fn bits_should_be_consistent(board: ShadowBoard) {
    // Check that black and white stones do not overlap with each other or the
    // empty set
    let board = Board::try_from(board).unwrap();

    let black = board.bits_for(Stone::Black);
    let white = board.bits_for(Stone::White);
    let empty = board.empty_squares();

    assert!(black & white == 0);
    assert!((black | white) & empty == 0);
}

#[quickcheck]
fn stone_at_consistency(board: ShadowBoard) {
    // Check that stone_at returns the correct stones
    let board = Board::try_from(board).unwrap();

    let black = board.bits_for(Stone::Black);
    let white = board.bits_for(Stone::White);
    let empty = board.empty_squares();

    let success = Bitboard::from(u64::MAX)
        .bits()
        .filter_map(|pos| Position::try_from(pos).ok())
        .all(|pos| {
            board
                .stone_at(pos)
                .map(|stone| match stone {
                    Stone::Black => pos & black != 0,
                    Stone::White => pos & white != 0,
                })
                .unwrap_or_else(|| pos & empty != 0)
        });

    assert!(success);
}

#[quickcheck]
fn squares_bit_count(rand_bitboard: ShadowBitboard) {
    let rand_bitboard = Bitboard::from(rand_bitboard);
    let bit_at = |index: usize| rand_bitboard & (1 << index);
    let success = rand_bitboard
        .bits()
        .enumerate()
        .all(|(index, pos)| bit_at(63 - index) == pos);

    assert!(success);
}

#[quickcheck]
fn stones_bit_count(rand_bitboard: ShadowBitboard) {
    let rand_bitboard = Bitboard::from(rand_bitboard);
    let expected = rand_bitboard.count_set();
    let result = rand_bitboard.hot_bits().filter(|pos| *pos != 0).count();

    assert_eq!(expected as usize, result);
}
