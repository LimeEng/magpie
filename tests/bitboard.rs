use magpie::othello::{Bitboard, Position};
use quickcheck_macros::quickcheck;
use std::convert::TryFrom;

mod common;

use common::{ShadowBitboard, ShadowPosition};

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
fn position_rank_matches() {
    let expected_order = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3,
        3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7,
        7, 7, 7, 7,
    ];
    let all_equal = Bitboard::from(u64::MAX)
        .hot_bits()
        .map(Position::rank)
        .zip(expected_order)
        .all(|(a, b)| a == b);
    assert!(all_equal);
}

#[test]
fn position_file_matches() {
    let expected_order = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5,
        6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3,
        4, 5, 6, 7,
    ];
    let all_equal = Bitboard::from(u64::MAX)
        .hot_bits()
        .map(Position::file)
        .zip(expected_order)
        .all(|(a, b)| a == b);
    assert!(all_equal);
}

#[test]
fn position_notation_matches() {
    let expected_order = vec![
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2",
        "h2", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4",
        "g4", "h4", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6",
        "f6", "g6", "h6", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8",
        "e8", "f8", "g8", "h8",
    ];
    let all_equal = Bitboard::from(u64::MAX)
        .hot_bits()
        .map(Position::to_notation)
        .zip(expected_order)
        .all(|(a, b)| a == b);
    assert!(all_equal);
}

#[test]
fn position_notation_identity() {
    let full_notation = vec![
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2",
        "h2", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4",
        "g4", "h4", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6",
        "f6", "g6", "h6", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8",
        "e8", "f8", "g8", "h8",
    ];

    for n in &full_notation {
        let identity = Position::try_from(*n).map(Position::to_notation).unwrap();
        assert_eq!(*n, identity);
    }
}

#[test]
fn full_bitboard_bits_equal_hot_bits() {
    let v1 = Bitboard::from(u64::MAX).bits();
    let v2 = Bitboard::from(u64::MAX).hot_bits();

    let v1: Vec<u64> = v1.map(Bitboard::raw).collect();
    let v2: Vec<u64> = v2.map(Position::raw).collect();

    assert_eq!(v1, v2);
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
