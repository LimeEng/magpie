mod common;

use common::ShadowPosition;
use magpie::othello::{Bitboard, Position};
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;
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

#[quickcheck]
fn position_eq_reflexive(p: ShadowPosition) -> bool {
    let p = pos(p);
    p == p
}

#[quickcheck]
fn position_eq_symmetric(a: ShadowPosition, b: ShadowPosition) -> bool {
    let a = pos(a);
    let b = pos(b);
    (a == b) == (b == a)
}

#[quickcheck]
fn position_ord_reflexive(p: ShadowPosition) -> bool {
    let p = pos(p);

    p.cmp(&p) == Ordering::Equal
}

#[quickcheck]
fn position_ord_antisymmetric(a: ShadowPosition, b: ShadowPosition) -> bool {
    let a = pos(a);
    let b = pos(b);
    if a.raw() == b.raw() {
        return true;
    }
    (a < b) != (a > b)
}

#[quickcheck]
fn position_ord_transitive(a: ShadowPosition, b: ShadowPosition, c: ShadowPosition) -> TestResult {
    let a = pos(a);
    let b = pos(b);
    let c = pos(c);
    if !(a.raw() < b.raw() && b.raw() < c.raw()) {
        return TestResult::discard();
    }
    TestResult::from_bool(a < c)
}

// =====[ PartialEq tests ]=====

#[quickcheck]
fn position_partialeq_position(a: ShadowPosition, b: ShadowPosition) -> bool {
    let a = pos(a);
    let b = pos(b);
    (a == b) == (a.raw() == b.raw())
}

#[quickcheck]
#[allow(clippy::nonminimal_bool)]
fn position_partialeq_u64(p: ShadowPosition) -> bool {
    let p = pos(p);
    let val = p.raw();
    (p == val) && (val == p)
}

#[quickcheck]
fn position_bitboard_partialeq_cross(a: u64, p: ShadowPosition) -> bool {
    let p = pos(p);
    let b = p.raw();
    (bb(a) == p) == (a == b) && (p == bb(a)) == (b == a)
}

// =====[ PartialOrd tests ]=====

#[quickcheck]
fn position_partialord_position(a: ShadowPosition, b: ShadowPosition) -> bool {
    let a = pos(a);
    let b = pos(b);
    a.partial_cmp(&b) == Some(a.raw().cmp(&b.raw()))
}

#[quickcheck]
fn position_partialord_u64(p: ShadowPosition) -> bool {
    let p = pos(p);
    let val = p.raw();
    p.partial_cmp(&val) == Some(val.cmp(&val)) && val.partial_cmp(&p) == Some(val.cmp(&val))
}

#[quickcheck]
fn position_bitboard_partialord_cross(a: u64, p: ShadowPosition) -> bool {
    let p = pos(p);
    let b = p.raw();
    bb(a).partial_cmp(&p) == Some(a.cmp(&b)) && p.partial_cmp(&bb(a)) == Some(b.cmp(&a))
}

// =====[ Hash tests ]=====

#[quickcheck]
fn position_hash_equality(p: ShadowPosition) -> bool {
    let p = pos(p);
    hash_value(&p) == hash_value(&p)
}
