#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Enum that represents the two different possible stone colors available on a standard Othello board.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    /// Returns the opposite side of a standard Othello stone.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::stone::Stone;
    ///
    /// let stone = Stone::Black;
    /// assert_eq!(stone.flip(), Stone::White);
    /// ```
    pub fn flip(&self) -> Stone {
        use Stone::*;
        match &self {
            Black => White,
            White => Black,
        }
    }
}
