use magpie::othello::Bitboard;
use quickcheck::quickcheck;

fn bb(val: u64) -> Bitboard {
    Bitboard::from(val)
}

// =====[ Bitboard BitAnd tests ]=====

quickcheck! {
    fn bitboard_bitand_bitboard(a: u64, b: u64) -> bool {
        (bb(a) & bb(b)) == bb(a & b)
    }

    fn bitboard_bitand_commutative(a: u64, b: u64) -> bool {
        (bb(a) & bb(b)) == (bb(b) & bb(a))
    }

    fn bitboard_bitand_associative(a: u64, b: u64, c: u64) -> bool {
        ((bb(a) & bb(b)) & bb(c)) == (bb(a) & (bb(b) & bb(c)))
    }

    fn bitboard_bitand_identity(a: u64) -> bool {
        (bb(a) & bb(!0)) == bb(a)
    }

    fn bitboard_bitand_annihilator(a: u64) -> bool {
        (bb(a) & bb(0)) == bb(0)
    }

    fn bitboard_bitand_u64(a: u64, b: u64) -> bool {
        (bb(a) & b) == bb(a & b)
    }

    fn u64_bitand_bitboard(a: u64, b: u64) -> bool {
        (a & bb(b)) == (a & b)
    }

    fn bitboard_bitand_assign_bitboard(a: u64, b: u64) -> bool {
        let mut x = bb(a);
        x &= bb(b);
        x == bb(a & b)
    }

    fn bitboard_bitand_assign_u64(a: u64, b: u64) -> bool {
        let mut x = bb(a);
        x &= b;
        x == bb(a & b)
    }

    fn u64_bitand_assign_bitboard(a: u64, b: u64) -> bool {
        let mut x = a;
        x &= bb(b);
        x == (a & b)
    }
}

// =====[ Bitboard BitOr tests ]=====

quickcheck! {
    fn bitboard_bitor_bitboard(a: u64, b: u64) -> bool {
        (bb(a) | bb(b)) == bb(a | b)
    }

    fn bitboard_bitor_commutative(a: u64, b: u64) -> bool {
        (bb(a) | bb(b)) == (bb(b) | bb(a))
    }

    fn bitboard_bitor_associative(a: u64, b: u64, c: u64) -> bool {
        ((bb(a) | bb(b)) | bb(c)) == (bb(a) | (bb(b) | bb(c)))
    }

    fn bitboard_bitor_identity(a: u64) -> bool {
        (bb(a) | bb(0)) == bb(a)
    }

    fn bitboard_bitor_annihilator(a: u64) -> bool {
        (bb(a) | bb(!0)) == bb(!0)
    }

    fn bitboard_bitor_u64(a: u64, b: u64) -> bool {
        (bb(a) | b) == bb(a | b)
    }

    fn u64_bitor_bitboard(a: u64, b: u64) -> bool {
        (a | bb(b)) == (a | b)
    }

    fn bitboard_bitor_assign_bitboard(a: u64, b: u64) -> bool {
        let mut x = bb(a);
        x |= bb(b);
        x == bb(a | b)
    }

    fn bitboard_bitor_assign_u64(a: u64, b: u64) -> bool {
        let mut x = bb(a);
        x |= b;
        x == bb(a | b)
    }

    fn u64_bitor_assign_bitboard(a: u64, b: u64) -> bool {
        let mut x = a;
        x |= bb(b);
        x == (a | b)
    }
}

// =====[ Bitboard BitXor tests ]=====

quickcheck! {
    fn bitboard_bitxor_bitboard(a: u64, b: u64) -> bool {
        (bb(a) ^ bb(b)) == bb(a ^ b)
    }

    fn bitboard_bitxor_commutative(a: u64, b: u64) -> bool {
        (bb(a) ^ bb(b)) == (bb(b) ^ bb(a))
    }

    fn bitboard_bitxor_associative(a: u64, b: u64, c: u64) -> bool {
        ((bb(a) ^ bb(b)) ^ bb(c)) == (bb(a) ^ (bb(b) ^ bb(c)))
    }

    fn bitboard_bitxor_identity(a: u64) -> bool {
        (bb(a) ^ bb(0)) == bb(a)
    }

    fn bitboard_bitxor_self_annihilates(a: u64) -> bool {
        (bb(a) ^ bb(a)) == bb(0)
    }

    fn bitboard_bitxor_u64(a: u64, b: u64) -> bool {
        (bb(a) ^ b) == bb(a ^ b)
    }

    fn u64_bitxor_bitboard(a: u64, b: u64) -> bool {
        (a ^ bb(b)) == (a ^ b)
    }

    fn bitboard_bitxor_assign_bitboard(a: u64, b: u64) -> bool {
        let mut x = bb(a);
        x ^= bb(b);
        x == bb(a ^ b)
    }

    fn bitboard_bitxor_assign_u64(a: u64, b: u64) -> bool {
        let mut x = bb(a);
        x ^= b;
        x == bb(a ^ b)
    }

    fn u64_bitxor_assign_bitboard(a: u64, b: u64) -> bool {
        let mut x = a;
        x ^= bb(b);
        x == (a ^ b)
    }
}

// =====[ Bitboard Not test ]=====

quickcheck! {
    fn bitboard_not(a: u64) -> bool {
        !bb(a) == bb(!a)
    }

    fn bitboard_not_involution(a: u64) -> bool {
        !!bb(a) == bb(a)
    }
}
