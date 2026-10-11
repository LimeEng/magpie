use magpie::othello::{Board, Position, Stone};

mod human;
mod random;

pub use human::HumanAgent;
pub use random::RandomAgent;

/// This trait defines an agent. Given a board with a certain state and a
/// stone, which represents the color of the agent, it needs to provide a
/// move to play. The game guarantees that the agent always has at least
/// one legal move available when called, since forced passes are resolved
/// automatically.
pub trait Agent {
    fn play(&mut self, stone: Stone, board: Board) -> Position;
}
