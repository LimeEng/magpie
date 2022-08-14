use magpie::othello::{Board, Stone};

pub mod human;
pub mod random;

pub use human::HumanAgent;
pub use random::RandomAgent;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Move(u64),
    Pass,
}

/// This trait defines an agent. Given a board with a certain state and a
/// stone, which represents the color of the agent, it needs to provide an
/// action.
pub trait Agent {
    fn play(&mut self, stone: Stone, board: &Board) -> Action;
}
