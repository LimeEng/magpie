use crate::agent::{Action, Agent};
use magpie::othello::{Board, Stone, StoneExt};
use rand::seq::IteratorRandom;

/// Plays completely randomly. If no legal moves are available, passes their
/// turn.
pub struct RandomAgent;

impl Agent for RandomAgent {
    fn play(&mut self, stone: Stone, board: &Board) -> Action {
        board
            .moves_for(stone)
            .stones()
            .choose(&mut rand::thread_rng())
            .map(Action::Move)
            .unwrap_or(Action::Pass)
    }
}
