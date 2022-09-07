use crate::othello::{Bitboard, Position};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
};

macro_rules! newtype_transform {
    (impl $trait: ident <$other_type: ident> for $self_type: ident { fn $method: ident -> $return_type: ident }) => {
        impl $trait<$other_type> for $self_type {
            type Output = $return_type;

            fn $method(self, $other_type(b): $other_type) -> Self::Output {
                let $self_type(a) = self;
                $return_type(a.$method(&b))
            }
        }
    };
}

macro_rules! newtype_mutate {
    (impl $trait: ident <$other_type: ident> for $self_type: ident { fn $method: ident }) => {
        impl $trait<$other_type> for $self_type {
            fn $method(&mut self, $other_type(b): $other_type) {
                self.0.$method(&b)
            }
        }
    };
}

macro_rules! newtype_transform_with_number {
    (impl $trait: ident <$num_type: ident> for $self_type: ident { fn $method: ident }) => {
        impl $trait<$num_type> for $self_type {
            type Output = $self_type;

            fn $method(self, b: $num_type) -> Self::Output {
                let $self_type(a) = self;
                $self_type(a.$method(&b))
            }
        }
    };
}

macro_rules! newtype_mutate_with_number {
    (impl $trait: ident <$num_type: ident> for $self_type: ident { fn $method: ident }) => {
        impl $trait<$num_type> for $self_type {
            fn $method(&mut self, b: $num_type) {
                self.0.$method(&b)
            }
        }
    };
}

macro_rules! newtype_bitshift {
    ($($t:ident)*) => ($(
        newtype_transform_with_number! {impl Shl<$t> for Bitboard { fn shl }}
        newtype_transform_with_number! {impl Shr<$t> for Bitboard { fn shr }}

        newtype_mutate_with_number! {impl ShlAssign<$t> for Bitboard { fn shl_assign }}
        newtype_mutate_with_number! {impl ShrAssign<$t> for Bitboard { fn shr_assign }}

        newtype_transform_with_number! {impl Shl<$t> for Position { fn shl }}
        newtype_transform_with_number! {impl Shr<$t> for Position { fn shr }}

        newtype_mutate_with_number! {impl ShlAssign<$t> for Position { fn shl_assign }}
        newtype_mutate_with_number! {impl ShrAssign<$t> for Position { fn shr_assign }}
    )*)
}

macro_rules! number_transform_with_newtype {
    (impl $trait: ident <$new_type: ident> for $num_type: ident { fn $method: ident }) => {
        impl $trait<$new_type> for $num_type {
            type Output = $num_type;

            fn $method(self, $new_type(b): $new_type) -> Self::Output {
                self.$method(&b)
            }
        }
    };
}

macro_rules! number_mutate_with_newtype {
    (impl $trait: ident <$new_type: ident> for $num_type: ident { fn $method: ident }) => {
        impl $trait<$new_type> for $num_type {
            fn $method(&mut self, $new_type(b): $new_type) {
                self.$method(&b)
            }
        }
    };
}

macro_rules! number_operation_with_newtype {
    ($($t:ident)*) => ($(
        number_transform_with_newtype! {impl BitAnd<$t> for u64 { fn bitand }}
        number_transform_with_newtype! {impl BitOr<$t> for u64 { fn bitor }}
        number_transform_with_newtype! {impl BitXor<$t> for u64 { fn bitxor }}

        number_mutate_with_newtype! {impl BitAndAssign<$t> for u64 { fn bitand_assign }}
        number_mutate_with_newtype! {impl BitOrAssign<$t> for u64 { fn bitor_assign }}
        number_mutate_with_newtype! {impl BitXorAssign<$t> for u64 { fn bitxor_assign }}
    )*)
}

macro_rules! partial_for_newtype {
    (impl $trait: ident <$other_type: ident> for $self_type: ident { fn $method: ident -> $return_type: ty }) => {
        impl $trait<$other_type> for $self_type {
            fn $method(&self, b: &$other_type) -> $return_type {
                self.0.$method(&b.0)
            }
        }
    };
}

macro_rules! partial_for_newtype_with_number {
    (impl $trait: ident <$num_type: ident> for $self_type: ident { fn $method: ident -> $return_type: ty }) => {
        impl $trait<$num_type> for $self_type {
            fn $method(&self, b: &$num_type) -> $return_type {
                self.0.$method(b)
            }
        }
    };
}

macro_rules! partial_for_number_with_newtype {
    (impl $trait: ident <$other_type: ident> for $num_type: ident { fn $method: ident -> $return_type: ty }) => {
        impl $trait<$other_type> for $num_type {
            fn $method(&self, b: &$other_type) -> $return_type {
                self.$method(&b.0)
            }
        }
    };
}

macro_rules! newtype_ord_and_eq {
    ($($t:ident)*) => ($(
        impl Eq for $t {}

        impl Hash for $t {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl Ord for $t {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }

        partial_for_newtype!{impl PartialEq<$t> for $t {fn eq -> bool}}
        partial_for_newtype_with_number!{impl PartialEq<u64> for $t {fn eq -> bool}}
        partial_for_number_with_newtype!{impl PartialEq<$t> for u64 {fn eq -> bool}}

        partial_for_newtype!{impl PartialOrd<$t> for $t {fn partial_cmp -> Option<Ordering>}}
        partial_for_newtype_with_number!{impl PartialOrd<u64> for $t {fn partial_cmp -> Option<Ordering>}}
        partial_for_number_with_newtype!{impl PartialOrd<$t> for u64 {fn partial_cmp -> Option<Ordering>}}
    )*)
}

newtype_ord_and_eq! {Bitboard Position}
partial_for_newtype! {impl PartialEq<Position> for Bitboard {fn eq -> bool}}
partial_for_newtype! {impl PartialEq<Bitboard> for Position {fn eq -> bool}}
partial_for_newtype! {impl PartialOrd<Position> for Bitboard {fn partial_cmp -> Option<Ordering>}}
partial_for_newtype! {impl PartialOrd<Bitboard> for Position {fn partial_cmp -> Option<Ordering>}}

// Bitboard operations
newtype_transform! { impl BitAnd<Bitboard> for Bitboard { fn bitand -> Bitboard } }
newtype_transform! { impl BitOr<Bitboard> for Bitboard { fn bitor -> Bitboard } }
newtype_transform! { impl BitXor<Bitboard> for Bitboard { fn bitxor -> Bitboard } }

newtype_mutate! { impl BitAndAssign<Bitboard> for Bitboard { fn bitand_assign } }
newtype_mutate! { impl BitOrAssign<Bitboard> for Bitboard { fn bitor_assign } }
newtype_mutate! { impl BitXorAssign<Bitboard> for Bitboard { fn bitxor_assign } }

// Position operations
newtype_transform! { impl BitAnd<Position> for Position { fn bitand -> Position } }

// Bitboard and Position operations
newtype_transform! { impl BitAnd<Bitboard> for Position { fn bitand -> Bitboard } }
newtype_transform! { impl BitAnd<Position> for Bitboard { fn bitand -> Bitboard } }

newtype_transform! { impl BitOr<Bitboard> for Position { fn bitor -> Bitboard } }
newtype_transform! { impl BitOr<Position> for Bitboard { fn bitor -> Bitboard } }

newtype_transform! { impl BitXor<Bitboard> for Position { fn bitxor -> Bitboard } }
newtype_transform! { impl BitXor<Position> for Bitboard { fn bitxor -> Bitboard } }

newtype_mutate! { impl BitAndAssign<Bitboard> for Position { fn bitand_assign } }
newtype_mutate! { impl BitAndAssign<Position> for Bitboard { fn bitand_assign } }

newtype_mutate! { impl BitOrAssign<Position> for Bitboard { fn bitor_assign } }
newtype_mutate! { impl BitXorAssign<Position> for Bitboard { fn bitxor_assign } }

number_operation_with_newtype! {Bitboard Position}

// Bitboard and Position with numbers
newtype_bitshift! {u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize}

// Bitboard and u64
newtype_transform_with_number! {impl BitAnd<u64> for Bitboard { fn bitand }}
newtype_transform_with_number! {impl BitOr<u64> for Bitboard { fn bitor }}
newtype_transform_with_number! {impl BitXor<u64> for Bitboard { fn bitxor }}

newtype_mutate_with_number! {impl BitAndAssign<u64> for Bitboard { fn bitand_assign }}
newtype_mutate_with_number! {impl BitOrAssign<u64> for Bitboard { fn bitor_assign }}
newtype_mutate_with_number! {impl BitXorAssign<u64> for Bitboard { fn bitxor_assign }}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
