/// Represents a 8x8 board.
mod bitboard;
/// Represents an Othello board and provides convenient functions to manipulate it.
mod board;
/// Collection of constants useful for various calculations.
mod constants;
/// Structs and functions that format Othello boards.
mod display;
/// Represents an Othello game.
mod game;
/// Implements various useful traits for Bitboards and Positions
mod ops;
/// Represents a single position on a 8x8 board.
mod position;
/// An enum that represents the two stone colors players can play with.
mod stone;

pub use bitboard::Bitboard;
pub use board::{Board, BoardError};
pub use display::{BoardDisplay, Format};
pub use game::{Game, GameError, Status};
pub use position::{Position, PositionError};
pub use stone::Stone;
