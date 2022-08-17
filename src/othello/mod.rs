/// Represents an Othello board and provides convenient methods to safely manipulate it.
mod board;
/// Collection of constants useful for various calculations.
mod constants;
/// Structs and functions that format Othello boards.
mod display;
/// An enum that represents the two stone colors players can play with.
mod stone;

mod position;

mod bitboard;

mod bitboard_position_impl;

pub use bitboard::Bitboard;
pub use board::{Board, OthelloError, SquareExt, StoneExt};
pub use display::{BoardDisplay, Format};
pub use position::{Position, PositionError};
pub use stone::Stone;
