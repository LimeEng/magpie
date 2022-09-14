use magpie::othello::{Board, Stone};
use std::convert::TryInto;

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
