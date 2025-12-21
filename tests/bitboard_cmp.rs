mod common;

use common::ShadowPosition;
use magpie::othello::{Bitboard, Position};
use quickcheck::{TestResult, quickcheck};
use std::{
    cmp::Ordering,
    hash::{DefaultHasher, Hash, Hasher},
};

fn bb(val: u64) -> Bitboard {
    Bitboard::from(val)
}

fn pos(shadow: ShadowPosition) -> Position {
    Position::try_from(shadow).unwrap()
}

fn hash_value<T: Hash>(val: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

// =====[ Eq and Ord tests ]=====

quickcheck! {
    fn bitboard_eq_reflexive(a: u64) -> bool {
        bb(a) == bb(a)
    }

    fn bitboard_eq_symmetric(a: u64, b: u64) -> bool {
        (bb(a) == bb(b)) == (bb(b) == bb(a))
    }

    fn bitboard_ord_reflexive(a: u64) -> bool {
        bb(a).cmp(&bb(a)) == Ordering::Equal
    }

    fn bitboard_ord_antisymmetric(a: u64, b: u64) -> bool {
        if a == b {
            return true;
        }
        (bb(a) < bb(b)) != (bb(a) > bb(b))
    }

    fn bitboard_ord_transitive(a: u64, b: u64, c: u64) -> TestResult {
        if !(a < b && b < c) {
            return TestResult::discard();
        }
        TestResult::from_bool(bb(a) < bb(c))
    }
}

// =====[ PartialEq tests ]=====

quickcheck! {
    fn bitboard_partialeq_bitboard(a: u64, b: u64) -> bool {
        (bb(a) == bb(b)) == (a == b)
    }

    fn bitboard_partialeq_u64(a: u64, b: u64) -> bool {
        (bb(a) == b) == (a == b) && (b == bb(a)) == (a == b)
    }

    fn bitboard_position_partialeq_cross(a: u64, p: ShadowPosition) -> bool {
        let p = pos(p);
        let b = p.raw();
        (bb(a) == p) == (a == b) && (p == bb(a)) == (b == a)
    }
}

// =====[ PartialOrd tests ]=====

quickcheck! {
    fn bitboard_partialord_bitboard(a: u64, b: u64) -> bool {
        bb(a).partial_cmp(&bb(b)) == Some(a.cmp(&b))
    }

    fn bitboard_partialord_u64(a: u64, b: u64) -> bool {
        bb(a).partial_cmp(&b) == Some(a.cmp(&b)) &&
        a.partial_cmp(&bb(b)) == Some(a.cmp(&b))
    }

    fn bitboard_position_partialord_cross(a: u64, p: ShadowPosition) -> bool {
        let p = pos(p);
        let b = p.raw();
        bb(a).partial_cmp(&p) == Some(a.cmp(&b)) &&
        p.partial_cmp(&bb(a)) == Some(b.cmp(&a))
    }
}

// =====[ Hash tests ]=====

quickcheck! {
    fn bitboard_hash_equality(a: u64) -> bool {
        hash_value(&bb(a)) == hash_value(&bb(a))
    }
}
