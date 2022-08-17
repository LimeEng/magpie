#[derive(Clone, Copy, Debug, Default)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn raw(self) -> u64 {
        self.0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn count_set(self) -> u8 {
        self.0.count_ones().try_into().unwrap()
    }

    pub fn count_empty(self) -> u8 {
        self.0.count_zeros().try_into().unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bits {
    remaining: usize,
    bitboard: Bitboard,
}

impl Iterator for Bits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let mask = 1u64 << (self.remaining - 1);
        let bit = !(self.bitboard & mask);
        self.remaining -= 1;
        Some(bit.raw())
    }
}

impl ExactSizeIterator for Bits {
    fn len(&self) -> usize {
        self.remaining
    }
}

/// Iterate over the bits in row-major order.
impl IntoIterator for Bitboard {
    type Item = u64;
    type IntoIter = Bits;

    fn into_iter(self) -> Self::IntoIter {
        Bits {
            // TODO: Maybe just replace this with a constant?
            // Feels silly to lookup the size of u64
            remaining: std::mem::size_of::<u64>() * 8,
            bitboard: self,
        }
    }
}

impl From<u64> for Bitboard {
    fn from(bitboard: u64) -> Self {
        Bitboard(bitboard)
    }
}

impl From<Bitboard> for u64 {
    fn from(bitboard: Bitboard) -> Self {
        bitboard.0
    }
}
