mod common;

use common::{ShadowBitboard, ShadowBoard, ShadowPosition, ShadowStone};
use magpie::othello::{Bitboard, Board, Game, Position, Stone};
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;
use serde_json::Result;

fn serde_roundtrip<T>(value: &T) -> bool
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq,
{
    let json = serde_json::to_string(value).unwrap();
    let decoded: T = serde_json::from_str(&json).unwrap();
    *value == decoded
}

#[test]
fn serde_legal_deserialization() -> Result<()> {
    let board = Board::standard();
    let json = serde_json::to_string_pretty(&board)?;
    let board: Board = serde_json::from_str(&json)?;

    assert_eq!(Board::standard(), board);

    Ok(())
}

#[test]
fn serde_illegal_deserialization() {
    // These two numbers share a bit with value 1 in the same position
    let json = r#"{
        "black_stones": 123456789,
        "white_stones": 1
    }"#;

    let board = serde_json::from_str::<Board>(json);
    assert!(board.is_err(), "The deserialization should not succeed");
}

#[quickcheck]
fn bitboard_roundtrip(bitboard: ShadowBitboard) -> bool {
    let bitboard = Bitboard::from(bitboard);
    serde_roundtrip(&bitboard)
}

#[quickcheck]
fn position_roundtrip(shadow: ShadowPosition) -> bool {
    let pos = Position::try_from(shadow).unwrap();
    serde_roundtrip(&pos)
}

#[quickcheck]
fn board_roundtrip(shadow: ShadowBoard) -> bool {
    let board = Board::try_from(shadow).unwrap();
    serde_roundtrip(&board)
}

#[quickcheck]
fn game_roundtrip(shadow: ShadowBoard, stone: ShadowStone, passed: bool) -> bool {
    let board = Board::try_from(shadow).unwrap();
    let stone = Stone::from(stone);
    let game = Game::from_state(board, stone, passed).unwrap();
    serde_roundtrip(&game)
}

#[quickcheck]
fn stone_roundtrip(stone: ShadowStone) -> bool {
    let stone = Stone::from(stone);
    serde_roundtrip(&stone)
}

#[test]
fn position_rejects_zero_bits() {
    let json = "0";
    let result = serde_json::from_str::<Position>(json);
    assert!(result.is_err());
}

#[quickcheck]
fn position_rejects_multiple_bits(val: u64) -> TestResult {
    if val.count_ones() > 1 {
        let json = format!("{val}");
        let result = serde_json::from_str::<Position>(&json);
        TestResult::from_bool(result.is_err())
    } else {
        TestResult::discard()
    }
}

#[quickcheck]
fn board_rejects_overlapping_stones(black: u64, white: u64) -> TestResult {
    if black & white != 0 {
        let json = format!(r#"{{"black_stones":{black},"white_stones":{white}}}"#);
        let result = serde_json::from_str::<Board>(&json);
        TestResult::from_bool(result.is_err())
    } else {
        TestResult::discard()
    }
}

#[quickcheck]
fn game_rejects_invalid_board(
    black: u64,
    white: u64,
    stone: ShadowStone,
    passed: bool,
) -> TestResult {
    let stone = Stone::from(stone);
    if black & white != 0 {
        let json = format!(
            r#"{{"board":{{"black_stones":{black},"white_stones":{white}}},"next_player":{},"passed_last_turn":{passed}}}"#,
            if stone == Stone::Black {
                r#""Black""#
            } else {
                r#""White""#
            },
        );
        let result = serde_json::from_str::<Game>(&json);
        TestResult::from_bool(result.is_err())
    } else {
        TestResult::discard()
    }
}
