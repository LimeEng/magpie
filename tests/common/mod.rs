use lazy_static::lazy_static;
use std::iter::successors;

lazy_static! {
    static ref MASKS: Vec<u64> = {
        successors(Some(1_u64), |n| {
            if *n == 1_u64 << 63 {
                None
            } else {
                Some(n << 1)
            }
        })
        .collect()
    };
}

pub fn moves_to_list(moves: u64) -> impl Iterator<Item = u64> {
    MASKS.iter().map(move |m| m & moves).filter(|m| *m != 0)
}
