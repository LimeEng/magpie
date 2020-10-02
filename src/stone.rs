#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Stone {
    White,
    Black,
}

impl Stone {
    pub fn flip(&self) -> Stone {
        use Stone::*;
        match &self {
            White => Black,
            Black => White,
        }
    }
}
