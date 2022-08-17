use magpie::othello::Bitboard;
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
