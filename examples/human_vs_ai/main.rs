use crate::agent::{Agent, HumanAgent, RandomAgent};
use magpie::othello::{Game, Status, Stone};

mod agent;

fn main() {
    play();
}

fn play() {
    let mut agent1 = HumanAgent;
    let mut agent2 = RandomAgent;

    let mut game = Game::new();

    while let Status::Progressing = game.status() {
        let current_turn = game.current_turn();
        println!("{}", game.display().with_stone(current_turn));
        let next_move = match current_turn {
            Stone::Black => agent1.play(Stone::Black, game.board()),
            Stone::White => agent2.play(Stone::White, game.board()),
        };

        game.play(next_move)
            .unwrap_or_else(|_| panic!("{current_turn:?} tried to make an illegal move"));
        println!("{current_turn:?} played {next_move}");

        // Passing is automatic: if the opponent has no legal moves the turn
        // stays with the current player
        if game.status() == Status::Progressing && game.current_turn() == current_turn {
            println!("{:?} has no legal moves and passes", current_turn.flip());
        }
    }
    println!("Final board");
    println!("{}", game.display());
    let black = game.bits_for(Stone::Black).count_set();
    let white = game.bits_for(Stone::White).count_set();
    println!("Game finished with {black} - {white} ((B)lack - (W)hite)");
}
