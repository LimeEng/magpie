#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![doc(html_logo_url = "https://cdn.github.emileng.se/repo/magpie/logo.svg")]

//! Magpie is a high-performance library for the classic board game [Othello](https://en.wikipedia.org/wiki/Reversi).
//!
//! ## Key Features
//!
//! - **Built with bitboards**: Uses bitboards for extremely fast board operations
//! - **Zero dependencies**: Core functionality has no external dependencies
//! - **Optional Serde support**: Serialization available through an optional feature flag
//!
//! Furthermore, the library offers two abstraction levels:
//!
//! - **[`Game`] API**: Ensures rule compliance, tracks turns, and maintains board consistency
//! - **[`Board`] API**: Provides raw board operations without validation, when performance is critical.
//!
//! ## Module Overview
//!
//! The [`othello`] module contains core structures and functions for playing Othello.
//!
//! [`Board`]: crate::othello::Board
//! [`Game`]: crate::othello::Game
//! [`othello`]: crate::othello

/// Contains core structures and functions for playing Othello
pub mod othello;
