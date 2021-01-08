use std::iter::Take;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn cardinals() -> Take<Self> {
        Direction::N.take(8)
    }
}

impl Iterator for Direction {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;
        let next = match self {
            N => NE,
            NE => E,
            E => SE,
            SE => S,
            S => SW,
            SW => W,
            W => NW,
            NW => N,
        };
        Some(std::mem::replace(self, next))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_iter_size() {
        use Direction::*;
        let correct_order = vec![N, NE, E, SE, S, SW, W, NW];
        let count = Direction::cardinals().take(100).count();

        assert_eq!(correct_order.len(), count);
    }

    #[test]
    fn direction_iter_order() {
        use Direction::*;
        let correct_order = vec![N, NE, E, SE, S, SW, W, NW];
        let equal_count = Direction::cardinals()
            .zip(correct_order.iter())
            .filter(|(a, b)| a == *b)
            .count();

        assert_eq!(correct_order.len(), equal_count);
    }
}
