#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![doc(html_logo_url = "https://cdn.github.emileng.se/repo/magpie/logo.svg")]

//! Magpie is a high-performance [Othello](https://en.wikipedia.org/wiki/Reversi) library built with bitboards and zero dependencies.
//!
//! All core types live in the [`othello`] module, which offers two abstraction levels:
//!
//! - [`Game`] — rule-checked, turn-aware game logic and state management.
//!   Enforces legal moves and tracks turns.
//! - [`Board`] — lower-level, unchecked board operations for maximum performance.
//!   Useful when building engines.
//!
//! Supporting types include [`Bitboard`] and [`Position`] for board representation,
//! [`Stone`] for player identity, and [`BoardDisplay`] for rendering boards.
//!
//! ## Getting Started
//!
//! ```rust
//! use magpie::othello::{Game, Status, Stone};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut game = Game::new();
//! // Black moves first in Othello
//! assert_eq!(game.current_turn(), Stone::Black);
//!
//! // Pick the first available move and play it
//! let pos = game.moves().hot_bits().next().unwrap();
//! game.play(pos)?;
//!
//! println!("{}", game.display());
//!
//! assert_eq!(game.current_turn(), Stone::White);
//! assert_eq!(game.status(), Status::Progressing);
//! # Ok(())
//! # }
//! ```
//!
//! More examples are available in the
//! [examples directory](https://github.com/LimeEng/magpie/tree/master/examples),
//! including a playable game against a random AI (`cargo run --example human_vs_ai`).
//!
//! [`Board`]: crate::othello::Board
//! [`BoardDisplay`]: crate::othello::BoardDisplay
//! [`Bitboard`]: crate::othello::Bitboard
//! [`Game`]: crate::othello::Game
//! [`Position`]: crate::othello::Position
//! [`Stone`]: crate::othello::Stone
//! [`othello`]: crate::othello

/// Contains core structures and functions for playing Othello
pub mod othello;
