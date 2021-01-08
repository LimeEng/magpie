use magpie::othello::{OthelloBoard, Stone};

pub mod human_agent;
pub mod random_agent;

pub use human_agent::HumanAgent;
pub use random_agent::RandomAgent;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Move(u64),
    Pass,
}

/// This trait defines an agent. Given a board with a certain state and a
/// stone, which represents the color of the agent, it needs to provide an
/// action.
pub trait Agent {
    fn play(&mut self, stone: Stone, board: &OthelloBoard) -> Action;
}
