use crate::agent::Agent;
use magpie::othello::{Board, Position, Stone};
use std::io::{self, Write};

/// Queries the user to provide a valid move to play.
pub struct HumanAgent;

impl Agent for HumanAgent {
    fn play(&mut self, stone: Stone, board: Board) -> Position {
        print_prompt();
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            if let Ok(pos) = input.parse() {
                if board.is_legal_move(stone, pos) {
                    break pos;
                }
                println!("\"{}\" is not a valid move", input.to_lowercase());
            }
            println!("Please enter a valid move");
            print_prompt();
        }
    }
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
}
