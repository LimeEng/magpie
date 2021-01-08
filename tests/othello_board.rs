use magpie::othello::{OthelloBoard, Stone};
use std::convert::TryFrom;

#[test]
fn legal_move_check_one_valid() {
    let board = board_one_legal_move();
    let pos = 0x00_00_00_00_08_00_00_00;
    assert_eq!(true, board.is_legal_move(Stone::Black, pos));
}

#[test]
fn legal_move_check_one_valid_one_invalid() {
    let board = board_one_legal_move();
    // Note the leading 8
    let pos = 0x80_00_00_00_08_00_00_00;
    assert_eq!(false, board.is_legal_move(Stone::Black, pos));
}

#[test]
fn legal_move_check_two_valid() {
    let board = board_two_legal_moves();
    let pos = 0x80_00_00_00_00_00_00_01;
    assert_eq!(false, board.is_legal_move(Stone::Black, pos));
}

#[test]
fn legal_move_check_none_valid() {
    let board = board_no_legal_moves();
    let pos = 0x00_00_00_00_08_00_00_00;
    assert_eq!(false, board.is_legal_move(Stone::Black, pos));
}

// Returns a board with only one legal move for black, that is, the following
// move represented as a bitboard: 0x00_00_00_00_08_00_00_00.
fn board_one_legal_move() -> OthelloBoard {
    let black_pos = 0x88_01_00_00_81_00_00_49;
    let white_pos = 0x00_48_2a_1c_76_1c_2a_00;

    OthelloBoard::try_from((black_pos, white_pos)).unwrap()
}

// Returns a board with only two legal moves for black, that is, the following
// moves represented as a bitboard: 0x80_00_00_00_00_00_00_01.
fn board_two_legal_moves() -> OthelloBoard {
    let black_pos = 0x01_00_00_00_00_00_00_00;
    let white_pos = 0x7e_01_01_01_01_01_01_00;

    OthelloBoard::try_from((black_pos, white_pos)).unwrap()
}

fn board_no_legal_moves() -> OthelloBoard {
    let board = board_one_legal_move();

    let black_pos = 0;
    let white_pos = board.bits_for(Stone::White) | board.bits_for(Stone::Black);

    OthelloBoard::try_from((black_pos, white_pos)).unwrap()
}
