use crate::othello::{OthelloBoard, Stone};

/// Helper struct to customize the printing of Othello boards.
///
/// Printing and thus visualizing the board is useful for both debugging. The
/// output can be customized by choosing formatting options and whether or not
/// legal moves should be shown for a specific player.
///
/// # Examples
/// ```rust
/// use magpie::othello::{Format, OthelloBoard, Stone};
///
/// let board = OthelloBoard::standard();
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
pub struct OthelloDisplay<'a> {
    board: &'a OthelloBoard,
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

impl<'a> OthelloDisplay<'a> {
    pub(crate) fn new(board: &OthelloBoard) -> OthelloDisplay {
        OthelloDisplay {
            board,
            display: Format::Standard,
            stone: None,
        }
    }

    /// Displays the board with the legal moves for the specified stone.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Format, OthelloBoard, Stone};
    ///
    /// let board = OthelloBoard::standard();
    /// println!("{}", board.display().with_stone(Stone::Black));
    /// ```
    pub fn with_stone(&self, stone: Stone) -> OthelloDisplay {
        OthelloDisplay {
            board: &self.board,
            display: self.display,
            stone: Some(stone),
        }
    }

    /// Displays the board with the specified formatting.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::{Format, OthelloBoard, Stone};
    ///
    /// let board = OthelloBoard::standard();
    /// println!("{}", board.display().with_format(Format::Compact));
    /// ```
    pub fn with_format(&self, display: Format) -> OthelloDisplay {
        OthelloDisplay {
            board: &self.board,
            display,
            stone: self.stone,
        }
    }
}

impl std::fmt::Display for OthelloDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        display(f, &self.board, self.stone, self.display)
    }
}

fn display(
    f: &mut std::fmt::Formatter,
    board: &OthelloBoard,
    stone: Option<Stone>,
    display: Format,
) -> std::fmt::Result {
    let legal_moves = stone.map(|stone| board.legal_moves_for(stone)).unwrap_or(0);
    let char_at = |rank: usize, file: usize| {
        let pos = RANKS[rank] & FILES[file];
        board
            .stone_at(pos)
            .map(|stone| match stone {
                Stone::White => "W",
                Stone::Black => "B",
            })
            .or_else(|| {
                if legal_moves & pos > 0 {
                    Some("*")
                } else {
                    None
                }
            })
            .unwrap_or_else(|| match display {
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

            writeln!(f, "{}", top_row)?;
            for rank in 0..8 {
                writeln!(f, "{}", horizontal)?;
                write!(f, "{} |", rank + 1)?;
                for file in 0..8 {
                    write!(f, " {} |", char_at(rank, file))?;
                }
                writeln!(f)?;
            }
            writeln!(f, "{}", horizontal)
        }
    }
}

const FILE_A: u64 = 0x80_80_80_80_80_80_80_80;
const FILE_B: u64 = 0x40_40_40_40_40_40_40_40;
const FILE_C: u64 = 0x20_20_20_20_20_20_20_20;
const FILE_D: u64 = 0x10_10_10_10_10_10_10_10;
const FILE_E: u64 = 0x08_08_08_08_08_08_08_08;
const FILE_F: u64 = 0x04_04_04_04_04_04_04_04;
const FILE_G: u64 = 0x02_02_02_02_02_02_02_02;
const FILE_H: u64 = 0x01_01_01_01_01_01_01_01;
const FILES: [u64; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];

const RANK_1: u64 = 0xff_00_00_00_00_00_00_00;
const RANK_2: u64 = 0x00_ff_00_00_00_00_00_00;
const RANK_3: u64 = 0x00_00_ff_00_00_00_00_00;
const RANK_4: u64 = 0x00_00_00_ff_00_00_00_00;
const RANK_5: u64 = 0x00_00_00_00_ff_00_00_00;
const RANK_6: u64 = 0x00_00_00_00_00_ff_00_00;
const RANK_7: u64 = 0x00_00_00_00_00_00_ff_00;
const RANK_8: u64 = 0x00_00_00_00_00_00_00_ff;
const RANKS: [u64; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];
