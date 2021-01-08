/// Represents an Othello board and provides convenient methods to safely manipulate it.
mod othello_board;
/// An enum that represents the two stone colors players can play with.
mod stone;

pub use othello_board::{OthelloBoard, OthelloError, PositionExt};
pub use stone::Stone;
