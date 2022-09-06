use magpie::othello::Bitboard;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn bitboard_raw_matches(num: u64) {
    assert_eq!(num, Bitboard::from(num).raw());
}

#[quickcheck]
fn bitboards_handles_bitwise(num1: u64, num2: u64) {
    let board1 = Bitboard::from(num1);
    let board2 = Bitboard::from(num2);

    assert_eq!(num1 & num2, (board1 & board2).raw());
    assert_eq!(num1 | num2, (board1 | board2).raw());
    assert_eq!(num1 ^ num2, (board1 ^ board2).raw());

    let mut num1_copy = num1;
    let mut board1_copy = board1;
    num1_copy &= num2;
    board1_copy &= board2;
    assert_eq!(num1_copy, board1_copy.raw());

    let mut num1_copy = num1;
    let mut board1_copy = board1;
    num1_copy |= num2;
    board1_copy |= board2;
    assert_eq!(num1_copy, board1_copy.raw());

    let mut num1_copy = num1;
    let mut board1_copy = board1;
    num1_copy ^= num2;
    board1_copy ^= board2;
    assert_eq!(num1_copy, board1_copy.raw());
}

// #[quickcheck]
// fn new_stones_and_squares_match(num: u64) {
//     let board = Bitboard::from(num);
//     let stones1: Vec<u64> = board.stones().map(Position::raw).collect();
//     let squares1: Vec<u64> = board.squares().map(Bitboard::raw).collect();
//     let stones2: Vec<u64> = num.stones().collect();
//     let squares2: Vec<u64> = num.squares().collect();

//     assert_eq!(stones1, stones2);
//     assert_eq!(squares1, squares2);
// }

#[test]
fn tmptmp() {
    // let mut p1 = Position(1);
    // let mut p2 = Position(2);
    // let mut b1 = Bitboard(4);
    // let mut b2 = Bitboard(8);

    // let t: Bitboard = p1 & b1;
    // let t: Bitboard = b1 & p1;
    // let t: Bitboard = p1 | b1;
    // let t: Bitboard = b1 | p1;
    // let t: Bitboard = p1 ^ b1;
    // let t: Bitboard = b1 ^ p1;
    // b1 |= p1;
    // b1 ^= p1;

    // Should not work
    // p1 |= b1;
    // p1 ^= b1;
    // let t: Position = p1 & p2;
    // let t: Position = p1 | p2; // Should not work
    // let t: Position = p1 ^ p2; // Should not work

    // let t: Position = p1 & b1;
    // let t: Position = b1 & p1;
    // let t: Position = p1 | b1;
    // let t: Position = b1 | p1;
    // let t: Position = p1 ^ b1;
    // let t: Position = b1 ^ p1;
}
