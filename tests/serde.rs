use magpie::othello::OthelloBoard;
use magpie::othello::Stone;
use serde_json::Result;

#[test]
fn serde_legal_deserialization() -> Result<()> {
    let board = OthelloBoard::standard();
    let json = serde_json::to_string_pretty(&board)?;
    let board: OthelloBoard = serde_json::from_str(&json)?;

    assert_eq!(OthelloBoard::standard(), board);

    Ok(())
}

#[test]
fn serde_illegal_deserialization() {
    // These two numbers share a bit with value 1 in the same position
    let json = r#"{
        "black_stones": 123456789,
        "white_stones": 1
    }"#;

    let board = serde_json::from_str::<OthelloBoard>(&json);
    assert!(board.is_err(), "The deserialization should not succeed");
}
