use magpie::othello_board::OthelloBoard;
use magpie::stone::Stone;

pub mod human_agent;
pub mod random_agent;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Move(u64),
    Pass,
}

pub trait Agent {
    fn play(&mut self, stone: Stone, board: &OthelloBoard) -> Action;
}
