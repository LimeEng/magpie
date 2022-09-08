use crate::gen_constants::common::{FILE_A, FILE_H, POSITIONS, SHIFT_DIRS};

pub fn generate() -> String {
    let masks = vec![
        u64::MAX,
        !FILE_A,
        !FILE_A,
        !FILE_A,
        u64::MAX,
        !FILE_H,
        !FILE_H,
        !FILE_H,
    ];
    let mut result: Vec<Vec<u64>> = Vec::new();
    for pos in u64::MAX.bits() {
        let mut pos_result = Vec::new();
        for (i, shift) in SHIFT_DIRS.iter().enumerate() {
            let mut moves = 0;
            let mut candidates = dir_shift(pos, *shift) & masks[i];
            while candidates != 0 {
                moves |= candidates;
                candidates = dir_shift(candidates, *shift) & masks[i];
            }
            pos_result.push(moves);
        }
        result.push(pos_result);
    }

    let formatted_numbers: Vec<String> = result
        .iter()
        .map(|pos_rays| {
            let hex_formatted: Vec<String> = pos_rays.iter().map(|ray| to_hex(*ray)).collect();
            format!(
                "    [ {}, {}, {}, {},\n      {}, {}, {}, {} ]",
                hex_formatted.get(0).unwrap(),
                hex_formatted.get(1).unwrap(),
                hex_formatted.get(2).unwrap(),
                hex_formatted.get(3).unwrap(),
                hex_formatted.get(4).unwrap(),
                hex_formatted.get(5).unwrap(),
                hex_formatted.get(6).unwrap(),
                hex_formatted.get(7).unwrap()
            )
        })
        .collect();

    let comment_lines = vec![
        "For each position on the board starting with A1, shift rays are presented in",
        "the following order: N, NE, E, SE, S, SW, W, NW.",
        "Each shift ray excludes the aformentioned position and extends to the end of the board.",
    ];

    let comment = comment_lines
        .iter()
        .map(|line| format!("// {}", line))
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "{}\n#[rustfmt::skip]\npub const SHIFT_RAYS: [[u64; 8]; 64] = [\n{}\n];",
        comment,
        formatted_numbers.join(",\n")
    )
}

fn to_hex(num: u64) -> String {
    let mut hex = format!("{:#018x}", num);
    for i in (4..23).step_by(3) {
        hex.insert(i, '_');
    }
    hex
}

fn dir_shift(x: u64, shift: i8) -> u64 {
    if shift > 0 {
        x >> shift
    } else {
        x << -shift
    }
}

pub trait HotBits: Sized {
    type Iter: Iterator<Item = Self>;
    fn hot_bits(&self) -> Self::Iter;
}

impl HotBits for u64 {
    type Iter = Box<dyn Iterator<Item = u64>>;
    fn hot_bits(&self) -> Self::Iter {
        let this = *self;
        let iter = POSITIONS.iter().map(move |m| m & this).filter(|m| *m != 0);

        Box::new(iter)
    }
}

pub trait Bits: Sized {
    type Iter: Iterator<Item = Self>;
    fn bits(&self) -> Self::Iter;
}

impl Bits for u64 {
    type Iter = Box<dyn Iterator<Item = u64>>;
    fn bits(&self) -> Self::Iter {
        let this = *self;
        let iter = POSITIONS.iter().map(move |m| m & this);

        Box::new(iter)
    }
}
