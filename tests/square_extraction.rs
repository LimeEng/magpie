use magpie::othello::{SquareExt, StoneExt};

#[test]
fn full_bitboard_positions_equal_stones() {
    let v1: Vec<u64> = u64::MAX.squares().collect();
    let v2: Vec<u64> = u64::MAX.stones().collect();

    assert_eq!(v1, v2);
}
