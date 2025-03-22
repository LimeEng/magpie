use magpie::othello::{Board, Stone};

macro_rules! perft_test {
    ($($depth:literal)*) => {
        $(
            perft_test!(#[test] $depth);
        )*
    };
    (ignore $($depth:literal)*) => {
        $(
            perft_test!(#[ignore] #[test] $depth);
        )*
    };
    ($(#[$m:meta])* $depth:literal) => {
        paste::item! {
            $(#[$m])*
            fn [< perft_ $depth >] () {
                test_perft($depth)
            }
        }
    };
}

perft_test!(1 2 3 4 5 6 7 8 9);
// Too expensive to run regularly.
perft_test!(ignore 10 11 12 13 14);

fn test_perft(depth: u8) {
    let target = perft_key(depth);
    let actual = perft(&Board::standard(), Stone::Black, false, depth);
    assert_eq!(target, actual);
}

// https://web.archive.org/web/20120129063410/http://othello.dk/book/index.php/Aart_Bik
fn perft_key(depth: u8) -> u64 {
    #[allow(clippy::unreadable_literal)]
    match depth {
        1 => 4,
        2 => 12,
        3 => 56,
        4 => 244,
        5 => 1396,
        6 => 8200,
        7 => 55092,
        8 => 390216,
        9 => 3005288,
        10 => 24571284,
        11 => 212258800,
        12 => 1939886636,
        13 => 18429641748,
        14 => 184042084512,
        _ => panic!("Unsupported perft depth"),
    }
}

fn perft(board: &Board, stone: Stone, passed: bool, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = board.moves_for(stone);
    if moves == 0 {
        if passed {
            1
        } else {
            perft(board, stone.flip(), true, depth - 1)
        }
    } else if depth == 1 {
        moves.count_set().into()
    } else {
        moves
            .hot_bits()
            .map(|pos| {
                let mut new_board = board.clone();
                new_board.play(stone, pos);
                perft(&new_board, stone.flip(), false, depth - 1)
            })
            .sum()
    }
}
