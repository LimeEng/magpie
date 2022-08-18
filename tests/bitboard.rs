use magpie::othello::{Bitboard, SquareExt, StoneExt};
use quickcheck_macros::quickcheck;

#[quickcheck]
fn bitboards_handles_bitwise(num1: u64, num2: u64) {
    let board1 = Bitboard::from(num1);
    let board2 = Bitboard::from(num2);

    assert_eq!(num1 & num2, (board1 & board2).raw());
    assert_eq!(num1 | num2, (board1 | board2).raw());
    assert_eq!(num1 ^ num2, (board1 ^ board2).raw());

    let mut bitboard1_copy = num1;
    let mut board1_copy = board1;
    bitboard1_copy &= num2;
    board1_copy &= board2;
    assert_eq!(bitboard1_copy, board1_copy.raw());

    let mut bitboard1_copy = num1;
    let mut board1_copy = board1;
    bitboard1_copy |= num2;
    board1_copy |= board2;
    assert_eq!(bitboard1_copy, board1_copy.raw());

    let mut bitboard1_copy = num1;
    let mut board1_copy = board1;
    bitboard1_copy ^= num2;
    board1_copy ^= board2;
    assert_eq!(bitboard1_copy, board1_copy.raw());
}

#[quickcheck]
fn new_stones_and_squares_match(num: u64) {
    let board = Bitboard::from(num);
    let stones1: Vec<u64> = board.stones().map(|pos| pos.raw()).collect();
    let squares1: Vec<u64> = board.squares().map(|board| board.raw()).collect();
    let stones2: Vec<u64> = num.stones().collect();
    let squares2: Vec<u64> = num.squares().collect();

    assert_eq!(stones1, stones2);
    assert_eq!(squares1, squares2);
}
