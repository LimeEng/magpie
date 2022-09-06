#![allow(dead_code)]

// Starting positions of a standard Othello board
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

// Shifts for each direction, in the following order: N, NE, E, SE, S, SW, W, NW.
pub const SHIFT_DIRS: [i8; 8] = [-8, -7, 1, 9, 8, 7, -1, -9];

// Shift masks for each direction, in the following order: N, NE, E, SE, S, SW, W, NW.
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
pub const MASKS: [u64; 64] = [
    1 << 63, 1 << 62, 1 << 61, 1 << 60, 1 << 59, 1 << 58, 1 << 57, 1 << 56,
    1 << 55, 1 << 54, 1 << 53, 1 << 52, 1 << 51, 1 << 50, 1 << 49, 1 << 48,
    1 << 47, 1 << 46, 1 << 45, 1 << 44, 1 << 43, 1 << 42, 1 << 41, 1 << 40,
    1 << 39, 1 << 38, 1 << 37, 1 << 36, 1 << 35, 1 << 34, 1 << 33, 1 << 32,
    1 << 31, 1 << 30, 1 << 29, 1 << 28, 1 << 27, 1 << 26, 1 << 25, 1 << 24,
    1 << 23, 1 << 22, 1 << 21, 1 << 20, 1 << 19, 1 << 18, 1 << 17, 1 << 16,
    1 << 15, 1 << 14, 1 << 13, 1 << 12, 1 << 11, 1 << 10, 1 << 9 , 1 << 8 ,
    1 << 7 , 1 << 6 , 1 << 5 , 1 << 4 , 1 << 3 , 1 << 2 , 1 << 1 , 1 << 0 ,
];

// For each position on the board starting with A1, shift rays are presented in the following order: N, NE, E, SE, S, SW, W, NW.
// Each shift ray excludes the aformentioned position and extends to the end of the board.
#[rustfmt::skip]
pub const SHIFT_RAYS: [[u64; 8]; 64] = [
    [ 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x7f_00_00_00_00_00_00_00, 0x00_40_20_10_08_04_02_01,
      0x00_80_80_80_80_80_80_80, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x3f_00_00_00_00_00_00_00, 0x00_20_10_08_04_02_01_00,
      0x00_40_40_40_40_40_40_40, 0x00_80_00_00_00_00_00_00, 0x80_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x1f_00_00_00_00_00_00_00, 0x00_10_08_04_02_01_00_00,
      0x00_20_20_20_20_20_20_20, 0x00_40_80_00_00_00_00_00, 0xc0_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x0f_00_00_00_00_00_00_00, 0x00_08_04_02_01_00_00_00,
      0x00_10_10_10_10_10_10_10, 0x00_20_40_80_00_00_00_00, 0xe0_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x07_00_00_00_00_00_00_00, 0x00_04_02_01_00_00_00_00,
      0x00_08_08_08_08_08_08_08, 0x00_10_20_40_80_00_00_00, 0xf0_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x03_00_00_00_00_00_00_00, 0x00_02_01_00_00_00_00_00,
      0x00_04_04_04_04_04_04_04, 0x00_08_10_20_40_80_00_00, 0xf8_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x01_00_00_00_00_00_00_00, 0x00_01_00_00_00_00_00_00,
      0x00_02_02_02_02_02_02_02, 0x00_04_08_10_20_40_80_00, 0xfc_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00,
      0x00_01_01_01_01_01_01_01, 0x00_02_04_08_10_20_40_80, 0xfe_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x80_00_00_00_00_00_00_00, 0x40_00_00_00_00_00_00_00, 0x00_7f_00_00_00_00_00_00, 0x00_00_40_20_10_08_04_02,
      0x00_00_80_80_80_80_80_80, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x40_00_00_00_00_00_00_00, 0x20_00_00_00_00_00_00_00, 0x00_3f_00_00_00_00_00_00, 0x00_00_20_10_08_04_02_01,
      0x00_00_40_40_40_40_40_40, 0x00_00_80_00_00_00_00_00, 0x00_80_00_00_00_00_00_00, 0x80_00_00_00_00_00_00_00 ],
    [ 0x20_00_00_00_00_00_00_00, 0x10_00_00_00_00_00_00_00, 0x00_1f_00_00_00_00_00_00, 0x00_00_10_08_04_02_01_00,
      0x00_00_20_20_20_20_20_20, 0x00_00_40_80_00_00_00_00, 0x00_c0_00_00_00_00_00_00, 0x40_00_00_00_00_00_00_00 ],
    [ 0x10_00_00_00_00_00_00_00, 0x08_00_00_00_00_00_00_00, 0x00_0f_00_00_00_00_00_00, 0x00_00_08_04_02_01_00_00,
      0x00_00_10_10_10_10_10_10, 0x00_00_20_40_80_00_00_00, 0x00_e0_00_00_00_00_00_00, 0x20_00_00_00_00_00_00_00 ],
    [ 0x08_00_00_00_00_00_00_00, 0x04_00_00_00_00_00_00_00, 0x00_07_00_00_00_00_00_00, 0x00_00_04_02_01_00_00_00,
      0x00_00_08_08_08_08_08_08, 0x00_00_10_20_40_80_00_00, 0x00_f0_00_00_00_00_00_00, 0x10_00_00_00_00_00_00_00 ],
    [ 0x04_00_00_00_00_00_00_00, 0x02_00_00_00_00_00_00_00, 0x00_03_00_00_00_00_00_00, 0x00_00_02_01_00_00_00_00,
      0x00_00_04_04_04_04_04_04, 0x00_00_08_10_20_40_80_00, 0x00_f8_00_00_00_00_00_00, 0x08_00_00_00_00_00_00_00 ],
    [ 0x02_00_00_00_00_00_00_00, 0x01_00_00_00_00_00_00_00, 0x00_01_00_00_00_00_00_00, 0x00_00_01_00_00_00_00_00,
      0x00_00_02_02_02_02_02_02, 0x00_00_04_08_10_20_40_80, 0x00_fc_00_00_00_00_00_00, 0x04_00_00_00_00_00_00_00 ],
    [ 0x01_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00,
      0x00_00_01_01_01_01_01_01, 0x00_00_02_04_08_10_20_40, 0x00_fe_00_00_00_00_00_00, 0x02_00_00_00_00_00_00_00 ],
    [ 0x80_80_00_00_00_00_00_00, 0x20_40_00_00_00_00_00_00, 0x00_00_7f_00_00_00_00_00, 0x00_00_00_40_20_10_08_04,
      0x00_00_00_80_80_80_80_80, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x40_40_00_00_00_00_00_00, 0x10_20_00_00_00_00_00_00, 0x00_00_3f_00_00_00_00_00, 0x00_00_00_20_10_08_04_02,
      0x00_00_00_40_40_40_40_40, 0x00_00_00_80_00_00_00_00, 0x00_00_80_00_00_00_00_00, 0x00_80_00_00_00_00_00_00 ],
    [ 0x20_20_00_00_00_00_00_00, 0x08_10_00_00_00_00_00_00, 0x00_00_1f_00_00_00_00_00, 0x00_00_00_10_08_04_02_01,
      0x00_00_00_20_20_20_20_20, 0x00_00_00_40_80_00_00_00, 0x00_00_c0_00_00_00_00_00, 0x80_40_00_00_00_00_00_00 ],
    [ 0x10_10_00_00_00_00_00_00, 0x04_08_00_00_00_00_00_00, 0x00_00_0f_00_00_00_00_00, 0x00_00_00_08_04_02_01_00,
      0x00_00_00_10_10_10_10_10, 0x00_00_00_20_40_80_00_00, 0x00_00_e0_00_00_00_00_00, 0x40_20_00_00_00_00_00_00 ],
    [ 0x08_08_00_00_00_00_00_00, 0x02_04_00_00_00_00_00_00, 0x00_00_07_00_00_00_00_00, 0x00_00_00_04_02_01_00_00,
      0x00_00_00_08_08_08_08_08, 0x00_00_00_10_20_40_80_00, 0x00_00_f0_00_00_00_00_00, 0x20_10_00_00_00_00_00_00 ],
    [ 0x04_04_00_00_00_00_00_00, 0x01_02_00_00_00_00_00_00, 0x00_00_03_00_00_00_00_00, 0x00_00_00_02_01_00_00_00,
      0x00_00_00_04_04_04_04_04, 0x00_00_00_08_10_20_40_80, 0x00_00_f8_00_00_00_00_00, 0x10_08_00_00_00_00_00_00 ],
    [ 0x02_02_00_00_00_00_00_00, 0x00_01_00_00_00_00_00_00, 0x00_00_01_00_00_00_00_00, 0x00_00_00_01_00_00_00_00,
      0x00_00_00_02_02_02_02_02, 0x00_00_00_04_08_10_20_40, 0x00_00_fc_00_00_00_00_00, 0x08_04_00_00_00_00_00_00 ],
    [ 0x01_01_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_01_01_01_01_01, 0x00_00_00_02_04_08_10_20, 0x00_00_fe_00_00_00_00_00, 0x04_02_00_00_00_00_00_00 ],
    [ 0x80_80_80_00_00_00_00_00, 0x10_20_40_00_00_00_00_00, 0x00_00_00_7f_00_00_00_00, 0x00_00_00_00_40_20_10_08,
      0x00_00_00_00_80_80_80_80, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x40_40_40_00_00_00_00_00, 0x08_10_20_00_00_00_00_00, 0x00_00_00_3f_00_00_00_00, 0x00_00_00_00_20_10_08_04,
      0x00_00_00_00_40_40_40_40, 0x00_00_00_00_80_00_00_00, 0x00_00_00_80_00_00_00_00, 0x00_00_80_00_00_00_00_00 ],
    [ 0x20_20_20_00_00_00_00_00, 0x04_08_10_00_00_00_00_00, 0x00_00_00_1f_00_00_00_00, 0x00_00_00_00_10_08_04_02,
      0x00_00_00_00_20_20_20_20, 0x00_00_00_00_40_80_00_00, 0x00_00_00_c0_00_00_00_00, 0x00_80_40_00_00_00_00_00 ],
    [ 0x10_10_10_00_00_00_00_00, 0x02_04_08_00_00_00_00_00, 0x00_00_00_0f_00_00_00_00, 0x00_00_00_00_08_04_02_01,
      0x00_00_00_00_10_10_10_10, 0x00_00_00_00_20_40_80_00, 0x00_00_00_e0_00_00_00_00, 0x80_40_20_00_00_00_00_00 ],
    [ 0x08_08_08_00_00_00_00_00, 0x01_02_04_00_00_00_00_00, 0x00_00_00_07_00_00_00_00, 0x00_00_00_00_04_02_01_00,
      0x00_00_00_00_08_08_08_08, 0x00_00_00_00_10_20_40_80, 0x00_00_00_f0_00_00_00_00, 0x40_20_10_00_00_00_00_00 ],
    [ 0x04_04_04_00_00_00_00_00, 0x00_01_02_00_00_00_00_00, 0x00_00_00_03_00_00_00_00, 0x00_00_00_00_02_01_00_00,
      0x00_00_00_00_04_04_04_04, 0x00_00_00_00_08_10_20_40, 0x00_00_00_f8_00_00_00_00, 0x20_10_08_00_00_00_00_00 ],
    [ 0x02_02_02_00_00_00_00_00, 0x00_00_01_00_00_00_00_00, 0x00_00_00_01_00_00_00_00, 0x00_00_00_00_01_00_00_00,
      0x00_00_00_00_02_02_02_02, 0x00_00_00_00_04_08_10_20, 0x00_00_00_fc_00_00_00_00, 0x10_08_04_00_00_00_00_00 ],
    [ 0x01_01_01_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_01_01_01_01, 0x00_00_00_00_02_04_08_10, 0x00_00_00_fe_00_00_00_00, 0x08_04_02_00_00_00_00_00 ],
    [ 0x80_80_80_80_00_00_00_00, 0x08_10_20_40_00_00_00_00, 0x00_00_00_00_7f_00_00_00, 0x00_00_00_00_00_40_20_10,
      0x00_00_00_00_00_80_80_80, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x40_40_40_40_00_00_00_00, 0x04_08_10_20_00_00_00_00, 0x00_00_00_00_3f_00_00_00, 0x00_00_00_00_00_20_10_08,
      0x00_00_00_00_00_40_40_40, 0x00_00_00_00_00_80_00_00, 0x00_00_00_00_80_00_00_00, 0x00_00_00_80_00_00_00_00 ],
    [ 0x20_20_20_20_00_00_00_00, 0x02_04_08_10_00_00_00_00, 0x00_00_00_00_1f_00_00_00, 0x00_00_00_00_00_10_08_04,
      0x00_00_00_00_00_20_20_20, 0x00_00_00_00_00_40_80_00, 0x00_00_00_00_c0_00_00_00, 0x00_00_80_40_00_00_00_00 ],
    [ 0x10_10_10_10_00_00_00_00, 0x01_02_04_08_00_00_00_00, 0x00_00_00_00_0f_00_00_00, 0x00_00_00_00_00_08_04_02,
      0x00_00_00_00_00_10_10_10, 0x00_00_00_00_00_20_40_80, 0x00_00_00_00_e0_00_00_00, 0x00_80_40_20_00_00_00_00 ],
    [ 0x08_08_08_08_00_00_00_00, 0x00_01_02_04_00_00_00_00, 0x00_00_00_00_07_00_00_00, 0x00_00_00_00_00_04_02_01,
      0x00_00_00_00_00_08_08_08, 0x00_00_00_00_00_10_20_40, 0x00_00_00_00_f0_00_00_00, 0x80_40_20_10_00_00_00_00 ],
    [ 0x04_04_04_04_00_00_00_00, 0x00_00_01_02_00_00_00_00, 0x00_00_00_00_03_00_00_00, 0x00_00_00_00_00_02_01_00,
      0x00_00_00_00_00_04_04_04, 0x00_00_00_00_00_08_10_20, 0x00_00_00_00_f8_00_00_00, 0x40_20_10_08_00_00_00_00 ],
    [ 0x02_02_02_02_00_00_00_00, 0x00_00_00_01_00_00_00_00, 0x00_00_00_00_01_00_00_00, 0x00_00_00_00_00_01_00_00,
      0x00_00_00_00_00_02_02_02, 0x00_00_00_00_00_04_08_10, 0x00_00_00_00_fc_00_00_00, 0x20_10_08_04_00_00_00_00 ],
    [ 0x01_01_01_01_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_00_01_01_01, 0x00_00_00_00_00_02_04_08, 0x00_00_00_00_fe_00_00_00, 0x10_08_04_02_00_00_00_00 ],
    [ 0x80_80_80_80_80_00_00_00, 0x04_08_10_20_40_00_00_00, 0x00_00_00_00_00_7f_00_00, 0x00_00_00_00_00_00_40_20,
      0x00_00_00_00_00_00_80_80, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x40_40_40_40_40_00_00_00, 0x02_04_08_10_20_00_00_00, 0x00_00_00_00_00_3f_00_00, 0x00_00_00_00_00_00_20_10,
      0x00_00_00_00_00_00_40_40, 0x00_00_00_00_00_00_80_00, 0x00_00_00_00_00_80_00_00, 0x00_00_00_00_80_00_00_00 ],
    [ 0x20_20_20_20_20_00_00_00, 0x01_02_04_08_10_00_00_00, 0x00_00_00_00_00_1f_00_00, 0x00_00_00_00_00_00_10_08,
      0x00_00_00_00_00_00_20_20, 0x00_00_00_00_00_00_40_80, 0x00_00_00_00_00_c0_00_00, 0x00_00_00_80_40_00_00_00 ],
    [ 0x10_10_10_10_10_00_00_00, 0x00_01_02_04_08_00_00_00, 0x00_00_00_00_00_0f_00_00, 0x00_00_00_00_00_00_08_04,
      0x00_00_00_00_00_00_10_10, 0x00_00_00_00_00_00_20_40, 0x00_00_00_00_00_e0_00_00, 0x00_00_80_40_20_00_00_00 ],
    [ 0x08_08_08_08_08_00_00_00, 0x00_00_01_02_04_00_00_00, 0x00_00_00_00_00_07_00_00, 0x00_00_00_00_00_00_04_02,
      0x00_00_00_00_00_00_08_08, 0x00_00_00_00_00_00_10_20, 0x00_00_00_00_00_f0_00_00, 0x00_80_40_20_10_00_00_00 ],
    [ 0x04_04_04_04_04_00_00_00, 0x00_00_00_01_02_00_00_00, 0x00_00_00_00_00_03_00_00, 0x00_00_00_00_00_00_02_01,
      0x00_00_00_00_00_00_04_04, 0x00_00_00_00_00_00_08_10, 0x00_00_00_00_00_f8_00_00, 0x80_40_20_10_08_00_00_00 ],
    [ 0x02_02_02_02_02_00_00_00, 0x00_00_00_00_01_00_00_00, 0x00_00_00_00_00_01_00_00, 0x00_00_00_00_00_00_01_00,
      0x00_00_00_00_00_00_02_02, 0x00_00_00_00_00_00_04_08, 0x00_00_00_00_00_fc_00_00, 0x40_20_10_08_04_00_00_00 ],
    [ 0x01_01_01_01_01_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_00_00_01_01, 0x00_00_00_00_00_00_02_04, 0x00_00_00_00_00_fe_00_00, 0x20_10_08_04_02_00_00_00 ],
    [ 0x80_80_80_80_80_80_00_00, 0x02_04_08_10_20_40_00_00, 0x00_00_00_00_00_00_7f_00, 0x00_00_00_00_00_00_00_40,
      0x00_00_00_00_00_00_00_80, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x40_40_40_40_40_40_00_00, 0x01_02_04_08_10_20_00_00, 0x00_00_00_00_00_00_3f_00, 0x00_00_00_00_00_00_00_20,
      0x00_00_00_00_00_00_00_40, 0x00_00_00_00_00_00_00_80, 0x00_00_00_00_00_00_80_00, 0x00_00_00_00_00_80_00_00 ],
    [ 0x20_20_20_20_20_20_00_00, 0x00_01_02_04_08_10_00_00, 0x00_00_00_00_00_00_1f_00, 0x00_00_00_00_00_00_00_10,
      0x00_00_00_00_00_00_00_20, 0x00_00_00_00_00_00_00_40, 0x00_00_00_00_00_00_c0_00, 0x00_00_00_00_80_40_00_00 ],
    [ 0x10_10_10_10_10_10_00_00, 0x00_00_01_02_04_08_00_00, 0x00_00_00_00_00_00_0f_00, 0x00_00_00_00_00_00_00_08,
      0x00_00_00_00_00_00_00_10, 0x00_00_00_00_00_00_00_20, 0x00_00_00_00_00_00_e0_00, 0x00_00_00_80_40_20_00_00 ],
    [ 0x08_08_08_08_08_08_00_00, 0x00_00_00_01_02_04_00_00, 0x00_00_00_00_00_00_07_00, 0x00_00_00_00_00_00_00_04,
      0x00_00_00_00_00_00_00_08, 0x00_00_00_00_00_00_00_10, 0x00_00_00_00_00_00_f0_00, 0x00_00_80_40_20_10_00_00 ],
    [ 0x04_04_04_04_04_04_00_00, 0x00_00_00_00_01_02_00_00, 0x00_00_00_00_00_00_03_00, 0x00_00_00_00_00_00_00_02,
      0x00_00_00_00_00_00_00_04, 0x00_00_00_00_00_00_00_08, 0x00_00_00_00_00_00_f8_00, 0x00_80_40_20_10_08_00_00 ],
    [ 0x02_02_02_02_02_02_00_00, 0x00_00_00_00_00_01_00_00, 0x00_00_00_00_00_00_01_00, 0x00_00_00_00_00_00_00_01,
      0x00_00_00_00_00_00_00_02, 0x00_00_00_00_00_00_00_04, 0x00_00_00_00_00_00_fc_00, 0x80_40_20_10_08_04_00_00 ],
    [ 0x01_01_01_01_01_01_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_00_00_00_01, 0x00_00_00_00_00_00_00_02, 0x00_00_00_00_00_00_fe_00, 0x40_20_10_08_04_02_00_00 ],
    [ 0x80_80_80_80_80_80_80_00, 0x01_02_04_08_10_20_40_00, 0x00_00_00_00_00_00_00_7f, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00 ],
    [ 0x40_40_40_40_40_40_40_00, 0x00_01_02_04_08_10_20_00, 0x00_00_00_00_00_00_00_3f, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_80, 0x00_00_00_00_00_00_80_00 ],
    [ 0x20_20_20_20_20_20_20_00, 0x00_00_01_02_04_08_10_00, 0x00_00_00_00_00_00_00_1f, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_c0, 0x00_00_00_00_00_80_40_00 ],
    [ 0x10_10_10_10_10_10_10_00, 0x00_00_00_01_02_04_08_00, 0x00_00_00_00_00_00_00_0f, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_e0, 0x00_00_00_00_80_40_20_00 ],
    [ 0x08_08_08_08_08_08_08_00, 0x00_00_00_00_01_02_04_00, 0x00_00_00_00_00_00_00_07, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_f0, 0x00_00_00_80_40_20_10_00 ],
    [ 0x04_04_04_04_04_04_04_00, 0x00_00_00_00_00_01_02_00, 0x00_00_00_00_00_00_00_03, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_f8, 0x00_00_80_40_20_10_08_00 ],
    [ 0x02_02_02_02_02_02_02_00, 0x00_00_00_00_00_00_01_00, 0x00_00_00_00_00_00_00_01, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_fc, 0x00_80_40_20_10_08_04_00 ],
    [ 0x01_01_01_01_01_01_01_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00,
      0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_00, 0x00_00_00_00_00_00_00_fe, 0x80_40_20_10_08_04_02_00 ]
];

// Code to generate the shift rays below
// ============================================================================
// const FILE_A: u64 = 0x80_80_80_80_80_80_80_80;
// const FILE_H: u64 = 0x01_01_01_01_01_01_01_01;

// const SHIFT_DIRS: [i8; 8] = [-8, -7, 1, 9, 8, 7, -1, -9];
// const MASKS: [u64; 8] = [
//     u64::MAX,
//     !FILE_A,
//     !FILE_A,
//     !FILE_A,
//     u64::MAX,
//     !FILE_H,
//     !FILE_H,
//     !FILE_H,
// ];

// fn main() {
//     let formatted_numbers: Vec<String> = std::iter::successors(Some(1_u64 << 63), |next| {
//         if *next != 1 {
//             Some(next >> 1)
//         } else {
//             None
//         }
//     })
//     .map(formatted_shift_rays_for)
//     .collect();

//     let output = format!(
//         "#[rustfmt::skip]\npub const SHIFT_RAYS: [[u64; 8]; 64] = [\n{}\n];",
//         formatted_numbers.join(",\n")
//     );

//     println!("{}", output);
// }

// fn formatted_shift_rays_for(pos: u64) -> String {
//     let shift_rays = shift_rays_for(pos)
//         .into_iter()
//         .map(to_hex)
//         .collect::<Vec<String>>();
//     format!(
//         "    [ {}, {}, {}, {},\n      {}, {}, {}, {} ]",
//         shift_rays.get(0).unwrap(),
//         shift_rays.get(1).unwrap(),
//         shift_rays.get(2).unwrap(),
//         shift_rays.get(3).unwrap(),
//         shift_rays.get(4).unwrap(),
//         shift_rays.get(5).unwrap(),
//         shift_rays.get(6).unwrap(),
//         shift_rays.get(7).unwrap()
//     )
// }

// fn shift_rays_for(pos: u64) -> Vec<u64> {
//     SHIFT_DIRS
//         .iter()
//         .enumerate()
//         .map(|(i, shift)| {
//             let mut moves = 0;
//             let mut candidates = dir_shift(pos, *shift) & MASKS[i];
//             while candidates != 0 {
//                 moves |= candidates;
//                 candidates = dir_shift(candidates, *shift) & MASKS[i];
//             }
//             moves
//         })
//         .collect()
// }

// fn to_hex(num: u64) -> String {
//     let mut hex = format!("{:#018x}", num);
//     for i in (4..23).step_by(3) {
//         hex.insert(i, '_');
//     }
//     hex
// }

// fn dir_shift(x: u64, shift: i8) -> u64 {
//     if shift > 0 {
//         x >> shift
//     } else {
//         x << -shift
//     }
// }
// ============================================================================
