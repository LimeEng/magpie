use magpie::othello::{Bitboard, Position};

#[test]
fn full_bitboard_bits_equal_hot_bits() {
    let v1 = Bitboard::from(u64::MAX).bits();
    let v2 = Bitboard::from(u64::MAX).hot_bits();

    let v1: Vec<u64> = v1.map(Bitboard::raw).collect();
    let v2: Vec<u64> = v2.map(Position::raw).collect();

    assert_eq!(v1, v2);
}
