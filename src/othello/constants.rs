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
    9223372036854775808, 4611686018427387904, 2305843009213693952, 1152921504606846976, 576460752303423488,
    288230376151711744, 144115188075855872, 72057594037927936, 36028797018963968, 18014398509481984,
    9007199254740992, 4503599627370496, 2251799813685248, 1125899906842624, 562949953421312, 281474976710656,
    140737488355328, 70368744177664, 35184372088832, 17592186044416, 8796093022208, 4398046511104, 2199023255552,
    1099511627776, 549755813888, 274877906944, 137438953472, 68719476736, 34359738368, 17179869184, 8589934592,
    4294967296, 2147483648, 1073741824, 536870912, 268435456, 134217728, 67108864, 33554432, 16777216, 8388608,
    4194304, 2097152, 1048576, 524288, 262144, 131072, 65536, 32768, 16384, 8192, 4096, 2048, 1024, 512, 256, 128,
    64, 32, 16, 8, 4, 2, 1,
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
// use magpie::othello::{SquareExt, FILE_A, FILE_H, SHIFT_DIRS};

// fn main() {
//     print_rays();
// }

// fn print_rays() {
//     let masks = vec![
//         u64::MAX,
//         !FILE_A,
//         !FILE_A,
//         !FILE_A,
//         u64::MAX,
//         !FILE_H,
//         !FILE_H,
//         !FILE_H,
//     ];
//     let mut result: Vec<Vec<u64>> = Vec::new();
//     for pos in u64::MAX.squares() {
//         let mut pos_result = Vec::new();
//         for (i, shift) in SHIFT_DIRS.iter().enumerate() {
//             let mut moves = 0;
//             let mut candidates = dir_shift(pos, *shift) & masks[i];
//             while candidates != 0 {
//                 moves |= candidates;
//                 candidates = dir_shift(candidates, *shift) & masks[i];
//             }
//             pos_result.push(moves);
//         }
//         result.push(pos_result)
//     }

//     let formatted_numbers: Vec<String> = result
//         .iter()
//         .map(|pos_rays| {
//             let hex_formatted: Vec<String> = pos_rays.iter().map(|ray| to_hex(*ray)).collect();
//             format!(
//                 "    [ {}, {}, {}, {},\n      {}, {}, {}, {} ]",
//                 hex_formatted.get(0).unwrap(),
//                 hex_formatted.get(1).unwrap(),
//                 hex_formatted.get(2).unwrap(),
//                 hex_formatted.get(3).unwrap(),
//                 hex_formatted.get(4).unwrap(),
//                 hex_formatted.get(5).unwrap(),
//                 hex_formatted.get(6).unwrap(),
//                 hex_formatted.get(7).unwrap()
//             )
//         })
//         .collect();

//     let output = format!(
//         "#[rustfmt::skip]\npub const SHIFT_RAYS: [[u64; 8]; 64] = [\n{}\n];",
//         formatted_numbers.join(",\n")
//     );

//     println!("{}", output);
// }

// fn to_hex(num: u64) -> String {
//     let mut hex = format!("{:#018x}", num);
//     for i in (4..23).step_by(3) {
//         hex.insert_str(i, "_");
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
