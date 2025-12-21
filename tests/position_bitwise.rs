mod common;

use common::ShadowPosition;
use magpie::othello::{Bitboard, Position};
use quickcheck::quickcheck;

fn bb(val: u64) -> Bitboard {
    Bitboard::from(val)
}

fn pos(shadow: ShadowPosition) -> Position {
    Position::try_from(shadow).unwrap()
}

// =====[ Position BitAnd tests ]=====

quickcheck! {
    fn position_bitand_position(a: ShadowPosition, b: ShadowPosition) -> bool {
        let a = pos(a);
        let b = pos(b);
        let result: Bitboard = a & b;
        result == bb(a.raw() & b.raw())
    }

    fn position_bitand_bitboard(a: ShadowPosition, b: u64) -> bool {
        let a = pos(a);
        let result: Bitboard = a & bb(b);
        result == bb(a.raw() & b)
    }

    fn bitboard_bitand_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        (bb(a) & b) == bb(a & b.raw())
    }

    fn bitboard_bitand_assign_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        let mut x = bb(a);
        x &= b;
        x == bb(a & b.raw())
    }

    fn u64_bitand_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        (a & b) == (a & b.raw())
    }

    fn u64_bitand_assign_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        let mut x = a;
        x &= b;
        x == (a & b.raw())
    }
}

// =====[ Position BitOr tests ]=====

quickcheck! {
    fn position_bitor_position(a: ShadowPosition, b: ShadowPosition) -> bool {
        let a = pos(a);
        let b = pos(b);
        let result: Bitboard = a | b;
        result == bb(a.raw() | b.raw())
    }

    fn position_bitor_bitboard(a: ShadowPosition, b: u64) -> bool {
        let a = pos(a);
        let result: Bitboard = a | bb(b);
        result == bb(a.raw() | b)
    }

    fn bitboard_bitor_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        (bb(a) | b) == bb(a | b.raw())
    }

    fn bitboard_bitor_assign_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        let mut x = bb(a);
        x |= b;
        x == bb(a | b.raw())
    }

    fn u64_bitor_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        (a | b) == (a | b.raw())
    }

    fn u64_bitor_assign_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        let mut x = a;
        x |= b;
        x == (a | b.raw())
    }
}

// =====[ Position BitXor tests ]=====

quickcheck! {
    fn position_bitxor_position(a: ShadowPosition, b: ShadowPosition) -> bool {
        let a = pos(a);
        let b = pos(b);
        let result: Bitboard = a ^ b;
        result == bb(a.raw() ^ b.raw())
    }

    fn position_bitxor_bitboard(a: ShadowPosition, b: u64) -> bool {
        let a = pos(a);
        let result: Bitboard = a ^ bb(b);
        result == bb(a.raw() ^ b)
    }

    fn bitboard_bitxor_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        (bb(a) ^ b) == bb(a ^ b.raw())
    }

    fn bitboard_bitxor_assign_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        let mut x = bb(a);
        x ^= b;
        x == bb(a ^ b.raw())
    }

    fn u64_bitxor_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        (a ^ b) == (a ^ b.raw())
    }

    fn u64_bitxor_assign_position(a: u64, b: ShadowPosition) -> bool {
        let b = pos(b);
        let mut x = a;
        x ^= b;
        x == (a ^ b.raw())
    }
}
