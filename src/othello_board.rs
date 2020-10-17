use crate::direction::Direction;
use crate::stone::Stone;

#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;

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
/// [`place_stone`]: #method.place_stone
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
    /// [`standard`]: #method.standard
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello_board::OthelloBoard;
    ///
    /// let board = OthelloBoard::empty();
    /// ```
    pub fn empty() -> OthelloBoard {
        OthelloBoard {
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
    /// use magpie::othello_board::OthelloBoard;
    ///
    /// let board = OthelloBoard::standard();
    /// ```
    pub fn standard() -> OthelloBoard {
        OthelloBoard {
            black_stones: BLACK_START_POS,
            white_stones: WHITE_START_POS,
        }
    }

    /// Returns a board built from the two specified bitboards.
    ///
    /// Returns an error if the two bitboards intersect.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello_board::OthelloBoard;
    /// use magpie::stone::Stone;
    ///
    /// let board = OthelloBoard::standard();
    /// let black = board.bits_for(Stone::Black);
    /// let white = board.bits_for(Stone::White);
    ///
    /// // Quite a contrived example
    /// let board = OthelloBoard::from_state(black, white).unwrap();
    /// ```
    pub fn from_state(black_stones: u64, white_stones: u64) -> Result<OthelloBoard, OthelloError> {
        if black_stones & white_stones != 0 {
            return Err(OthelloError::PiecesOverlapping);
        }
        let board = OthelloBoard {
            black_stones,
            white_stones,
        };
        Ok(board)
    }

    /// Places a stone in the specified position, which may be several at a time.
    ///
    /// Unlike the similar [`place_stone`] function, this function places no
    /// restrictions on the `pos` argument. Multiple stones may be placed at
    /// once, but no other stones will be flipped as during normal play.
    /// The only check is regarding whether or not any of the stones would be
    /// placed on top of a stone of the opposite color, and if so, returns an
    /// error leaving the board untouched.
    ///
    ///  [`place_stone`]: #method.place_stone
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello_board::OthelloBoard;
    /// use magpie::stone::Stone;
    ///
    /// let mut board = OthelloBoard::empty();
    /// board.place_stone_unchecked(Stone::Black, 1_u64).unwrap()
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

    /// Places a stone in the specified position and updates the board accordingly.
    ///
    /// If the argument `pos` does not have exactly one bit set or the move is
    /// not legal, an error will be returned, leaving the board untouched.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello_board::OthelloBoard;
    /// use magpie::stone::Stone;
    ///
    /// let mut board = OthelloBoard::standard();
    /// board.place_stone(Stone::Black, 1_u64);
    /// ```
    pub fn place_stone(&mut self, stone: Stone, pos: u64) -> Result<(), OthelloError> {
        if pos.count_ones() != 1 {
            return Err(OthelloError::MultipleMovesAttempted);
        }
        if !self.is_legal_move(stone, pos) {
            return Err(OthelloError::IllegalMove);
        }

        let mut mask = 0;
        let current_bits = self.bits_for(stone);
        let opponent_bits = self.bits_for(stone.flip());
        for dir in Direction::cardinals() {
            let mut dir_mask = 0;
            let mut candidates = dir.shift(pos) & opponent_bits;
            let mut valid_direction = false;
            while candidates != 0 {
                dir_mask |= candidates;
                let is_own_piece = dir.shift(candidates) & current_bits != 0;
                candidates = dir.shift(candidates) & opponent_bits;
                if candidates == 0 && is_own_piece {
                    valid_direction = true;
                }
            }
            if valid_direction {
                mask |= dir_mask;
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
        Ok(())
    }

    /// Returns the bitboard representation of the specified player.
    ///
    /// The returned bitboard represents the Othello board. For a more detailed
    /// description, refer to the documentation of the [`OthelloBoard struct`].
    ///
    /// [`OthelloBoard struct`]: ../othello_board/struct.OthelloBoard.html
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello_board::OthelloBoard;
    /// use magpie::stone::Stone;
    ///
    /// let board = OthelloBoard::standard();
    /// let black = board.bits_for(Stone::Black);
    /// let white = board.bits_for(Stone::White);
    /// // The two bitboards do not intersect
    /// assert_eq!(black & white, 0);
    /// ```
    pub fn bits_for(&self, stone: Stone) -> u64 {
        match stone {
            Stone::Black => self.black_stones,
            Stone::White => self.white_stones,
        }
    }

    /// Checks whether or not a move is valid for the specified player
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello_board::OthelloBoard;
    /// use magpie::stone::Stone;
    ///
    /// let board = OthelloBoard::standard();
    /// assert_eq!(board.is_legal_move(Stone::Black, 1_u64), false);
    /// ```
    pub fn is_legal_move(&self, stone: Stone, pos: u64) -> bool {
        (pos & self.legal_moves_for(stone)) != 0
    }

    /// Calculates and returns the set of all legal moves for the specified player.
    ///
    /// The returned bitboard represents the Othello board. For a more detailed
    /// description, refer to the documentation of the [`OthelloBoard struct`].
    ///
    /// [`OthelloBoard struct`]: ../othello_board/struct.OthelloBoard.html
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello_board::OthelloBoard;
    /// use magpie::stone::Stone;
    ///
    /// let board = OthelloBoard::standard();
    /// let stone = Stone::Black;
    /// println!("{:?} has {} legal moves", stone, board.legal_moves_for(stone).count_ones());
    /// ```
    pub fn legal_moves_for(&self, stone: Stone) -> u64 {
        let empty_cells = self.empty_cells();
        let current_bits = self.bits_for(stone);
        let opponent_bits = self.bits_for(stone.flip());

        let mut moves = 0;
        for dir in Direction::cardinals() {
            let mut candidates = dir.shift(current_bits) & opponent_bits;
            while candidates != 0 {
                moves |= empty_cells & dir.shift(candidates);
                candidates = dir.shift(candidates) & opponent_bits;
            }
        }
        moves
    }

    /// Returns the set of all empty cells on the board.
    ///
    /// The returned bitboard represents the Othello board. For a more detailed
    /// description, refer to the documentation of the [`OthelloBoard struct`]
    ///
    /// [`OthelloBoard struct`]: ../othello_board/struct.OthelloBoard.html
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello_board::OthelloBoard;
    ///
    /// let board = OthelloBoard::standard();
    /// println!("Number of free cells: {}", board.empty_cells().count_ones());
    /// ```
    pub fn empty_cells(&self) -> u64 {
        (!self.black_stones) & (!self.white_stones)
    }

    /// Queries the board in the specified position after a stone.
    ///
    /// If the argument `pos` does not have exactly one bit set, the function
    /// will evaluate to None.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello_board::OthelloBoard;
    /// use magpie::stone::Stone;
    ///
    /// let board = OthelloBoard::standard();
    /// let pos = 0x8000000;
    /// match board.stone_at(pos) {
    ///     Some(stone) => println!("Found stone: {:?}", stone),
    ///     None => println!("Nothing found at that position"),
    /// }
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
}

/// This enum represents errors that may occur when using the Othello board.
#[derive(Debug)]
pub enum OthelloError {
    /// Indicates that an illegal move was attempted.
    IllegalMove,
    /// Indicates that multiple moves were attempted at once.
    MultipleMovesAttempted,
    /// Indicates that the operation would have resulted in one or more stones overlapping.
    PiecesOverlapping,
}

impl Direction {
    fn shift(&self, pos: u64) -> u64 {
        use Direction::*;
        match self {
            N => (pos << 8),
            NE => (pos << 7) & !LEFT_MASK,
            E => (pos >> 1) & !LEFT_MASK,
            SE => (pos >> 9) & !LEFT_MASK,
            S => (pos >> 8),
            SW => (pos >> 7) & !RIGHT_MASK,
            W => (pos << 1) & !RIGHT_MASK,
            NW => (pos << 9) & !RIGHT_MASK,
        }
    }
}

const BLACK_START_POS: u64 = 0x00_00_00_08_10_00_00_00;
const WHITE_START_POS: u64 = 0x00_00_00_10_08_00_00_00;

const LEFT_MASK: u64 = 0x80_80_80_80_80_80_80_80;
const RIGHT_MASK: u64 = 0x01_01_01_01_01_01_01_01;
