use std::cmp::Ordering;

use crate::othello::{Bitboard, Board, BoardDisplay, OthelloError, Position, Stone};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an Othello game.
///
/// To interact with the game it is useful to understand the bitboards that
/// the board uses. The [`Board`]-struct documents these.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Game {
    board: Board,
    next_player: Stone,
    passed_last_turn: bool,
}

/// This enum represents all states the game can be in.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Status {
    /// Indicates that the game has concluded with the specified winner.
    Win(Stone),
    /// Indicates that the game has concluded in a draw.
    Draw,
    /// Indicates that the game is still in progress.
    Progressing,
}

impl Game {
    /// Returns a game with the standard opening position configured.
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
    /// use magpie::othello::Game;
    ///
    /// let game = Game::new();
    /// assert_eq!(60, game.empty_squares().count_set());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Game::from_state(Board::standard(), Stone::Black, false).unwrap()
    }

    /// Returns a game with the specified parameters set.
    ///
    /// If the supplied board is invalid an error will be returned instead.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Game, Stone};
    ///
    /// let board = Board::empty();
    ///
    /// let game = Game::from_state(board, Stone::Black, false)
    ///                .expect("The supplied board is invalid");
    /// assert_eq!(game.board(), Board::empty());
    /// ```
    pub fn from_state(
        board: Board,
        next_player: Stone,
        passed_last_turn: bool,
    ) -> Result<Self, OthelloError> {
        if board.is_valid() {
            Ok(Self {
                board,
                next_player,
                passed_last_turn,
            })
        } else {
            Err(OthelloError::PiecesOverlapping)
        }
    }

    /// Returns the stone of the current player.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Game, Stone};
    ///
    /// let game = Game::new();
    /// assert!(game.current_turn() == Stone::Black);
    /// ```
    #[must_use]
    pub fn current_turn(&self) -> Stone {
        self.next_player
    }

    /// The current player will pass their turn.
    ///
    /// If two players pass in a row the game is considered concluded.
    /// Do note that it is up to the caller to determine this by
    /// calling [`.status()`].
    ///
    /// [`.status()`]: crate::othello::Game::status
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Game, Stone};
    ///
    /// let mut game = Game::new();
    /// game.pass_turn();
    /// assert!(game.current_turn() == Stone::White);
    /// ```
    pub fn pass_turn(&mut self) {
        self.next_player = self.next_player.flip();
        self.passed_last_turn = true;
    }

    /// Reports the status of the game.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Game, Status, Stone};
    ///
    /// let mut game = Game::new();
    /// assert!(game.status() == Status::Progressing);
    /// ```
    #[must_use]
    pub fn status(&self) -> Status {
        let finished = self.passed_last_turn && self.board.moves_for(self.next_player).is_empty();
        if finished {
            let black_stones = self.board.bits_for(Stone::Black).count_set();
            let white_stones = self.board.bits_for(Stone::White).count_set();

            match black_stones.cmp(&white_stones) {
                Ordering::Greater => Status::Win(Stone::Black),
                Ordering::Less => Status::Win(Stone::White),
                Ordering::Equal => Status::Draw,
            }
        } else {
            Status::Progressing
        }
    }

    /// Places a stone in the specified position and updates the board accordingly.
    ///
    /// If the move is illegal an error will be returned leaving the game
    /// untouched.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Game, Stone};
    ///
    /// let mut game = Game::new();
    /// let pos = game
    ///     .moves()
    ///     .hot_bits()
    ///     .next()
    ///     .unwrap();
    /// assert!(game.play(pos).is_ok());
    /// ```
    pub fn play(&mut self, pos: Position) -> Result<(), OthelloError> {
        if self.is_legal_move(pos) {
            self.board.play(self.next_player, pos);
            self.next_player = self.next_player.flip();
            Ok(())
        } else {
            Err(OthelloError::IllegalMove)
        }
    }

    /// Returns a copy of the internal board used in this game.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Game};
    ///
    /// let mut game = Game::new();
    /// let board = game.board();
    /// assert_eq!(board, Board::standard());
    /// ```
    #[must_use]
    pub fn board(&self) -> Board {
        self.board.clone()
    }

    /// Returns whether or not the previous player passed their turn.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Game;
    ///
    /// let mut game = Game::new();
    /// assert_eq!(game.passed_last_turn(), false);
    /// ```
    #[must_use]
    pub fn passed_last_turn(&self) -> bool {
        self.passed_last_turn
    }

    /// Checks if the supplied position is a legal move for the current player.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Game;
    ///
    /// let mut game = Game::new();
    /// let pos = game
    ///     .moves()
    ///     .hot_bits()
    ///     .next()
    ///     .unwrap();
    /// assert_eq!(game.is_legal_move(pos), true);
    /// ```
    #[must_use]
    pub fn is_legal_move(&self, pos: Position) -> bool {
        self.board.is_legal_move(self.next_player, pos)
    }

    /// Calculates and returns the set of all legal moves for the current player.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Game;
    ///
    /// let game = Game::new();
    /// assert_eq!(4, game.moves().count_set());
    /// ```
    #[must_use]
    pub fn moves(&self) -> Bitboard {
        self.board.moves_for(self.next_player)
    }

    /// Returns a bitboard representing all stones for the specified player.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Game, Stone};
    ///
    /// let mut game = Game::new();
    /// let black = game.bits_for(Stone::Black);
    /// let white = game.bits_for(Stone::White);
    /// // The two bitboards do not intersect
    /// assert_eq!(0, black & white);
    /// // They do contain the same number of stones
    /// assert_eq!(black.count_set(), white.count_set());
    /// ```
    #[must_use]
    pub fn bits_for(&self, stone: Stone) -> Bitboard {
        self.board.bits_for(stone)
    }

    /// Returns a bitboard representing the set of all empty squares on the board.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Game;
    ///
    /// let game = Game::new();
    /// assert_eq!(60, game.empty_squares().count_set());
    /// ```
    #[must_use]
    pub fn empty_squares(&self) -> Bitboard {
        self.board.empty_squares()
    }

    /// Queries the board at the specified position for the presence of a stone.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Game, Stone};
    ///
    /// let game = Game::new();
    /// let pos = 0x8000000.try_into().unwrap();
    /// assert_eq!(Some(Stone::White), game.stone_at(pos));
    ///  ```
    #[must_use]
    pub fn stone_at(&self, pos: Position) -> Option<Stone> {
        self.board.stone_at(pos)
    }

    /// Returns a struct that implements [`Display`] for customizing the display of Othello boards.
    ///
    /// [`Display`]: std::fmt::Display
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Game, Stone};
    ///
    /// let game = Game::new();
    /// println!("{}", game.display());
    ///  ```
    #[must_use]
    pub fn display(&self) -> BoardDisplay {
        self.board.display()
    }
}

impl Default for Game {
    /// Returns a game with the standard opening position configured.
    ///
    /// Simply delegates to the [`new`] constructor.
    ///
    /// [`new`]: crate::othello::Game::new
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Game;
    ///
    /// assert_eq!(Game::new(), Game::default());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}
