use magpie::othello::{Bitboard, Board, Game, Position, Stone};
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

fn main() -> Result<(), serde_json::Error> {
    print_serde(&Game::new())?;
    print_serde(&Board::standard())?;
    print_serde(&Stone::Black)?;
    print_serde(&Bitboard::FILLED)?;
    print_serde(&Position::try_from("A1").unwrap())?;

    Ok(())
}

fn print_serde<T>(value: &T) -> Result<(), serde_json::Error>
where
    T: Serialize + DeserializeOwned + Debug,
{
    let json = serde_json::to_string_pretty(value)?;
    let value: T = serde_json::from_str(&json)?;
    println!("-----[ Deserialized ]-----");
    println!("{value:#?}");
    println!("-----[  Serialized  ]-----");
    println!("{json}");
    println!("==========================");

    Ok(())
}
