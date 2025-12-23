use crate::othello::{
    Bitboard, Position, Stone,
    constants::{
        BLACK_START_POS, FILE_A, FILE_H, RANK_1, RANK_8, SHIFT_DIRS, SHIFT_MASKS, SHIFT_RAYS,
        WHITE_START_POS,
    },
    display::BoardDisplay,
};
use std::{error, fmt};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an Othello board and provides convenient functions to manipulate it.
///
/// The board is represented by two bitboards, one for black and one for white.
/// Each bitboard is a 64-bit unsigned integer, where each bit encodes if a
/// stone for that player exists on that space. As can be seen in the graphic
/// below, MSB denotes A1 while LSB denotes H8.
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
#[cfg_attr(feature = "serde", serde(try_from = "ShadowBoard"))]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Board {
    black_stones: Bitboard,
    white_stones: Bitboard,
}

impl Board {
    /// Returns a completely empty board.
    ///
    /// This can be useful for setting up specific scenarios but for most
    /// users, the [`standard`] constructor will be more useful.
    ///
    /// [`standard`]: crate::othello::Board::standard
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Board;
    ///
    /// let board = Board::empty();
    /// assert_eq!(64, board.empty_squares().count_set());
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        Self {
            black_stones: Bitboard::EMPTY,
            white_stones: Bitboard::EMPTY,
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
    /// use magpie::othello::Board;
    ///
    /// let board = Board::standard();
    /// assert_eq!(60, board.empty_squares().count_set());
    /// ```
    #[must_use]
    pub fn standard() -> Self {
        Self {
            black_stones: BLACK_START_POS.into(),
            white_stones: WHITE_START_POS.into(),
        }
    }

    /// Evaluates if the board is in a consistent state
    ///
    /// Consistency is defined as whether or not multiple stones occupy
    /// the same square.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Bitboard, Board, Stone};
    ///
    /// let mut board = Board::empty();
    /// // The board should be valid
    /// assert!(board.is_valid());
    ///
    /// // Here multiple stones are placed on the same
    /// // squares, which is not valid
    /// board.place_stone_unchecked(Stone::Black, Bitboard::FILLED);
    /// board.place_stone_unchecked(Stone::White, Bitboard::FILLED);
    /// assert!(!board.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.black_stones & self.white_stones == 0
    }

    /// Places stones in the specified positions.
    ///
    /// Multiple stones may be placed at
    /// once, but no other stones will be flipped as during normal play.
    /// No restrictions are placed on the number or position of the pieces
    /// which may result in an invalid board where multiple stones occupy the
    /// same square. It is the responsibility of the caller to ensure that the
    /// board remains consistent.
    ///
    /// [`remove_stone_unchecked`] may be of use as well.
    ///
    /// [`remove_stone_unchecked`]: crate::othello::Board::remove_stone_unchecked
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Stone};
    ///
    /// let mut board = Board::empty();
    /// let pos = 1_u64.try_into().unwrap();
    /// board.place_stone_unchecked(Stone::Black, pos);
    /// assert_ne!(board, Board::empty());
    /// ```
    pub fn place_stone_unchecked(&mut self, stone: Stone, pos: Bitboard) {
        match stone {
            Stone::Black => self.black_stones |= pos,
            Stone::White => self.white_stones |= pos,
        }
    }

    /// Removes stones in the specified positions.
    ///
    /// Multiple stones may be removed at once.
    ///
    /// [`place_stone_unchecked`] may be of use as well.
    ///
    /// [`place_stone_unchecked`]: crate::othello::Board::place_stone_unchecked
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Stone};
    ///
    /// let mut board = Board::standard();
    /// let black_stones = board.bits_for(Stone::Black);
    /// let white_stones = board.bits_for(Stone::White);
    /// board.remove_stone_unchecked(Stone::Black, black_stones);
    /// board.remove_stone_unchecked(Stone::White, white_stones);
    /// assert_eq!(board, Board::empty());
    /// ```
    pub fn remove_stone_unchecked(&mut self, stone: Stone, pos: Bitboard) {
        match stone {
            Stone::Black => self.black_stones &= !pos,
            Stone::White => self.white_stones &= !pos,
        }
    }

    /// Places a stone in the specified position and updates the board accordingly.
    ///
    /// It is the responsibility of the caller to ensure that the move is legal.
    /// Playing an illegal move will result in an incorrect board state.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Stone};
    ///
    /// let mut board = Board::standard();
    /// let player = Stone::Black;
    /// let pos = board
    ///     .moves_for(player)
    ///     .hot_bits()
    ///     .next()
    ///     .unwrap();
    /// board.play(Stone::Black, pos);
    /// assert_ne!(board, Board::standard());
    /// ```
    pub fn play(&mut self, stone: Stone, pos: Position) {
        let pos: Bitboard = pos.into();
        let current_bits = self.bits_for(stone);
        let opponent_bits = self.bits_for(stone.flip());

        let mut mask = 0;
        for (i, shift) in SHIFT_DIRS.iter().enumerate() {
            let mut dir_mask = 0;
            let shift_mask = SHIFT_MASKS[i] & SHIFT_RAYS[pos.raw().leading_zeros() as usize][i];
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
    }

    /// Returns the bitboard representation of the specified player.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Stone};
    ///
    /// let board = Board::standard();
    /// let black = board.bits_for(Stone::Black);
    /// let white = board.bits_for(Stone::White);
    /// // The two bitboards do not intersect
    /// assert_eq!(0, black & white);
    /// // They do contain the same number of stones
    /// assert_eq!(black.count_set(), white.count_set());
    /// ```
    #[must_use]
    pub fn bits_for(&self, stone: Stone) -> Bitboard {
        match stone {
            Stone::Black => self.black_stones,
            Stone::White => self.white_stones,
        }
    }

    /// Checks whether or not a move is valid for the specified player.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Stone};
    ///
    /// let board = Board::standard();
    /// let pos = 1_u64.try_into().unwrap();
    /// assert!(!board.is_legal_move(Stone::Black, pos));
    /// ```
    #[must_use]
    pub fn is_legal_move(&self, stone: Stone, pos: Position) -> bool {
        let pos = Bitboard::from(pos);
        let current_bits = self.bits_for(stone);
        let opponent_bits = self.bits_for(stone.flip());

        // Pos must be on an empty square to be legal
        if pos & (current_bits | opponent_bits) != 0 {
            return false;
        }

        for (i, shift) in SHIFT_DIRS.iter().enumerate() {
            let mut dir_mask = 0;
            let shift_mask = SHIFT_MASKS[i] & SHIFT_RAYS[pos.raw().leading_zeros() as usize][i];
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
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Stone};
    ///
    /// let board = Board::standard();
    /// let stone = Stone::Black;
    /// assert_eq!(4, board.moves_for(stone).count_set());
    /// ```
    #[must_use]
    pub fn moves_for(&self, stone: Stone) -> Bitboard {
        let current_bits = self.bits_for(stone);
        let opponent_bits = self.bits_for(stone.flip());
        let empty_squares = self.empty_squares();

        let move_in_dir = |mask: u64, shift: i8| {
            let excluded: Bitboard = opponent_bits & mask;
            let mut m: Bitboard = dir_shift(current_bits, shift) & excluded;
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
    /// # Examples
    /// ```rust
    /// use magpie::othello::Board;
    ///
    /// let board = Board::standard();
    /// assert_eq!(60, board.empty_squares().count_set());
    /// ```
    #[must_use]
    pub fn empty_squares(&self) -> Bitboard {
        !(self.black_stones | self.white_stones)
    }

    /// Queries the board at the specified position for the presence of a stone.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Stone};
    ///
    /// let board = Board::standard();
    /// let pos = 0x8000000.try_into().unwrap();
    /// assert_eq!(Some(Stone::White), board.stone_at(pos));
    ///  ```
    #[must_use]
    pub fn stone_at(&self, pos: Position) -> Option<Stone> {
        if self.black_stones & pos > 0 {
            Some(Stone::Black)
        } else if self.white_stones & pos > 0 {
            Some(Stone::White)
        } else {
            None
        }
    }

    /// Returns a struct that implements [`Display`] for customizing the display of Othello boards.
    ///
    /// [`Display`]: std::fmt::Display
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Stone};
    ///
    /// let board = Board::standard();
    /// println!("{}", board.display());
    ///  ```
    #[must_use]
    pub fn display(&'_ self) -> BoardDisplay<'_> {
        BoardDisplay::new(self)
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg(feature = "serde")]
struct ShadowBoard {
    black_stones: u64,
    white_stones: u64,
}

#[cfg(feature = "serde")]
impl std::convert::TryFrom<ShadowBoard> for Board {
    type Error = BoardError;

    fn try_from(unchecked: ShadowBoard) -> Result<Self, Self::Error> {
        Board::try_from((unchecked.black_stones, unchecked.white_stones))
    }
}

impl Default for Board {
    /// Returns a board with the standard opening position configured.
    ///
    /// Simply delegates to the [`standard`] constructor.
    ///
    /// [`standard`]: crate::othello::Board::standard
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Board;
    ///
    /// assert_eq!(Board::standard(), Board::default());
    /// ```
    fn default() -> Self {
        Self::standard()
    }
}

impl TryFrom<(u64, u64)> for Board {
    type Error = BoardError;

    /// Returns a board built from the two specified bitboards.
    ///
    /// Returns an error if the two bitboards intersect.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Stone};
    ///
    /// let board = Board::standard();
    /// let black = board.bits_for(Stone::Black);
    /// let white = board.bits_for(Stone::White);
    ///
    /// // Quite a contrived example
    /// let board = Board::try_from((black, white));
    /// assert_eq!(Ok(Board::standard()), board);
    /// ```
    fn try_from(stones: (u64, u64)) -> Result<Self, Self::Error> {
        let (black_stones, white_stones) = stones;
        if black_stones & white_stones != 0 {
            return Err(BoardError::OverlappingPieces);
        }
        let board = Self {
            black_stones: black_stones.into(),
            white_stones: white_stones.into(),
        };
        Ok(board)
    }
}

impl TryFrom<(Bitboard, Bitboard)> for Board {
    type Error = BoardError;

    /// Returns a board built from the two specified bitboards.
    ///
    /// Returns an error if the two bitboards intersect.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Stone};
    ///
    /// let board = Board::standard();
    /// let black = board.bits_for(Stone::Black);
    /// let white = board.bits_for(Stone::White);
    ///
    /// // Quite a contrived example
    /// let board = Board::try_from((black, white));
    /// assert_eq!(Ok(Board::standard()), board);
    /// ```
    fn try_from(stones: (Bitboard, Bitboard)) -> Result<Self, Self::Error> {
        let (black_stones, white_stones) = stones;
        Board::try_from((black_stones.raw(), white_stones.raw()))
    }
}

/// This enum represents errors that may occur when using a Othello board.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BoardError {
    /// Indicates that the operation would have resulted in two or more stones overlapping.
    OverlappingPieces,
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OverlappingPieces => write!(f, "overlapping pieces"),
        }
    }
}

impl error::Error for BoardError {}

// https://www.chessprogramming.org/General_Setwise_Operations#Generalized_Shift
fn dir_shift(x: Bitboard, shift: i8) -> Bitboard {
    if shift > 0 { x >> shift } else { x << -shift }
}
