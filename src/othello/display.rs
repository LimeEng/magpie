use crate::othello::{
    constants::{FILES, RANKS},
    Board, Position, Stone,
};

/// Helper struct to customize the printing of Othello boards.
///
/// Printing and thus visualizing the board is useful for both debugging. The
/// output can be customized by choosing formatting options and whether or not
/// legal moves should be shown for a specific player.
///
/// # Examples
/// ```rust
/// use magpie::othello::{Format, Board, Stone};
///
/// let board = Board::standard();
/// println!("{}", board.display());
/// println!("{}", board.display().with_format(Format::Compact));
/// println!("{}", board.display().with_stone(Stone::Black));
/// println!(
///     "{}",
///     board
///         .display()
///         .with_stone(Stone::White)
///         .with_format(Format::Standard)
/// );
/// ```
#[derive(Clone)]
pub struct BoardDisplay<'a> {
    board: &'a Board,
    display: Format,
    stone: Option<Stone>,
}

/// Represents the different formatting options available when displaying an
/// Othello board.
#[derive(Copy, Clone)]
pub enum Format {
    /// Formats the board compactly.
    Compact,
    /// Standard formatting.
    Standard,
}

impl<'a> BoardDisplay<'a> {
    pub(crate) fn new(board: &'a Board) -> Self {
        Self {
            board,
            display: Format::Standard,
            stone: None,
        }
    }

    /// Displays the board with the legal moves for the specified stone.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Board, Stone};
    ///
    /// let board = Board::standard();
    /// println!("{}", board.display().with_stone(Stone::Black));
    /// ```
    #[must_use]
    pub fn with_stone(&self, stone: Stone) -> Self {
        Self {
            board: self.board,
            display: self.display,
            stone: Some(stone),
        }
    }

    /// Displays the board with the specified formatting.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Format, Board};
    ///
    /// let board = Board::standard();
    /// println!("{}", board.display().with_format(Format::Compact));
    /// ```
    #[must_use]
    pub fn with_format(&self, display: Format) -> Self {
        Self {
            board: self.board,
            display,
            stone: self.stone,
        }
    }
}

impl std::fmt::Display for BoardDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        display(f, self.board, self.stone, self.display)
    }
}

fn display(
    f: &mut std::fmt::Formatter,
    board: &Board,
    stone: Option<Stone>,
    display: Format,
) -> std::fmt::Result {
    let legal_moves = stone.map_or(0.into(), |stone| board.moves_for(stone));
    let char_at = |rank: usize, file: usize| {
        let pos = RANKS[rank] & FILES[file];
        let pos = Position::new_unchecked(pos);
        board
            .stone_at(pos)
            .map(|stone| match stone {
                Stone::White => "W",
                Stone::Black => "B",
            })
            .or({
                if legal_moves & pos > 0 {
                    Some("*")
                } else {
                    None
                }
            })
            .unwrap_or(match display {
                Format::Compact => ".",
                Format::Standard => " ",
            })
    };

    match display {
        Format::Compact => {
            writeln!(f, "   ABCDEFGH")?;
            writeln!(f, "  +--------+")?;
            for rank in 0..8 {
                write!(f, "{} |", rank + 1)?;
                for file in 0..8 {
                    write!(f, "{}", char_at(rank, file))?;
                }
                writeln!(f, "|")?;
            }
            writeln!(f, "  +--------+")
        }
        Format::Standard => {
            let top_row = "    A   B   C   D   E   F   G   H";
            let horizontal = format!("  +{}", "---+".repeat(8));

            writeln!(f, "{top_row}")?;
            for rank in 0..8 {
                writeln!(f, "{horizontal}")?;
                write!(f, "{} |", rank + 1)?;
                for file in 0..8 {
                    write!(f, " {} |", char_at(rank, file))?;
                }
                writeln!(f)?;
            }
            writeln!(f, "{horizontal}")
        }
    }
}
