use magpie::othello::OthelloBoard;
use magpie::othello::Stone;
use serde_json::Result;

#[test]
fn serde_legal_deserialization() -> Result<()> {
    let board = OthelloBoard::standard();
    let json = serde_json::to_string_pretty(&board)?;
    let board: OthelloBoard = serde_json::from_str(&json)?;

    assert_eq!(board.legal_moves_for(Stone::Black).count_ones(), 4);

    Ok(())
}

#[test]
fn serde_illegal_deserialization() {
    let json = r#"{
        "black_stones": 111111111,
        "white_stones": 111111111
    }"#;

    let board = serde_json::from_str::<OthelloBoard>(&json);
    assert!(board.is_err(), "The deserialization should not succeed");
}
