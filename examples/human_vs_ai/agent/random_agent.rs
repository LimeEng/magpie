use crate::agent::Action;
use crate::agent::Agent;
use magpie::othello_board::OthelloBoard;
use magpie::othello_board::PositionExt;
use magpie::stone::Stone;
use rand::seq::SliceRandom;

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
