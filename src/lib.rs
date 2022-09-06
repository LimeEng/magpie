#![doc(html_logo_url = "https://limeeng.github.io/cdn/repo/magpie/logo.svg")]

//! Magpie is a reasonably efficient Othello library.
//!
//! This library allows for the implementation of a full game of Othello. It is
//! important to note that there is no explicit support for Reversi, a similar
//! game. However, magpie is flexible enough that it is possible to implement
//! Reversi as well, in exchange for some additional bookkeeping.
//!
//! Magpie is built with bitboards which allows for extremely fast updates and
//! queries. The library is intentionally minimalistic and requires the user to
//! keep track of various aspects of the game, such as the next player to move.
//! Magpie is used for calculating legal moves and applying them while still
//! giving the user enough access to the internals to satisfy a wide array of
//! applications in a safe way.
//!
//! Serialization with [Serde](https://serde.rs/) is not supported by default.
//! If you want to opt into using magpie with Serde you can enable a feature
//! flag. Simply change your magpie dependency to the following:
//!
//! ```toml
//! [dependencies]
//! magpie = {version = "0.9.0", features = ["serde"]}
//! ```
//!
//! [`Serde`]: https://serde.rs

/// Contains structs and functions that are useful when playing Othello.
pub mod othello;
