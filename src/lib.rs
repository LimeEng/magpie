//! Magpie is a reasonably efficient Othello library.
//!
//! This library allows the user to play a full game of Othello. Do note that
//! there is currently no explicit support of Reversi, a very similar game.
//! However, magpie is flexible enough that it is possible to play Reversi as
//! well, if the user is willing to do some additional bookkeeping.
//!
//! Magpie is implemented using bitboards, which allows for extremely fast
//! updates and queries while also being memory efficient. More information
//! about bitboards can be found at [`Wikipedia`].
//!
//! Magpie is intentionally minimalistic and requires the user to keep track of
//! various aspects of the game, such as the next player to move. The library
//! is used for calculating legal moves and applying them while still giving
//! the user enough access to the internals to satisfy a wide array of
//! applications in a safe way.
//!
//! Serialization with [Serde](https://serde.rs/) is not supported by default.
//! If you want to opt into using magpie with Serde you can enable a feature
//! flag. Simply change your magpie dependency to the following:
//!
//! ```toml
//! [dependencies]
//! magpie = {version = "0.8.0", features = ["serde"]}
//! ```
//!
//! [`Wikipedia`]: https://en.wikipedia.org/wiki/Bitboard
//! [`Serde`]: https://serde.rs

mod direction;
/// Contains structs and functions that are useful when playing Othello.
pub mod othello;
