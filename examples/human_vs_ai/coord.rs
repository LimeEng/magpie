use std::convert::TryInto;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Coord {
    rank: u8,
    file: u8,
}

impl Coord {
    pub fn from_notation(text: &str) -> Result<Coord, ParseCoordError> {
        let mut graphemes = UnicodeSegmentation::graphemes(text, true).take(2);

        let file = graphemes.next();
        let rank = graphemes.next();

        let file = file
            .ok_or(ParseCoordError::InvalidSize)
            .and_then(|chr| match chr {
                "a" => Ok(0),
                "b" => Ok(1),
                "c" => Ok(2),
                "d" => Ok(3),
                "e" => Ok(4),
                "f" => Ok(5),
                "g" => Ok(6),
                "h" => Ok(7),
                _ => Err(ParseCoordError::InvalidFile),
            })?;

        let rank = rank
            .ok_or(ParseCoordError::InvalidSize)
            .and_then(|chr| match chr {
                "1" => Ok(0),
                "2" => Ok(1),
                "3" => Ok(2),
                "4" => Ok(3),
                "5" => Ok(4),
                "6" => Ok(5),
                "7" => Ok(6),
                "8" => Ok(7),
                _ => Err(ParseCoordError::InvalidRank),
            })?;
        Ok(Coord { rank, file })
    }

    pub fn from_cartesian(rank: u8, file: u8) -> Option<Coord> {
        if rank > 7 || file > 7 {
            return None;
        }
        Some(Coord { rank, file })
    }

    pub fn from_bitboard(bitboard: u64) -> Option<Coord> {
        // An Othello board is 8x8
        let file: u8 = (bitboard.leading_zeros() % 8).try_into().unwrap();
        let rank: u8 = (bitboard.leading_zeros() / 8).try_into().unwrap();

        Coord::from_cartesian(rank, file)
    }

    pub fn rank(&self) -> u8 {
        self.rank
    }

    pub fn file(&self) -> u8 {
        self.file
    }

    pub fn as_bitboard(&self) -> u64 {
        RANKS[self.rank as usize] & FILES[self.file as usize]
    }

    pub fn as_notation(&self) -> String {
        let file = match self.file {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => panic!(),
        };
        return format!("{}{}", file, self.rank + 1);
    }
}

#[derive(Debug)]
pub enum ParseCoordError {
    InvalidSize,
    InvalidRank,
    InvalidFile,
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
