/// Represents a 8x8 board.
mod bitboard;
/// Implements various useful traits for Bitboards and Positions
mod bitboard_position_impl;
/// Represents an Othello board and provides convenient functions to manipulate it.
mod board;
/// Collection of constants useful for various calculations.
mod constants;
/// Structs and functions that format Othello boards.
mod display;
/// Represents an Othello game.
mod game;
/// Represents a single position on a 8x8 board.
mod position;
/// An enum that represents the two stone colors players can play with.
mod stone;

pub use bitboard::Bitboard;
pub use board::{Board, OthelloError};
pub use display::{BoardDisplay, Format};
pub use game::{Game, Status};
pub use position::{Position, PositionError};
pub use stone::Stone;
