use magpie::othello_board::OthelloBoard;
use magpie::stone::Stone;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Game {
    board: OthelloBoard,
    next_player: Stone,
}

fn main() -> Result<()> {
    let board = OthelloBoard::standard();
    // In Othello, black moves first
    let next_player = Stone::Black;

    let game = Game { board, next_player };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&game)?;
    println!("{}", json);

    // Deserialize from JSON
    let game: Game = serde_json::from_str(&json)?;
    println!("{:?}", game);

    Ok(())
}
