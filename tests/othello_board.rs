use magpie::othello::{Bitboard, Board, Position, Stone};
use quickcheck_macros::quickcheck;
use std::convert::TryInto;

mod common;

use common::ShadowBoard;

#[test]
fn legal_move_check_one_valid() {
    let board = board_one_legal_move();
    let pos = 0x00_00_00_00_08_00_00_00;
    assert!(board.is_legal_move(Stone::Black, pos.try_into().unwrap()));
}

#[test]
fn legal_move_check_none_valid() {
    let board = board_no_legal_moves();
    let pos = 0x00_00_00_00_08_00_00_00;
    assert!(!board.is_legal_move(Stone::Black, pos.try_into().unwrap()));
}

// Returns a board with only one legal move for black, that is, the following
// move represented as a bitboard: 0x00_00_00_00_08_00_00_00.
fn board_one_legal_move() -> Board {
    let black_pos = 0x88_01_00_00_81_00_00_49;
    let white_pos = 0x00_48_2a_1c_76_1c_2a_00;

    (black_pos, white_pos).try_into().unwrap()
}

fn board_no_legal_moves() -> Board {
    let board = board_one_legal_move();

    let black_pos = 0;
    let white_pos = (board.bits_for(Stone::White) | board.bits_for(Stone::Black)).into();

    (black_pos, white_pos).try_into().unwrap()
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
