use magpie::othello::{Bitboard, Board, Game, OthelloError, Position, PositionError, Stone};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct ShadowBitboard(u64);

impl From<ShadowBitboard> for Bitboard {
    fn from(bitboard: ShadowBitboard) -> Self {
        Bitboard::from(bitboard.0)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct ShadowPosition(u64);

impl TryFrom<ShadowPosition> for Position {
    type Error = PositionError;

    fn try_from(pos: ShadowPosition) -> Result<Self, Self::Error> {
        Position::try_from(pos.0)
    }
}

#[derive(Debug, Clone)]
pub struct ShadowBoard {
    black_stones: u64,
    white_stones: u64,
}

#[cfg(kani)]
impl kani::Arbitrary for ShadowBoard {
    fn any() -> Self {
        // Generate a random bitboard
        let bits = kani::any::<u64>();

        let mut black_stones = 0;
        let mut white_stones = 0;

        // Iterate over all bits
        for i in 0..63 {
            // Extract the next bit
            let next_bit = (bits >> i) & 1;
            if kani::any::<bool>() {
                black_stones |= next_bit << i;
            } else {
                white_stones |= next_bit << i;
            }
        }
        ShadowBoard::try_from((black_stones, white_stones)).unwrap()
    }
}

impl TryFrom<(u64, u64)> for ShadowBoard {
    type Error = OthelloError;

    fn try_from(stones: (u64, u64)) -> Result<Self, Self::Error> {
        let (black_stones, white_stones) = stones;
        if black_stones & white_stones != 0 {
            return Err(OthelloError::PiecesOverlapping);
        }
        let board = ShadowBoard {
            black_stones,
            white_stones,
        };
        Ok(board)
    }
}

impl TryFrom<ShadowBoard> for Board {
    type Error = OthelloError;

    fn try_from(board: ShadowBoard) -> Result<Self, Self::Error> {
        Board::try_from((board.black_stones, board.white_stones))
    }
}

#[derive(Debug, Clone)]
pub struct ShadowGame {
    board: Board,
    next_player: Stone,
    passed_last_turn: bool,
}

#[cfg(kani)]
impl kani::Arbitrary for ShadowGame {
    fn any() -> Self {
        let board = ShadowBoard::any();
        let player_black = kani::any::<bool>();
        let passed_last_turn = kani::any::<bool>();

        let board = Board::try_from(board).unwrap();
        let next_player = if player_black {
            Stone::Black
        } else {
            Stone::White
        };

        ShadowGame {
            board,
            next_player,
            passed_last_turn,
        }
    }
}

impl TryFrom<ShadowGame> for Game {
    type Error = OthelloError;

    fn try_from(game: ShadowGame) -> Result<Self, Self::Error> {
        Game::from_state(game.board, game.next_player, game.passed_last_turn)
    }
}
