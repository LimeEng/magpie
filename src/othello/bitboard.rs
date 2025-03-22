use crate::othello::{Position, constants::POSITIONS};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a 8x8 board as a `u64`.
///
/// There are no restrictions placed on the bits represented, unlike the
/// similar [`Position`] where only a single bit may be set.
///
/// [`Position`]: crate::othello::Position
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Bitboard(pub(crate) u64);

impl Bitboard {
    /// Retrieves the underlying u64.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = 0.into();
    /// assert_eq!(b.raw(), 0);
    /// ```
    #[must_use]
    pub fn raw(self) -> u64 {
        self.0
    }

    /// Returns true if and only if no bits are set.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = 0.into();
    /// assert!(b.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Counts the number of set bits.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = u64::MAX.into();
    /// assert_eq!(b.count_set(), 64);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn count_set(self) -> u8 {
        self.0.count_ones() as u8
    }

    /// Counts the number of bits that are set to zero.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = u64::MAX.into();
    /// assert_eq!(b.count_empty(), 0);
    /// ```
    #[must_use]
    pub fn count_empty(self) -> u8 {
        self.0.count_zeros().try_into().unwrap()
    }

    /// Extracts each bit as its own bitboard.
    ///
    /// For example, given the following (tiny) bitboard:
    /// ```text
    /// 111
    /// 000
    /// 111
    /// ```
    ///
    /// The iterator will break up that bitboard and yield the following
    /// bitboards:
    /// ```text
    /// 100    010    001    000    000    000    000    000    000
    /// 000 => 000 => 000 => 000 => 000 => 000 => 000 => 000 => 000
    /// 000    000    000    000    000    000    100    010    001
    /// ```
    /// The iterator always return 64 bitboards.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = 0.into();
    /// assert_eq!(b.bits().len(), 64);
    ///  ```
    #[must_use]
    pub fn bits(self) -> impl ExactSizeIterator<Item = Bitboard> {
        POSITIONS.iter().map(move |m| self & *m)
    }

    /// Extracts each bit set to one as its own bitboard.
    ///
    /// For example, given the following (tiny) bitboard:
    /// ```text
    /// 100
    /// 000
    /// 001
    /// ```
    ///
    /// The iterator will break up that bitboard and yield the following
    /// bitboards:
    /// ```text
    /// 100    000
    /// 000 => 000
    /// 000    001
    /// ```
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = u64::from(u32::MAX).into();
    /// assert_eq!(b.hot_bits().len(), 32);
    ///  ```
    #[must_use]
    pub fn hot_bits(self) -> impl ExactSizeIterator<Item = Position> {
        let positions = HotBits {
            remaining: self.count_set(),
            bitboard: self,
        };
        positions.into_iter()
    }
}

#[derive(Clone, Debug)]
struct HotBits {
    remaining: u8,
    bitboard: Bitboard,
}

#[derive(Clone, Debug)]
struct HotBitsIntoIterator {
    remaining: u8,
    bitboard: Bitboard,
}

impl IntoIterator for HotBits {
    type Item = Position;
    type IntoIter = HotBitsIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        HotBitsIntoIterator {
            remaining: self.remaining,
            bitboard: self.bitboard,
        }
    }
}

impl Iterator for HotBitsIntoIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bitboard.is_empty() {
            None
        } else {
            self.remaining -= 1;
            let position = 1 << (63 - self.bitboard.raw().leading_zeros());
            self.bitboard ^= position;

            Some(Position::new_unchecked(position))
        }
    }
}

impl ExactSizeIterator for HotBitsIntoIterator {
    fn len(&self) -> usize {
        self.remaining.into()
    }
}

impl From<u64> for Bitboard {
    fn from(bitboard: u64) -> Self {
        Bitboard(bitboard)
    }
}

impl From<Bitboard> for u64 {
    fn from(bitboard: Bitboard) -> Self {
        bitboard.0
    }
}
