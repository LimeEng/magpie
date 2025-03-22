#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![doc(html_logo_url = "https://cdn.github.emileng.se/repo/magpie/logo.svg")]

//! Magpie is a reasonably efficient Othello library.
//!
//! Magpie is built with bitboards which allows for extremely fast updates and
//! queries. Two abstraction levels are available, the higher level [`Game`]
//! and lower-level [`Board`]. The [`Game`]-struct guarantees that only legal
//! moves will be made and that the board will be kept consistent. The drawback
//! is that it is not as flexible or performant as the alternative. The
//! [`Board`]-struct does not keep track of whose turn it is, whether a player
//! passed their turn, or validates inputs, which makes it better suited for
//! engines.
//!
//! There is no explicit support for Reversi, a similar game. However, magpie
//! is flexible enough that it is possible to implement Reversi as well, in
//! exchange for some additional bookkeeping.
//!
//! Serialization with [Serde](https://serde.rs/) is not supported by default.
//! If you want to opt into using magpie with Serde you can enable a feature
//! flag. Simply change your magpie dependency to the following:
//!
//! ```toml
//! [dependencies]
//! magpie = {version = "0.11", features = ["serde"]}
//! ```
//!
//! [`Serde`]: https://serde.rs
//! [`Board`]: crate::othello::Board
//! [`Game`]: crate::othello::Game

/// Contains structs and functions that are useful when playing Othello.
pub mod othello;
