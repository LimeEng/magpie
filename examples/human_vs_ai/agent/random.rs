use crate::agent::{Action, Agent};
use magpie::othello::{Board, Stone};
use rand::seq::IteratorRandom;

/// Plays completely randomly. If no legal moves are available, passes their
/// turn.
pub struct RandomAgent;

impl Agent for RandomAgent {
    fn play(&mut self, stone: Stone, board: Board) -> Action {
        board
            .moves_for(stone)
            .hot_bits()
            .choose(&mut rand::rng())
            .map_or(Action::Pass, Action::Move)
    }
}
