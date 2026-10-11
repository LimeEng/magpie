use crate::agent::Agent;
use magpie::othello::{Board, Position, Stone};
use rand::seq::IteratorRandom;

/// Plays completely randomly.
pub struct RandomAgent;

impl Agent for RandomAgent {
    fn play(&mut self, stone: Stone, board: Board) -> Position {
        board
            .moves_for(stone)
            .hot_bits()
            .choose(&mut rand::rng())
            .expect("at least one legal move must be available")
    }
}
