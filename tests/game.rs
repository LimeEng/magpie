use magpie::othello::{Bitboard, Game, Position};
use quickcheck_macros::quickcheck;

mod common;

use common::ShadowGame;

#[quickcheck]
fn legal_moves_should_place(game: ShadowGame) {
    // Check so that all legal moves returned can actually be placed
    let game = Game::try_from(game).unwrap();

    let result = game
        .moves()
        .hot_bits()
        .map(|pos| game.clone().play(pos))
        .all(|result| result.is_ok());
    assert!(result);
}

#[quickcheck]
fn illegal_moves_should_not_place(game: ShadowGame) {
    // Check so that all moves not contained in the set of legal moves
    // cannot actually be placed
    let game = Game::try_from(game).unwrap();

    let legal_positions = game.moves();

    let failed = Bitboard::from(u64::MAX)
        .bits()
        .filter(|pos| *pos & legal_positions == 0)
        .filter_map(|pos| Position::try_from(pos).ok())
        .map(|pos| game.clone().play(pos))
        .any(|result| result.is_ok());

    assert!(!failed);
}

#[quickcheck]
fn legal_moves_should_be_legal(game: ShadowGame) {
    // Check so that all legal moves returned can be individually verified as legal
    let game = Game::try_from(game).unwrap();

    let result = game.moves().hot_bits().all(|pos| game.is_legal_move(pos));
    assert!(result);
}

#[quickcheck]
fn illegal_moves_should_be_illegal(game: ShadowGame) {
    // Check so that all moves not contained in the set of legal moves
    // actually is illegal
    let game = Game::try_from(game).unwrap();

    let legal_positions = game.moves();

    let failed = Bitboard::from(u64::MAX)
        .bits()
        .filter(|pos| *pos & legal_positions == 0)
        .filter_map(|pos| Position::try_from(pos).ok())
        .any(|pos| game.is_legal_move(pos));

    assert!(!failed);
}
