mod common;

use common::ShadowPosition;
use magpie::othello::{Bitboard, Position, PositionError};
use quickcheck_macros::quickcheck;

#[quickcheck]
fn position_equals_bitboard(position: ShadowPosition) {
    let position = Position::try_from(position).unwrap();
    assert_eq!(position, Bitboard::from(position));
}

#[test]
fn position_rank_matches() {
    #[rustfmt::skip]
    let expected_order = vec![
        0, 0, 0, 0, 0, 0, 0, 0,
        1, 1, 1, 1, 1, 1, 1, 1,
        2, 2, 2, 2, 2, 2, 2, 2,
        3, 3, 3, 3, 3, 3, 3, 3,
        4, 4, 4, 4, 4, 4, 4, 4,
        5, 5, 5, 5, 5, 5, 5, 5,
        6, 6, 6, 6, 6, 6, 6, 6,
        7, 7, 7, 7, 7, 7, 7, 7,
    ];
    let all_equal = Bitboard::FILLED
        .hot_bits()
        .map(Position::rank)
        .zip(expected_order)
        .all(|(a, b)| a == b);
    assert!(all_equal);
}

#[test]
fn position_file_matches() {
    #[rustfmt::skip]
    let expected_order = vec![
        0, 1, 2, 3, 4, 5, 6, 7,
        0, 1, 2, 3, 4, 5, 6, 7,
        0, 1, 2, 3, 4, 5, 6, 7,
        0, 1, 2, 3, 4, 5, 6, 7,
        0, 1, 2, 3, 4, 5, 6, 7,
        0, 1, 2, 3, 4, 5, 6, 7,
        0, 1, 2, 3, 4, 5, 6, 7,
        0, 1, 2, 3, 4, 5, 6, 7,
    ];
    let all_equal = Bitboard::FILLED
        .hot_bits()
        .map(Position::file)
        .zip(expected_order)
        .all(|(a, b)| a == b);
    assert!(all_equal);
}

#[test]
fn position_notation_matches() {
    #[rustfmt::skip]
    let expected_order = vec![
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    ];
    let all_equal = Bitboard::FILLED
        .hot_bits()
        .map(|pos| pos.to_string())
        .zip(expected_order)
        .all(|(a, b)| a == b);
    assert!(all_equal);
}

#[test]
fn position_notation_parses_case_insensitively() {
    let pos_1 = "E7".parse::<Position>().unwrap();
    let pos_2 = "e7".parse::<Position>().unwrap();
    assert_eq!(pos_1, pos_2);
}

#[test]
fn position_notation_rejects_invalid_input() {
    for input in ["", "a", "a0", "i1", "a10", "é1"] {
        assert_eq!(
            input.parse::<Position>(),
            Err(PositionError::InvalidNotation)
        );
    }
}

#[test]
fn position_notation_identity() {
    #[rustfmt::skip]
    let full_notation = vec![
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    ];

    for n in &full_notation {
        let identity = n.parse().map(|pos: Position| pos.to_string()).unwrap();
        assert_eq!(*n, identity);
    }
}
