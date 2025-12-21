mod common;

use common::{ShadowBitboard, ShadowPosition};
use magpie::othello::{Bitboard, Position};
use quickcheck_macros::quickcheck;

#[quickcheck]
fn bitboard_raw_matches(num: u64) {
    assert_eq!(num, Bitboard::from(num).raw());
}

#[quickcheck]
fn bitboard_equals_position(position: ShadowPosition) {
    let position = Position::try_from(position).unwrap();
    assert_eq!(position, Bitboard::from(position));
}

#[quickcheck]
fn bitboard_handles_bitwise(num1: u64, num2: u64) {
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
    let v1 = Bitboard::FILLED.bits();
    let v2 = Bitboard::FILLED.hot_bits();

    let v1: Vec<u64> = v1.map(Bitboard::raw).collect();
    let v2: Vec<u64> = v2.map(Position::raw).collect();

    assert_eq!(v1, v2);
}

#[test]
fn bitboard_filled_constant() {
    assert_eq!(Bitboard::FILLED.raw(), u64::MAX);
    assert_eq!(Bitboard::FILLED.count_set(), 64);
    assert_eq!(Bitboard::FILLED.count_empty(), 0);
    assert!(!Bitboard::FILLED.is_empty());
}

#[test]
fn bitboard_empty_constant() {
    assert_eq!(Bitboard::EMPTY.raw(), 0);
    assert_eq!(Bitboard::EMPTY.count_set(), 0);
    assert_eq!(Bitboard::EMPTY.count_empty(), 64);
    assert!(Bitboard::EMPTY.is_empty());
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
