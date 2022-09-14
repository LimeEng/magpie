pub const BLACK_START_POS: u64 = 0x00_00_00_08_10_00_00_00;
pub const WHITE_START_POS: u64 = 0x00_00_00_10_08_00_00_00;

// Bitboards representing their named rank
pub const RANK_1: u64 = 0xff_00_00_00_00_00_00_00;
pub const RANK_2: u64 = 0x00_ff_00_00_00_00_00_00;
pub const RANK_3: u64 = 0x00_00_ff_00_00_00_00_00;
pub const RANK_4: u64 = 0x00_00_00_ff_00_00_00_00;
pub const RANK_5: u64 = 0x00_00_00_00_ff_00_00_00;
pub const RANK_6: u64 = 0x00_00_00_00_00_ff_00_00;
pub const RANK_7: u64 = 0x00_00_00_00_00_00_ff_00;
pub const RANK_8: u64 = 0x00_00_00_00_00_00_00_ff;
pub const RANKS: [u64; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];

// Bitboards representing their named file
pub const FILE_A: u64 = 0x80_80_80_80_80_80_80_80;
pub const FILE_B: u64 = 0x40_40_40_40_40_40_40_40;
pub const FILE_C: u64 = 0x20_20_20_20_20_20_20_20;
pub const FILE_D: u64 = 0x10_10_10_10_10_10_10_10;
pub const FILE_E: u64 = 0x08_08_08_08_08_08_08_08;
pub const FILE_F: u64 = 0x04_04_04_04_04_04_04_04;
pub const FILE_G: u64 = 0x02_02_02_02_02_02_02_02;
pub const FILE_H: u64 = 0x01_01_01_01_01_01_01_01;
pub const FILES: [u64; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];

// Shifts for each direction, in the following order:
// N, NE, E, SE, S, SW, W, NW.
pub const SHIFT_DIRS: [i8; 8] = [-8, -7, 1, 9, 8, 7, -1, -9];

// Shift masks for each direction, in the following order:
// N, NE, E, SE, S, SW, W, NW.
pub const SHIFT_MASKS: [u64; 8] = [
    !RANK_1,
    !(RANK_1 | FILE_H),
    !FILE_H,
    !(RANK_8 | FILE_H),
    !RANK_8,
    !(RANK_8 | FILE_A),
    !FILE_A,
    !(RANK_1 | FILE_A),
];

// Masks representing each position on the board starting with A1.
#[rustfmt::skip]
pub const POSITIONS: [u64; 64] = [
    1 << 63, 1 << 62, 1 << 61, 1 << 60, 1 << 59, 1 << 58, 1 << 57, 1 << 56,
    1 << 55, 1 << 54, 1 << 53, 1 << 52, 1 << 51, 1 << 50, 1 << 49, 1 << 48,
    1 << 47, 1 << 46, 1 << 45, 1 << 44, 1 << 43, 1 << 42, 1 << 41, 1 << 40,
    1 << 39, 1 << 38, 1 << 37, 1 << 36, 1 << 35, 1 << 34, 1 << 33, 1 << 32,
    1 << 31, 1 << 30, 1 << 29, 1 << 28, 1 << 27, 1 << 26, 1 << 25, 1 << 24,
    1 << 23, 1 << 22, 1 << 21, 1 << 20, 1 << 19, 1 << 18, 1 << 17, 1 << 16,
    1 << 15, 1 << 14, 1 << 13, 1 << 12, 1 << 11, 1 << 10, 1 << 9 , 1 << 8 ,
    1 << 7 , 1 << 6 , 1 << 5 , 1 << 4 , 1 << 3 , 1 << 2 , 1 << 1 , 1 << 0 ,
];

#[rustfmt::skip]
pub const POSITIONS_AS_NOTATION: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];
