use crate::othello::{
    Bitboard,
    constants::{FILES, POSITIONS, POSITIONS_AS_NOTATION, RANKS},
};
use std::{
    error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a single position on a 8x8 board as a `u64`.
///
/// Unlike the similar [`Bitboard`] which places no restrictions on the bits it
/// represents, this struct represents exactly a single set bit.
///
/// Bitboard representations are quite inconvenient in some contexts which is
/// why some convenience functions are provided to convert between different
/// formats. In these contexts, MSB denotes A1 while LSB denotes H8, as can be
/// seen in the graphic below.
///
/// ```text
///     A    B    C    D    E    F    G    H
///   +----+----+----+----+----+----+----+----+
/// 1 | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 |
///   +----+----+----+----+----+----+----+----+
/// 2 | 08 | 09 | 10 | 11 | 12 | 13 | 14 | 15 |
///   +----+----+----+----+----+----+----+----+
/// 3 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 |
///   +----+----+----+----+----+----+----+----+
/// 4 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 |
///   +----+----+----+----+----+----+----+----+
/// 5 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 |
///   +----+----+----+----+----+----+----+----+
/// 6 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 |
///   +----+----+----+----+----+----+----+----+
/// 7 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 |
///   +----+----+----+----+----+----+----+----+
/// 8 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 |
///   +----+----+----+----+----+----+----+----+
/// ```
///
/// # Invariant Safety
///
/// Position maintains the invariant that exactly one bit is always set.
/// Operations that could violate this return [`Bitboard`] instead:
///
/// - `Position & Position => Bitboard` (may produce 0 or multiple bits)
/// - `Position << T => Bitboard` (may shift bit out of range)
/// - `Position >> T => Bitboard` (may shift bit out of range)
///
/// The following operations are intentionally NOT implemented:
///
/// ```compile_fail
/// use magpie::othello::Position;
///
/// let mut pos = Position::try_from(1 << 63)?;
/// // Does not compile - would shift bit out of range (0 bits)
/// pos <<= 1u8;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ```compile_fail
/// use magpie::othello::Position;
///
/// let mut pos = Position::try_from(1)?;
/// // Does not compile - would shift bit out of range (0 bits)
/// pos >>= 1u8;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ```compile_fail
/// use magpie::othello::{Position, Bitboard};
///
/// let mut pos = Position::try_from(1)?;
/// // Does not compile - would violate invariant
/// pos &= Bitboard::from(0);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ```compile_fail
/// use magpie::othello::Position;
///
/// let pos = Position::try_from(1)?;
/// // Does not compile - would violate invariant
/// let pos = !pos;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ```compile_fail
/// use magpie::othello::Position;
/// // Does not compile - derived default would be 0 bits
/// let pos = Position::default();
/// ```
///
/// [`Bitboard`]: crate::othello::Bitboard
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(pub(crate) u64);

impl Position {
    /// Constructs a new Position from a bitboard but does not check if
    /// a single bit is set.
    pub(crate) fn new_unchecked(bitboard: u64) -> Self {
        Self(bitboard)
    }

    /// Retrieves the underlying u64.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Position;
    ///
    /// let p: Position = (1 << 32).try_into()?;
    /// assert_eq!(p.raw(), (1 << 32));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn raw(self) -> u64 {
        self.0
    }

    /// Calculates the zero-indexed rank the position is referring to.
    ///
    /// How ranks and files are represented can be found in the top-level
    /// documentation for [`Position`].
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Position;
    ///
    /// let rank_and_file = (3, 4);
    /// let p = Position::try_from(rank_and_file)?;
    /// assert_eq!(p.rank(), rank_and_file.0);
    /// assert_eq!(p.file(), rank_and_file.1);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Position`]: crate::othello::Position
    #[must_use]
    pub fn rank(self) -> u8 {
        (self.0.leading_zeros() / 8).try_into().unwrap()
    }

    /// Calculates the zero-indexed file the position is referring to.
    ///
    /// How ranks and files are represented can be found in the top-level
    /// documentation for [`Position`].
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Position;
    ///
    /// let rank_and_file = (3, 4);
    /// let p = Position::try_from(rank_and_file)?;
    /// assert_eq!(p.rank(), rank_and_file.0);
    /// assert_eq!(p.file(), rank_and_file.1);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Position`]: crate::othello::Position
    #[must_use]
    pub fn file(self) -> u8 {
        (self.0.leading_zeros() % 8).try_into().unwrap()
    }
}

/// Calculates a human-readable board position.
///
/// How board positions are represented can be found in the top-level
/// documentation for [`Position`].
///
/// # Examples
/// ```rust
/// use magpie::othello::Position;
///
/// let notation = "A1";
/// let p: Position = notation.parse()?;
/// assert_eq!(p.to_string().to_lowercase(), notation.to_lowercase());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// [`Position`]: crate::othello::Position
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(POSITIONS_AS_NOTATION[self.0.leading_zeros() as usize])
    }
}

impl From<Position> for Bitboard {
    fn from(position: Position) -> Self {
        Bitboard::from(position.0)
    }
}

impl From<Position> for u64 {
    fn from(position: Position) -> Self {
        position.0
    }
}

impl TryFrom<(u8, u8)> for Position {
    type Error = PositionError;

    /// Constructs a position from a zero-indexed rank and file pair.
    ///
    /// Returns an error if either the rank or file does not fit
    /// into a 8x8 board.
    ///
    /// How ranks and files are represented can be found in the top-level
    /// documentation for [`Position`].
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Position;
    ///
    /// let rank_and_file = (3, 4);
    /// let p = Position::try_from(rank_and_file)?;
    /// assert_eq!(p.rank(), rank_and_file.0);
    /// assert_eq!(p.file(), rank_and_file.1);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Position`]: crate::othello::Position
    fn try_from(pair: (u8, u8)) -> Result<Self, Self::Error> {
        let (rank, file) = pair;
        if rank > 7 || file > 7 {
            Err(PositionError::OutOfRange)
        } else {
            let bitboard = RANKS[rank as usize] & FILES[file as usize];
            Ok(Position::new_unchecked(bitboard))
        }
    }
}

/// Constructs a position from human-readable notation.
///
/// The conversion is case-insensitive. Returns an error if the notation is invalid.
///
/// How board positions are represented can be found in the top-level
/// documentation for [`Position`].
///
/// # Examples
/// ```rust
/// use magpie::othello::Position;
///
/// let notation = "A1";
/// let p: Position = notation.parse()?;
/// assert_eq!(p.to_string().to_lowercase(), notation.to_lowercase());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
impl FromStr for Position {
    type Err = PositionError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let text = text.to_lowercase();
        let bitboard = POSITIONS_AS_NOTATION
            .iter()
            .position(|position| position == &text)
            .map(|index| POSITIONS[index])
            .ok_or(PositionError::InvalidNotation)?;
        Ok(Position::new_unchecked(bitboard))
    }
}

impl TryFrom<u64> for Position {
    type Error = PositionError;

    /// Constructs a position from a `u64`.
    ///
    /// Returns an error if the `u64` does not have exactly one bit set.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Position;
    ///
    /// let num = 1 << 32;
    /// let p = Position::try_from(num)?;
    /// assert_eq!(p.raw(), num);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn try_from(bitboard: u64) -> Result<Self, Self::Error> {
        // Equivalent to bitboard.count_ones() == 1
        // Suggested improvement from Clippy that might
        // lead to negligible performance gains ¯\_(ツ)_/¯
        if bitboard.is_power_of_two() {
            Ok(Position::new_unchecked(bitboard))
        } else {
            Err(PositionError::ExpectedSingleBit)
        }
    }
}

impl TryFrom<Bitboard> for Position {
    type Error = PositionError;

    /// Constructs a position from a [`Bitboard`].
    ///
    /// Returns an error if the [`Bitboard`] does not have exactly one bit set.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Bitboard, Position};
    ///
    /// let bitboard = Bitboard::from(1 << 32);
    /// let p = Position::try_from(bitboard)?;
    /// assert_eq!(p.raw(), bitboard);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Bitboard`]: crate::othello::Bitboard
    fn try_from(bitboard: Bitboard) -> Result<Self, Self::Error> {
        Position::try_from(bitboard.raw())
    }
}

/// This enum represents errors that may occur when handling Positions.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum PositionError {
    /// Indicates that the bitboard did not contain exactly one set bit.
    ExpectedSingleBit,
    /// Indicates that the textual notation was invalid (e.g. "Z9" or wrong format).
    InvalidNotation,
    /// Indicates that the provided rank/file pair was out of the [0, 8) range.
    OutOfRange,
}

/// Formats the position error as a human-readable message.
impl Display for PositionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedSingleBit => write!(f, "expected exactly one set bit"),
            Self::InvalidNotation => write!(f, "invalid notation"),
            Self::OutOfRange => write!(f, "rank or file out of range (expected [0, 8))"),
        }
    }
}

impl error::Error for PositionError {}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Position::try_from(value)
            .map_err(|_| serde::de::Error::custom("Position must have exactly one bit set"))
    }
}
