use crate::agent::Action;
use crate::agent::Agent;
use magpie::othello::OthelloBoard;
use magpie::othello::PositionExt;
use magpie::othello::Stone;
use rand::seq::SliceRandom;

/// Plays completely randomly. If no legal moves are available, passes their
/// turn.
pub struct RandomAgent;

impl Agent for RandomAgent {
    fn play(&mut self, stone: Stone, board: &OthelloBoard) -> Action {
        let moves = board.legal_moves_for(stone);
        let segmented_moves: Vec<u64> = moves.positions().collect();

        match segmented_moves.choose(&mut rand::thread_rng()) {
            Some(pos) => Action::Move(*pos),
            None => Action::Pass,
        }
    }
}
