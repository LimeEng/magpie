/// Collection of constants useful for various calculations.
mod constants;
/// Structs and functions that format Othello boards.
mod display;
/// Represents an Othello board and provides convenient methods to safely manipulate it.
mod othello_board;
/// An enum that represents the two stone colors players can play with.
mod stone;

pub use display::{Format, OthelloDisplay};
pub use othello_board::{OthelloBoard, OthelloError, SquareExt, StoneExt};
pub use stone::Stone;
