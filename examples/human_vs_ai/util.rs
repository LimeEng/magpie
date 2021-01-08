use crate::coord::Coord;
use magpie::othello::{OthelloBoard, Stone};
use std::convert::TryFrom;

/// Prints the specified board with optional legal moves included
///
/// If a stone is specified, this function prints the legal moves for that
/// stone.
pub fn display(board: &OthelloBoard, stone: Option<Stone>) -> String {
    let legal_moves = stone.map(|stone| board.legal_moves_for(stone)).unwrap_or(0);
    let char_at = |rank: usize, file: usize| {
        let pos = Coord::try_from((rank as u8, file as u8))
            .unwrap()
            .as_bitboard();
        board
            .stone_at(pos)
            .map(|stone| match stone {
                Stone::White => "W",
                Stone::Black => "B",
            })
            .or_else(|| Some(if legal_moves & pos > 0 { "*" } else { " " }))
            .unwrap_or(" ")
    };

    let horizontal = format!("  +{}", "---+".repeat(8));
    let top_row = "    A   B   C   D   E   F   G   H\n";

    let display = (0..8)
        .map(|rank| {
            format!(
                "{}{} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                format!("{}\n", horizontal),
                rank + 1,
                char_at(rank, 0),
                char_at(rank, 1),
                char_at(rank, 2),
                char_at(rank, 3),
                char_at(rank, 4),
                char_at(rank, 5),
                char_at(rank, 6),
                char_at(rank, 7)
            )
        })
        .fold("".to_owned(), |acc, val| acc + &val);

    let display = &[top_row, &display, &horizontal].concat();
    display.to_string()
}
