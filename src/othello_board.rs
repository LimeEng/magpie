use crate::direction::Direction;
use crate::stone::Stone;

#[derive(Clone)]
pub struct OthelloBoard {
    white_stones: u64,
    black_stones: u64,
}

impl OthelloBoard {
    pub fn new() -> OthelloBoard {
        OthelloBoard {
            white_stones: WHITE_START_POS,
            black_stones: BLACK_START_POS,
        }
    }

    pub fn place_stone_unchecked(&mut self, stone: Stone, pos: u64) {
        match stone {
            Stone::White => self.white_stones |= pos,
            Stone::Black => self.black_stones |= pos,
        }
    }

    pub fn place_stone(&mut self, stone: Stone, pos: u64) -> Result<(), OthelloError> {
        if pos.count_ones() != 1 {
            return Err(OthelloError::IllegalMove);
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
            Stone::White => {
                self.white_stones |= mask;
                self.white_stones |= pos;
                self.black_stones ^= mask;
            }
            Stone::Black => {
                self.black_stones |= mask;
                self.black_stones |= pos;
                self.white_stones ^= mask;
            }
        }
        Ok(())
    }

    pub fn bits_for(&self, stone: Stone) -> u64 {
        match stone {
            Stone::White => self.white_stones,
            Stone::Black => self.black_stones,
        }
    }

    fn is_legal_move(&self, stone: Stone, pos: u64) -> bool {
        (pos & self.legal_moves_for(stone)) != 0
    }

    // https://core.ac.uk/download/pdf/33500946.pdf
    pub fn legal_moves_for(&self, stone: Stone) -> u64 {
        let free_places = self.free_spaces();
        let current_bits = self.bits_for(stone);
        let opponent_bits = self.bits_for(stone.flip());

        let mut moves = 0;
        for dir in Direction::cardinals() {
            let mut candidates = dir.shift(current_bits) & opponent_bits;
            while candidates != 0 {
                moves |= free_places & dir.shift(candidates);
                candidates = dir.shift(candidates) & opponent_bits;
            }
        }
        moves
    }

    pub fn free_spaces(&self) -> u64 {
        (!self.white_stones) & (!self.black_stones)
    }

    pub fn stone_at(&self, pos: u64) -> Option<Stone> {
        if self.white_stones & pos > 0 {
            Some(Stone::White)
        } else if self.black_stones & pos > 0 {
            Some(Stone::Black)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum OthelloError {
    IllegalMove,
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

const WHITE_START_POS: u64 = 0x00_00_00_10_08_00_00_00;
const BLACK_START_POS: u64 = 0x00_00_00_08_10_00_00_00;

const LEFT_MASK: u64 = 0x80_80_80_80_80_80_80_80;
const RIGHT_MASK: u64 = 0x01_01_01_01_01_01_01_01;
