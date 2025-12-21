use magpie::othello::Bitboard;
use quickcheck::{TestResult, quickcheck};

fn bb(val: u64) -> Bitboard {
    Bitboard::from(val)
}

macro_rules! quickcheck_shift_tests {
    ($($shift_type:ty),+) => {
        paste::paste! {
            quickcheck! {
                $(
                    fn [<bitboard_shl_ $shift_type>](input: u64, shift: $shift_type) -> TestResult {
                        if !(0..64).contains(&shift) {
                            return TestResult::discard();
                        }
                        let wrapped = bb(input);
                        TestResult::from_bool((wrapped << shift) == bb(input << shift))
                    }

                    fn [<bitboard_shr_ $shift_type>](input: u64, shift: $shift_type) -> TestResult {
                        if !(0..64).contains(&shift) {
                            return TestResult::discard();
                        }
                        let wrapped = bb(input);
                        TestResult::from_bool((wrapped >> shift) == bb(input >> shift))
                    }

                    fn [<bitboard_shl_assign_ $shift_type>](input: u64, shift: $shift_type) -> TestResult {
                        if !(0..64).contains(&shift) {
                            return TestResult::discard();
                        }
                        let mut x = bb(input);
                        x <<= shift;
                        TestResult::from_bool(x == bb(input << shift))
                    }

                    fn [<bitboard_shr_assign_ $shift_type>](input: u64, shift: $shift_type) -> TestResult {
                        if !(0..64).contains(&shift) {
                            return TestResult::discard();
                        }
                        let mut x = bb(input);
                        x >>= shift;
                        TestResult::from_bool(x == bb(input >> shift))
                    }
                )+
            }
        }
    };
}

quickcheck_shift_tests!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
