use magpie::othello::{Bitboard, Board, Stone};

fn main() {
    // The board can be initialized in a few different ways.

    // ========================================================================
    // This board is completely empty. This constructor combined with the
    // function `place_stone_unchecked` can be leveraged to implement a game of
    // Reversi, which is distinctly different from Othello. Unfortunately, some
    // additional bookkeeping is required in this case since magpie does not
    // support Reversi.
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
    // To better visualize what is happening, the board can be displayed in a
    // human friendly way.
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
    // position. A bitboard is returned, represented as an `u64`, where each
    // bit with value 1 is a legal move.
    let legal_moves: Bitboard = board.moves_for(stone);
    // The bitboard is a pretty generic container for an `u64`. The
    // `.hot_bits()` function may be of particular interest, as it iterates
    // through all hot bits contained in the `Bitboard`. These bits are yielded
    // as if they were independent bitboards, and to conserve this invariant in
    // the type system, they are returned as `Position`. `Position` is similar
    // to `Bitboard` but they are guaranteed to always contain exactly one set
    // bit. Here, `.hot_bits()` is used to extract all legal moves.
    let mut positions = legal_moves.hot_bits();
    // Here it is verified that all legal moves extracted are indeed legal to
    // play.
    assert!(positions.all(|pos| board.is_legal_move(stone, pos)));
    // ========================================================================
    // Naturally, the library provides a way to advance the state of the game.
    let mut board = Board::standard();
    let stone = Stone::Black;
    let any_move = board
        .moves_for(stone)
        .hot_bits()
        .next() // Get the first legal move found.
        .unwrap();

    // This is what the board looks like before any move is made.
    println!("Board before move");
    println!("{}", board.display());
    // This function will return an error if the move is illegal or if the move
    // consists of multiple set bits.
    board.play(stone, any_move).unwrap();
    // Let's see how the board looks like after black made their move.
    println!("Board after move");
    println!("{}", board.display());
    // ========================================================================
    // Sometimes it is necessary to take complete control of the game. The two
    // functions `place_stone_unchecked` and `remove_stone_unchecked` allows
    // you to both place and remove arbitrary stones without having to comply
    // with Othello's rules. Both functions will always leave the board in a
    // playable state. Reversi can be implemented using these two functions.
    let mut board = Board::empty();
    let stone = Stone::Black;
    // In binary this is 32 set bits. It is then padded with 32 zeroes to
    // create an `u64`.
    let pos: Bitboard = (u64::from(u32::MAX)).into();
    // This function allows us to place as many stones as we please at once, as
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
