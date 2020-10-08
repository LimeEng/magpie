#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    pub fn flip(&self) -> Stone {
        use Stone::*;
        match &self {
            Black => White,
            White => Black,
        }
    }
}
