use crate::agent::{Action, Agent};
use magpie::othello::{Board, Stone, StoneExt};
use rand::seq::SliceRandom;

/// Plays completely randomly. If no legal moves are available, passes their
/// turn.
pub struct RandomAgent;

impl Agent for RandomAgent {
    fn play(&mut self, stone: Stone, board: &Board) -> Action {
        let moves = board.moves_for(stone);
        let segmented_moves: Vec<u64> = moves.stones().collect();

        match segmented_moves.choose(&mut rand::thread_rng()) {
            Some(pos) => Action::Move(*pos),
            None => Action::Pass,
        }
    }
}
