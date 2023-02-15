use crate::agent::{Action, Agent, HumanAgent, RandomAgent};
use magpie::othello::{Game, Status, Stone};

pub mod agent;

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
        let action = match current_turn {
            Stone::Black => agent1.play(Stone::Black, game.board()),
            Stone::White => agent2.play(Stone::White, game.board()),
        };

        match action {
            Action::Move(next_move) => {
                game.play(next_move)
                    .unwrap_or_else(|_| panic!("{current_turn:?} tried to make an illegal move"));
                println!("{:?} played {}", current_turn, next_move.to_notation());
            }
            Action::Pass => {
                println!("{current_turn:?} passed their turn");
                game.pass_turn();
            }
        }
    }
    println!("Final board");
    println!("{}", game.display());
    let black = game.bits_for(Stone::Black).count_set();
    let white = game.bits_for(Stone::White).count_set();
    println!("Game finished with {black} - {white} ((B)lack - (W)hite)");
}
