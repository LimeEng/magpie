use magpie::othello::{OthelloBoard, PositionExt, Stone};
use std::convert::TryFrom;
fn main() {
    // The board can be initialized in a few different ways.

    // ========================================================================
    // This board is completely empty. It is not expected to be heavily used.
    // However, this constructor combined with the method
    // `place_stone_unchecked` can be leveraged to implement a game of Reversi,
    // which is distinctly different from Othello. Unfortunately, some
    // additional bookkeeping is required in this case since magpie does not
    // yet support Reversi.
    let _board = OthelloBoard::empty();
    // ========================================================================
    // These two constructors are equivalent. These boards are setup in the
    // default Othello opening, as can be seen in this graphic, where "B" and
    // "W" stands for black and white, respectively:
    //     ABCDEFGH
    //     +--------+
    //   1 |........|
    //   2 |........|
    //   3 |........|
    //   4 |...WB...|
    //   5 |...BW...|
    //   6 |........|
    //   7 |........|
    //   8 |........|
    //     +--------+
    let board_standard = OthelloBoard::standard();
    let board_default = OthelloBoard::default();
    assert_eq!(board_standard, board_default);
    // To better visualize what is happening, this debug function is included
    // in this example. This is what the standard Othello opening looks like.
    println!("Standard Othello opening");
    debug_board(&board_standard);
    // ========================================================================
    // It is also possible to construct a new board with the TryFrom trait.
    // It makes it possible to set the board to a custom state. The only check
    // that is performed is whether or not any black and white stones overlap.
    // Other than that, any configuration is allowed, even those that are
    // impossible to reach during normal play.

    // This particular example will fail since at least one stone of each
    // player overlap.
    let black_stones = 1; // In binary: ...0001
    let white_stones = 1; // In binary: ...0001
    assert!(OthelloBoard::try_from((black_stones, white_stones)).is_err());

    // However, this example will succeed since the stones do not overlap, even
    // if this particular example is unreachable during normal play.
    let black_stones = 1; // In binary: ...0001
    let white_stones = 4; // In binary: ...0100
    assert!(OthelloBoard::try_from((black_stones, white_stones)).is_ok());
    // ========================================================================
    // The legal move generator and extractor is demonstrated next.
    let board = OthelloBoard::standard();
    let stone = Stone::Black;
    // Here, the legal moves for black is calculated from the starting
    // position. A bitboard is returned, represented as an `u64`, where each bit
    // with value 1 is a legal move.
    let legal_moves: u64 = board.legal_moves_for(stone);
    // Since the bitboard might be difficult to work with, magpie defines an
    // extension trait called `PositionExt`. It extracts all individual bits
    // that are set to 1 and returns an iterator, yielding these bits as if
    // they were independent bitboards (i.e with only one bit set).
    let mut positions = legal_moves.positions();
    // Here it is verified that all legal moves extracted are indeed legal to
    // play.
    assert!(positions.all(|pos| board.is_legal_move(stone, pos)));
    // ========================================================================
    // Naturally, the library would not be complete without a way of advancing
    // the state of the game.
    let mut board = OthelloBoard::standard();
    let stone = Stone::Black;
    let any_move = board
        .legal_moves_for(stone)
        .positions()
        .next() // Get the first legal move we find
        .unwrap(); // Errors should be handled in a real application
                   // (I won't tell anyone though)

    //This is what the board looks like before any move is made.
    println!("Board before move");
    debug_board(&board);
    // Errors are not handled in this example but should of course be checked
    // in real applications. While it is guaranteed that this specific example
    // will work as intended (knock on wood), the method will return an error
    // if the move is not legal or if the move consists of multiple set bits.
    board.place_stone(stone, any_move).unwrap();
    // Let's see how the board looks like after black made their move.
    println!("Board after move");
    debug_board(&board);
    // ========================================================================
    // Sometimes it is necessary to take complete control of the game. The two
    // methods `place_stone_unchecked` and `remove_stone_unchecked` allow you
    // to both place and remove arbitrary stones without having to comply with
    // Othello's rules. Both methods will always leave the board in a playable
    // state. Reversi can be implemented using these two methods.
    let mut board = OthelloBoard::empty();
    let stone = Stone::Black;
    // In binary this is 32 set bits. It is then padded with 32 zeroes to
    // create an `u64`.
    let pos = u32::MAX as u64;
    // This method allows us to place as many stones as we please at once, as
    // long as no stones of opposite colors overlap. Here we fill half the
    // board with black stones.
    board.place_stone_unchecked(stone, pos).unwrap();
    println!("After placing stones unchecked");
    debug_board(&board);
    // Now we can remove those same stones we placed earlier, leaving a
    // completely empty board behind.
    board.remove_stone_unchecked(stone, pos);
    println!("After removing stones unchecked");
    debug_board(&board);
    // ========================================================================
}

// This is largely unimportant and quite messy.
pub fn debug_board(board: &OthelloBoard) {
    let char_at = |rank: usize, file: usize| {
        let pos = RANKS[rank] & FILES[file];
        board
            .stone_at(pos)
            .map(|stone| match stone {
                Stone::Black => "B",
                Stone::White => "W",
            })
            .unwrap_or(".")
    };

    let black = board.bits_for(Stone::Black);
    let white = board.bits_for(Stone::White);

    if black & white != 0 {
        panic!("Overlapping pieces detected");
    }

    println!("   ABCDEFGH");
    println!("  +--------+");
    for rank in 0..8 {
        print!("{} |", rank + 1);
        for file in 0..8 {
            print!("{}", char_at(rank, file));
        }
        println!("|");
    }
    println!("  +--------+");
}

// Only used for debug purposes.
const FILE_A: u64 = 0x80_80_80_80_80_80_80_80;
const FILE_B: u64 = 0x40_40_40_40_40_40_40_40;
const FILE_C: u64 = 0x20_20_20_20_20_20_20_20;
const FILE_D: u64 = 0x10_10_10_10_10_10_10_10;
const FILE_E: u64 = 0x08_08_08_08_08_08_08_08;
const FILE_F: u64 = 0x04_04_04_04_04_04_04_04;
const FILE_G: u64 = 0x02_02_02_02_02_02_02_02;
const FILE_H: u64 = 0x01_01_01_01_01_01_01_01;
const FILES: [u64; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];

const RANK_1: u64 = 0xff_00_00_00_00_00_00_00;
const RANK_2: u64 = 0x00_ff_00_00_00_00_00_00;
const RANK_3: u64 = 0x00_00_ff_00_00_00_00_00;
const RANK_4: u64 = 0x00_00_00_ff_00_00_00_00;
const RANK_5: u64 = 0x00_00_00_00_ff_00_00_00;
const RANK_6: u64 = 0x00_00_00_00_00_ff_00_00;
const RANK_7: u64 = 0x00_00_00_00_00_00_ff_00;
const RANK_8: u64 = 0x00_00_00_00_00_00_00_ff;
const RANKS: [u64; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];