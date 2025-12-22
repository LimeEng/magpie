mod common;

use common::ShadowPosition;
use magpie::othello::{Bitboard, Position};
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;

fn bb(val: u64) -> Bitboard {
    Bitboard::from(val)
}

fn pos(shadow: ShadowPosition) -> Position {
    Position::try_from(shadow).unwrap()
}

// =====[ Position shift tests ]=====

macro_rules! quickcheck_position_shift_tests {
    ($($shift_type:ty),+) => {
        paste::paste! {
            $(
                #[quickcheck]
                fn [<position_shl_ $shift_type>](p: ShadowPosition, shift: $shift_type) -> TestResult {
                    if !(0..64).contains(&shift) {
                        return TestResult::discard();
                    }
                    let p = pos(p);
                    let result: Bitboard = p << shift;
                    TestResult::from_bool(result == bb(p.raw() << shift))
                }

                #[quickcheck]
                fn [<position_shr_ $shift_type>](p: ShadowPosition, shift: $shift_type) -> TestResult {
                    if !(0..64).contains(&shift) {
                        return TestResult::discard();
                    }
                    let p = pos(p);
                    let result: Bitboard = p >> shift;
                    TestResult::from_bool(result == bb(p.raw() >> shift))
                }
            )+
        }
    };
}

quickcheck_position_shift_tests!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
