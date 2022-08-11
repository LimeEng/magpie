use crate::othello::{
    constants::{
        BLACK_START_POS, FILE_A, FILE_H, MASKS, RANK_1, RANK_8, SHIFT_DIRS, SHIFT_MASKS,
        SHIFT_RAYS, WHITE_START_POS,
    },
    display::OthelloDisplay,
    Position, Stone,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an Othello board and provides convenient methods to safely manipulate it.
///
/// The board is represented by two bitboards, one for black player and one for
/// white player. Each bitboard is a 64-bit unsigned integer, where each bit
/// encodes if a stone for that player exists on that space. As can be seen in
/// the graphic below, MSB denotes A1 while LSB denotes H8.
///
/// Many operations on the Othello board either requires or returns `u64`, all
/// of which are interpreted the same way as the graphic below. Some
/// operations, like [`place_stone`] expects that the argument bitboard only
/// has a single bit set and will return an error if that is false.
///
/// [`place_stone`]: crate::othello::OthelloBoard::place_stone
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// Some validation need to be performed when deserializing an Othello board.
// Instead of writing a custom serde deserializer, a shadow type is created.
// This shadow type can be deserialized and nothing else. Thanks to some Serde
// magic it is possible to reuse the TryFrom trait and get proper validation.
#[cfg_attr(feature = "serde", serde(try_from = "ShadowOthelloBoard"))]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OthelloBoard {
    black_stones: u64,
    white_stones: u64,
}

impl OthelloBoard {
    /// Returns a completely empty board.
    ///
    /// This can be useful for setting up specific scenarios but for most
    /// users, the [`standard`] constructor will be more useful.
    ///
    /// [`standard`]: crate::othello::OthelloBoard::standard
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::OthelloBoard;
    ///
    /// let board = OthelloBoard::empty();
    /// assert_eq!(64, board.empty_squares().count_ones());
    /// ```
    pub fn empty() -> Self {
        Self {
            black_stones: 0,
            white_stones: 0,
        }
    }

    /// Returns a board with the standard opening position configured.
    ///
    /// If `W` denotes white and `B` denotes black, this is the opening
    /// position:
    /// ```text
    ///     ABCDEFGH
    ///    +--------+
    ///  1 |........|
    ///  2 |........|
    ///  3 |........|
    ///  4 |...WB...|
    ///  5 |...BW...|
    ///  6 |........|
    ///  7 |........|
    ///  8 |........|
    ///    +--------+
    /// ```
    /// # Examples
    /// ```rust
    /// use magpie::othello::OthelloBoard;
    ///
    /// let board = OthelloBoard::standard();
    /// assert_eq!(60, board.empty_squares().count_ones());
    /// ```
    pub fn standard() -> Self {
        Self {
            black_stones: BLACK_START_POS,
            white_stones: WHITE_START_POS,
        }
    }

    /// Places stones in the specified positions.
    ///
    /// Unlike the similar [`place_stone`] function, this function places no
    /// restrictions on the `pos` argument. Multiple stones may be placed at
    /// once, but no other stones will be flipped as during normal play.
    /// The only check is regarding whether or not any of the stones would be
    /// placed on top of a stone of the opposite color, and if so, returns an
    /// error leaving the board untouched.
    ///
    ///  [`place_stone`]: crate::othello::OthelloBoard::place_stone
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{OthelloBoard, Stone};
    ///
    /// let mut board = OthelloBoard::empty();
    /// assert!(board.place_stone_unchecked(Stone::Black, 1_u64).is_ok());
    /// ```
    pub fn place_stone_unchecked(&mut self, stone: Stone, pos: u64) -> Result<(), OthelloError> {
        if self.bits_for(stone.flip()) & pos != 0 {
            return Err(OthelloError::PiecesOverlapping);
        }
        match stone {
            Stone::Black => self.black_stones |= pos,
            Stone::White => self.white_stones |= pos,
        }
        Ok(())
    }

    /// Removes stones in the specified positions.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{OthelloBoard, Stone};
    ///
    /// let mut board = OthelloBoard::standard();
    /// let black_stones = board.bits_for(Stone::Black);
    /// let white_stones = board.bits_for(Stone::White);
    /// board.remove_stone_unchecked(Stone::Black, black_stones);
    /// board.remove_stone_unchecked(Stone::White, white_stones);
    /// assert_eq!(OthelloBoard::empty(), board);
    /// ```
    pub fn remove_stone_unchecked(&mut self, stone: Stone, pos: u64) {
        match stone {
            Stone::Black => self.black_stones &= !pos,
            Stone::White => self.white_stones &= !pos,
        }
    }

    /// Places a stone in the specified position and updates the board accordingly.
    ///
    /// If the argument `pos` does not have exactly one bit set or the move is
    /// not legal, an error will be returned, leaving the board untouched.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{OthelloBoard, StoneExt, Stone};
    ///
    /// let mut board = OthelloBoard::standard();
    /// let player = Stone::Black;
    /// let pos = board
    ///     .moves_for(player)
    ///     .stones()
    ///     .next()
    ///     .unwrap();
    /// assert!(board.place_stone(Stone::Black, pos).is_ok());
    /// ```
    pub fn place_stone(&mut self, stone: Stone, pos: u64) -> Result<(), OthelloError> {
        if pos.count_ones() != 1 {
            return Err(OthelloError::MultipleMovesAttempted);
        }

        let current_bits = self.bits_for(stone);
        let opponent_bits = self.bits_for(stone.flip());

        // Pos must be on an empty square to be legal
        if pos & (current_bits | opponent_bits) != 0 {
            return Err(OthelloError::IllegalMove);
        }

        let mut mask = 0;
        for (i, shift) in SHIFT_DIRS.iter().enumerate() {
            let mut dir_mask = 0;
            let shift_mask = SHIFT_MASKS[i] & SHIFT_RAYS[pos.leading_zeros() as usize][i];
            let opponent_bits = opponent_bits & shift_mask;

            let mut current = pos;
            let mut next = current;
            while current != 0 {
                dir_mask |= current;
                next = dir_shift(current, *shift);
                current = next & opponent_bits;
            }
            if next & current_bits != 0 {
                mask |= dir_mask ^ pos;
            }
        }

        if mask == 0 {
            return Err(OthelloError::IllegalMove);
        }

        match stone {
            Stone::Black => {
                self.black_stones |= mask | pos;
                self.white_stones ^= mask;
            }
            Stone::White => {
                self.white_stones |= mask | pos;
                self.black_stones ^= mask;
            }
        }
        Ok(())
    }

    /// Returns the bitboard representation of the specified player.
    ///
    /// The returned bitboard represents the Othello board. For a more detailed
    /// description, refer to the documentation of the [`OthelloBoard struct`].
    ///
    /// [`OthelloBoard struct`]: crate::othello::OthelloBoard
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{OthelloBoard, Stone};
    ///
    /// let board = OthelloBoard::standard();
    /// let black = board.bits_for(Stone::Black);
    /// let white = board.bits_for(Stone::White);
    /// // The two bitboards do not intersect
    /// assert_eq!(0, black & white);
    /// ```
    pub fn bits_for(&self, stone: Stone) -> u64 {
        match stone {
            Stone::Black => self.black_stones,
            Stone::White => self.white_stones,
        }
    }

    /// Checks whether or not a move is valid for the specified player.
    ///
    /// The specified bitboard must have one and only one bit set. If this is
    /// not true, the function will always return false.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{OthelloBoard, Stone};
    ///
    /// let board = OthelloBoard::standard();
    /// assert!(!board.is_legal_move(Stone::Black, 1_u64));
    /// ```
    pub fn is_legal_move(&self, stone: Stone, pos: u64) -> bool {
        if pos.count_ones() != 1 {
            return false;
        }
        let current_bits = self.bits_for(stone);
        let opponent_bits = self.bits_for(stone.flip());

        // Pos must be on an empty square to be legal
        if pos & (current_bits | opponent_bits) != 0 {
            return false;
        }

        for (i, shift) in SHIFT_DIRS.iter().enumerate() {
            let mut dir_mask = 0;
            let shift_mask = SHIFT_MASKS[i] & SHIFT_RAYS[pos.leading_zeros() as usize][i];
            let opponent_bits = opponent_bits & shift_mask;

            let mut current = pos;
            let mut next = current;
            while current != 0 {
                dir_mask |= current;
                next = dir_shift(current, *shift);
                current = next & opponent_bits;
            }
            if next & current_bits != 0 && dir_mask ^ pos != 0 {
                return true;
            }
        }
        false
    }

    /// Calculates and returns the set of all legal moves for the specified player.
    ///
    /// The returned bitboard represents the Othello board. For a more detailed
    /// description, refer to the documentation of the [`OthelloBoard struct`].
    ///
    /// [`OthelloBoard struct`]: crate::othello::OthelloBoard
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{OthelloBoard, Stone};
    ///
    /// let board = OthelloBoard::standard();
    /// let stone = Stone::Black;
    /// assert_eq!(4, board.moves_for(stone).count_ones());
    /// ```
    pub fn moves_for(&self, stone: Stone) -> u64 {
        let current_bits = self.bits_for(stone);
        let opponent_bits = self.bits_for(stone.flip());
        let empty_squares = self.empty_squares();

        let move_in_dir = |mask: u64, shift: i8| {
            let excluded: u64 = opponent_bits & mask;
            let mut m: u64 = dir_shift(current_bits, shift) & excluded;
            m |= dir_shift(m, shift) & excluded;
            m |= dir_shift(m, shift) & excluded;
            m |= dir_shift(m, shift) & excluded;
            m |= dir_shift(m, shift) & excluded;
            m |= dir_shift(m, shift) & excluded;
            dir_shift(m, shift) & empty_squares
        };

        let exclude_top_bottom = !(RANK_1 | RANK_8);
        let exclude_left_right = !(FILE_A | FILE_H);

        let mut moves = move_in_dir(exclude_top_bottom, -8);
        moves |= move_in_dir(exclude_left_right, -7);
        moves |= move_in_dir(exclude_left_right, 1);
        moves |= move_in_dir(exclude_left_right, 9);
        moves |= move_in_dir(exclude_top_bottom, 8);
        moves |= move_in_dir(exclude_left_right, 7);
        moves |= move_in_dir(exclude_left_right, -1);
        moves | move_in_dir(exclude_left_right, -9)
    }

    /// Returns the set of all empty squares on the board.
    ///
    /// The returned bitboard represents the Othello board. For a more detailed
    /// description, refer to the documentation of the [`OthelloBoard struct`]
    ///
    /// [`OthelloBoard struct`]: crate::othello::OthelloBoard
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::OthelloBoard;
    ///
    /// let board = OthelloBoard::standard();
    /// assert_eq!(60, board.empty_squares().count_ones());
    /// ```
    pub fn empty_squares(&self) -> u64 {
        !(self.black_stones | self.white_stones)
    }

    /// Queries the board in the specified position after a stone.
    ///
    /// If the argument `pos` does not have exactly one bit set, the function
    /// will evaluate to None.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{OthelloBoard, Stone};
    ///
    /// let board = OthelloBoard::standard();
    /// let pos = 0x8000000;
    /// assert_eq!(Some(Stone::White), board.stone_at(pos));
    ///  ```
    pub fn stone_at(&self, pos: u64) -> Option<Stone> {
        if pos.count_ones() != 1 {
            None
        } else if self.black_stones & pos > 0 {
            Some(Stone::Black)
        } else if self.white_stones & pos > 0 {
            Some(Stone::White)
        } else {
            None
        }
    }

    /// Returns a struct that implements [`Display`] for customizing the display of Othello boards.
    ///
    /// Formatting options can be found in the docs for [`OthelloDisplay`].
    ///
    /// [`Display`]: std::fmt::Display
    /// [`OthelloDisplay`]: crate::othello::OthelloDisplay
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{OthelloBoard, Stone};
    ///
    /// let board = OthelloBoard::standard();
    /// println!("{}", board.display());
    ///  ```
    pub fn display(&self) -> OthelloDisplay {
        OthelloDisplay::new(self)
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg(feature = "serde")]
struct ShadowOthelloBoard {
    black_stones: u64,
    white_stones: u64,
}

#[cfg(feature = "serde")]
impl std::convert::TryFrom<ShadowOthelloBoard> for OthelloBoard {
    type Error = &'static str;

    fn try_from(unchecked: ShadowOthelloBoard) -> Result<Self, Self::Error> {
        // Simply delegate to the main TryFrom trait implementation
        OthelloBoard::try_from((unchecked.black_stones, unchecked.white_stones)).map_err(|_| {
            // While it would be possible to simply implement fmt::Display on OthelloError,
            // this solution leaves that trait open for future uses.
            // Furthermore, in this case, no other error will be reasonably returned.
            "Overlapping pieces detected"
        })
    }
}

impl Default for OthelloBoard {
    /// Returns a board with the standard opening position configured.
    ///
    /// Simply delegates to the [`standard`] constructor.
    ///
    /// [`standard`]: crate::othello::OthelloBoard::standard
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::OthelloBoard;
    ///
    /// assert_eq!(OthelloBoard::standard(), OthelloBoard::default());
    /// ```
    fn default() -> Self {
        Self::standard()
    }
}

impl TryFrom<(u64, u64)> for OthelloBoard {
    type Error = OthelloError;

    /// Returns a board built from the two specified bitboards.
    ///
    /// Returns an error if the two bitboards intersect.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{OthelloBoard, Stone};
    ///
    /// let board = OthelloBoard::standard();
    /// let black = board.bits_for(Stone::Black);
    /// let white = board.bits_for(Stone::White);
    ///
    /// // Quite a contrived example
    /// let board = OthelloBoard::try_from((black, white));
    /// assert_eq!(Ok(OthelloBoard::standard()), board);
    /// ```
    fn try_from(stones: (u64, u64)) -> Result<Self, Self::Error> {
        let (black_stones, white_stones) = stones;
        if black_stones & white_stones != 0 {
            return Err(OthelloError::PiecesOverlapping);
        }
        let board = Self {
            black_stones,
            white_stones,
        };
        Ok(board)
    }
}

/// This enum represents errors that may occur when using the Othello board.
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum OthelloError {
    /// Indicates that an illegal move was attempted.
    IllegalMove,
    /// Indicates that multiple moves were attempted at once.
    MultipleMovesAttempted,
    /// Indicates that the operation would have resulted in one or more stones overlapping.
    PiecesOverlapping,
}

/// Extension trait that extracts all set bits from a bitboard.
pub trait StoneExt: Sized {
    type Iter: Iterator<Item = Self>;
    /// Given a bitboard, extracts each bit set to one as its own bitboard.
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
    /// [`SquareExt`] is a similar extension trait which may be of use as well.
    ///
    /// [`SquareExt`]: crate::othello::SquareExt
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{OthelloBoard, StoneExt, Stone};
    ///
    /// let mut board = OthelloBoard::standard();
    /// let player = Stone::Black;
    /// let pos = board
    ///     .moves_for(player) // Returns bitboard
    ///     .stones() // Convert that into multiple bitboards
    ///     .next()
    ///     .unwrap(); // The standard Othello opening is guaranteed to have at
    ///                // least one valid move
    /// assert!(board.place_stone(player, pos).is_ok());
    ///  ```
    fn stones(&self) -> Self::Iter;
}

impl StoneExt for u64 {
    type Iter = Box<dyn Iterator<Item = Self>>;
    fn stones(&self) -> Self::Iter {
        let this = *self;
        let iter = MASKS.iter().map(move |m| m & this).filter(|m| *m != 0);

        Box::new(iter)
    }
}

/// Extension trait that extracts all bits from a bitboard.
pub trait SquareExt: Sized {
    type Iter: Iterator<Item = Self>;
    /// Given a bitboard, extracts each bit as its own bitboard.
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
    /// The iterator always return 64 bitboards since Othello has 64 positions.
    ///
    /// [`StoneExt`] is a similar extension trait which may be of use as well.
    ///
    /// [`StoneExt`]: crate::othello::StoneExt
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{OthelloBoard, SquareExt};
    ///
    /// let mut board = OthelloBoard::standard();
    /// let pos = u64::MAX // Full bitboard (all bits set to 1)
    ///     .squares() // Convert that into multiple bitboards
    ///     .next()
    ///     .unwrap(); // This iterator will always return 64 bitboards
    /// assert_eq!(2_u64.pow(63), pos);
    ///  ```
    fn squares(&self) -> Self::Iter;
}

impl SquareExt for u64 {
    type Iter = Box<dyn Iterator<Item = Self>>;
    fn squares(&self) -> Self::Iter {
        let this = *self;
        let iter = MASKS.iter().map(move |m| m & this);

        Box::new(iter)
    }
}

pub trait PositionExt {
    fn to_position(&self) -> Position;
}

// https://www.chessprogramming.org/General_Setwise_Operations#Generalized%20Shift
fn dir_shift(x: u64, shift: i8) -> u64 {
    if shift > 0 {
        x >> shift
    } else {
        x << -shift
    }
}
