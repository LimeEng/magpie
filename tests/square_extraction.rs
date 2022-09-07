use magpie::othello::{Bitboard, Position};

#[test]
fn full_bitboard_bits_equal_hot_bits() {
    let v1: Vec<Bitboard> = Bitboard::from(u64::MAX).bits().collect();
    let v2: Vec<Position> = Bitboard::from(u64::MAX).hot_bits().collect();

    assert_eq!(v1.len(), v2.len());

    let success = v1
        .iter()
        .zip(v2.iter())
        .all(|(e1, e2)| e1.raw() == e2.raw());
    assert!(success);
}
