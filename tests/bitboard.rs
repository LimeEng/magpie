use magpie::othello::{Bitboard, Position};
use quickcheck_macros::quickcheck;
use std::convert::TryFrom;

mod common;

use common::ShadowPosition;

#[quickcheck]
fn bitboard_raw_matches(num: u64) {
    assert_eq!(num, Bitboard::from(num).raw());
}

#[quickcheck]
fn position_equals_bitboard(position: ShadowPosition) {
    let position = Position::try_from(position).unwrap();
    assert_eq!(position, Bitboard::from(position));
}

#[quickcheck]
fn bitboards_handles_bitwise(num1: u64, num2: u64) {
    let board1 = Bitboard::from(num1);
    let board2 = Bitboard::from(num2);

    assert_eq!(num1 & num2, (board1 & board2).raw());
    assert_eq!(num1 | num2, (board1 | board2).raw());
    assert_eq!(num1 ^ num2, (board1 ^ board2).raw());

    let mut num1_copy = num1;
    let mut board1_copy = board1;
    num1_copy &= num2;
    board1_copy &= board2;
    assert_eq!(num1_copy, board1_copy.raw());

    let mut num1_copy = num1;
    let mut board1_copy = board1;
    num1_copy |= num2;
    board1_copy |= board2;
    assert_eq!(num1_copy, board1_copy.raw());

    let mut num1_copy = num1;
    let mut board1_copy = board1;
    num1_copy ^= num2;
    board1_copy ^= board2;
    assert_eq!(num1_copy, board1_copy.raw());
}

#[test]
fn full_bitboard_bits_equal_hot_bits() {
    let v1 = Bitboard::from(u64::MAX).bits();
    let v2 = Bitboard::from(u64::MAX).hot_bits();

    let v1: Vec<u64> = v1.map(Bitboard::raw).collect();
    let v2: Vec<u64> = v2.map(Position::raw).collect();

    assert_eq!(v1, v2);
}
