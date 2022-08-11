use magpie::othello::{Board, Stone, StoneExt};

fn main() {
    // The board can be initialized in a few different ways.

    // ========================================================================
    // This board is completely empty. It is not expected to be heavily used.
    // However, this constructor combined with the method
    // `place_stone_unchecked` can be leveraged to implement a game of Reversi,
    // which is distinctly different from Othello. Unfortunately, some
    // additional bookkeeping is required in this case since magpie does not
    // yet support Reversi.
    let _board = Board::empty();
    // ========================================================================
    // These two constructors are equivalent. These boards are setup in the
    // default Othello opening, as can be seen in this graphic, where "B" and
    // "W" stands for black and white, respectively:
    //      ABCDEFGH
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
    let board_standard = Board::standard();
    let board_default = Board::default();
    assert_eq!(board_standard, board_default);
    // To better visualize what is happening, this debug function is included
    // in this example. This is what the standard Othello opening looks like.
    println!("Standard Othello opening");
    println!("{}", board_standard.display());
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
    assert!(Board::try_from((black_stones, white_stones)).is_err());

    // However, this example will succeed since the stones do not overlap, even
    // if this particular example is unreachable during normal play.
    let black_stones = 1; // In binary: ...0001
    let white_stones = 4; // In binary: ...0100
    assert!(Board::try_from((black_stones, white_stones)).is_ok());
    // ========================================================================
    // The legal move generator and extractor is demonstrated next.
    let board = Board::standard();
    let stone = Stone::Black;
    // Here, the legal moves for black is calculated from the starting
    // position. A bitboard is returned, represented as an `u64`, where each bit
    // with value 1 is a legal move.
    let legal_moves: u64 = board.moves_for(stone);
    // Since the bitboard might be difficult to work with, magpie defines an
    // extension trait called `StoneExt`. It extracts all individual bits
    // that are set to 1 and returns an iterator, yielding these bits as if
    // they were independent bitboards (i.e with only one bit set).
    let mut positions = legal_moves.stones();
    // Here it is verified that all legal moves extracted are indeed legal to
    // play.
    assert!(positions.all(|pos| board.is_legal_move(stone, pos)));
    // ========================================================================
    // Naturally, the library would not be complete without a way of advancing
    // the state of the game.
    let mut board = Board::standard();
    let stone = Stone::Black;
    let any_move = board
        .moves_for(stone)
        .stones()
        .next() // Get the first legal move we find
        .unwrap(); // Errors should be handled in a real application
                   // (I won't tell anyone though)

    //This is what the board looks like before any move is made.
    println!("Board before move");
    println!("{}", board.display());
    // Errors are not handled in this example but should of course be checked
    // in real applications. While it is guaranteed that this specific example
    // will work as intended (knock on wood), the method will return an error
    // if the move is not legal or if the move consists of multiple set bits.
    board.place_stone(stone, any_move).unwrap();
    // Let's see how the board looks like after black made their move.
    println!("Board after move");
    println!("{}", board.display());
    // ========================================================================
    // Sometimes it is necessary to take complete control of the game. The two
    // methods `place_stone_unchecked` and `remove_stone_unchecked` allow you
    // to both place and remove arbitrary stones without having to comply with
    // Othello's rules. Both methods will always leave the board in a playable
    // state. Reversi can be implemented using these two methods.
    let mut board = Board::empty();
    let stone = Stone::Black;
    // In binary this is 32 set bits. It is then padded with 32 zeroes to
    // create an `u64`.
    let pos = u64::from(u32::MAX);
    // This method allows us to place as many stones as we please at once, as
    // long as no stones of opposite colors overlap. Here we fill half the
    // board with black stones.
    board.place_stone_unchecked(stone, pos).unwrap();
    println!("After placing stones unchecked");
    println!("{}", board.display());
    // Now we can remove those same stones we placed earlier, leaving a
    // completely empty board behind.
    board.remove_stone_unchecked(stone, pos);
    println!("After removing stones unchecked");
    println!("{}", board.display());
    // ========================================================================
}
