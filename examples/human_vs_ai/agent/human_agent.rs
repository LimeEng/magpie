use crate::agent::Action;
use crate::agent::Agent;
use crate::coord::Coord;
use magpie::othello_board::OthelloBoard;
use magpie::stone::Stone;
use std::io;
use std::io::Write;

/// Queries the user to provide a valid move to play. The user can also pass
/// their turn.
pub struct HumanAgent;

impl Agent for HumanAgent {
    fn play(&mut self, stone: Stone, board: &OthelloBoard) -> Action {
        print_prompt();
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            if input.to_lowercase() == "pass" {
                break Action::Pass;
            } else if let Ok(coord) = input.parse::<Coord>() {
                let action = coord.as_bitboard();
                if board.is_legal_move(stone, action) {
                    break Action::Move(action);
                }
                println!("{} is not a valid move", input.to_lowercase());
            }
            println!("Please enter a valid move, or \"pass\" your turn");
            print_prompt();
        }
    }
}

fn print_prompt() {
    print!("> ");
    // print!() does not flush, unlike println!(), so it needs to be done manually
    io::stdout().flush().expect("Could not flush stdout");
}
