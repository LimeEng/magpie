#[cfg(kani)]
use crate::kani_common::{ShadowBoard, ShadowGame};
#[cfg(kani)]
use magpie::othello::{Bitboard, Board, Game, Position, Stone};

#[cfg(kani)]
mod kani_common;

#[cfg(kani)]
#[kani::proof]
fn legal_moves_should_place() {
    // Check so that all legal moves returned can actually be placed
    let game = Game::try_from(kani::any::<ShadowGame>()).unwrap();

    let result = game
        .moves()
        .hot_bits()
        .map(|pos| game.clone().play(pos))
        .all(|result| result.is_ok());
    assert!(result);
}

#[cfg(kani)]
#[kani::proof]
fn illegal_moves_should_not_place() {
    // Check so that all moves not contained in the set of legal moves
    // cannot actually be placed
    let game = Game::try_from(kani::any::<ShadowGame>()).unwrap();

    let legal_positions = game.moves();

    let failed = Bitboard::from(u64::MAX)
        .bits()
        .filter(|pos| *pos & legal_positions == 0)
        .filter_map(|pos| Position::try_from(pos).ok())
        .map(|pos| game.clone().play(pos))
        .any(|result| result.is_ok());

    assert!(!failed);
}

#[cfg(kani)]
#[kani::proof]
fn legal_moves_should_be_legal() {
    // Check so that all legal moves returned can be individually verified as legal

    let game = Game::try_from(kani::any::<ShadowGame>()).unwrap();

    let result = game.moves().hot_bits().all(|pos| game.is_legal_move(pos));
    assert!(result);
}

#[cfg(kani)]
#[kani::proof]
fn illegal_moves_should_be_illegal() {
    // Check so that all moves not contained in the set of legal moves
    // actually is illegal
    let game = Game::try_from(kani::any::<ShadowGame>()).unwrap();

    let legal_positions = game.moves();

    let failed = Bitboard::from(u64::MAX)
        .bits()
        .filter(|pos| *pos & legal_positions == 0)
        .filter_map(|pos| Position::try_from(pos).ok())
        .any(|pos| game.is_legal_move(pos));

    assert!(!failed);
}

#[cfg(kani)]
#[kani::proof]
fn bits_should_be_consistent() {
    // Check that black and white stones do not overlap with each other or the
    // empty set
    let board = Board::try_from(kani::any::<ShadowBoard>()).unwrap();

    let black = board.bits_for(Stone::Black);
    let white = board.bits_for(Stone::White);
    let empty = board.empty_squares();

    assert!(black & white == 0);
    assert!((black | white) & empty == 0);
}

#[cfg(kani)]
#[kani::proof]
fn stone_at_consistency() {
    // Check that stone_at returns the correct stones
    let board = Board::try_from(kani::any::<ShadowBoard>()).unwrap();

    let black = board.bits_for(Stone::Black);
    let white = board.bits_for(Stone::White);
    let empty = board.empty_squares();

    let success = Bitboard::from(u64::MAX)
        .bits()
        .filter_map(|pos| Position::try_from(pos).ok())
        .all(|pos| {
            board
                .stone_at(pos)
                .map(|stone| match stone {
                    Stone::Black => pos & black != 0,
                    Stone::White => pos & white != 0,
                })
                .unwrap_or_else(|| pos & empty != 0)
        });

    assert!(success);
}
