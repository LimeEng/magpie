use magpie::othello::Bitboard;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn bitboards_handles_bitwise(bitboard1: u64, bitboard2: u64) {
    let board1 = Bitboard::from(bitboard1);
    let board2 = Bitboard::from(bitboard2);

    assert_eq!(bitboard1 & bitboard2, (board1 & board2).raw());
    assert_eq!(bitboard1 | bitboard2, (board1 | board2).raw());
    assert_eq!(bitboard1 ^ bitboard2, (board1 ^ board2).raw());

    let mut bitboard1_copy = bitboard1;
    let mut board1_copy = board1;
    bitboard1_copy &= bitboard2;
    board1_copy &= board2;
    assert_eq!(bitboard1_copy, board1_copy.raw());

    let mut bitboard1_copy = bitboard1;
    let mut board1_copy = board1;
    bitboard1_copy |= bitboard2;
    board1_copy |= board2;
    assert_eq!(bitboard1_copy, board1_copy.raw());

    let mut bitboard1_copy = bitboard1;
    let mut board1_copy = board1;
    bitboard1_copy ^= bitboard2;
    board1_copy ^= board2;
    assert_eq!(bitboard1_copy, board1_copy.raw());
}
