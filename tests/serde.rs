use magpie::othello::Board;
use serde_json::Result;

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
