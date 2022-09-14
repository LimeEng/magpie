use crate::agent::{Action, Agent};
use magpie::othello::{Board, Position, Stone};
use std::io;
use std::io::Write;

/// Queries the user to provide a valid move to play. The user can also pass
/// their turn.
pub struct HumanAgent;

impl Agent for HumanAgent {
    fn play(&mut self, stone: Stone, board: &Board) -> Action {
        print_prompt();
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            if input.to_lowercase() == "pass" {
                break Action::Pass;
            } else if let Ok(pos) = Position::try_from(input) {
                if board.is_legal_move(stone, pos) {
                    break Action::Move(pos);
                }
                println!("\"{}\" is not a valid move", input.to_lowercase());
            }
            println!("Please enter a valid move, or \"pass\" your turn");
            print_prompt();
        }
    }
}

fn print_prompt() {
    print!("> ");
    // print!() does not flush, unlike println!(), so it needs to be done manually
    io::stdout().flush().expect("Failed to flush stdout");
}
