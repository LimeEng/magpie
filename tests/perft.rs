use magpie::othello::OthelloBoard;
use magpie::othello::PositionExt;
use magpie::othello::Stone;

macro_rules! perft_tests {
    ($($test_name:ident: $depth:expr,)*) => {
    $(
        #[test]
        fn $test_name() -> Result<(), TestError> {
            let board = OthelloBoard::standard();
            let stone = Stone::Black;
            let target = perft_key($depth)?;
            let nodes = perft(&board, stone, false, $depth);
            if target != nodes {
                return Err(TestError::PerftTargetMissed);
            }
            Ok(())
        }
    )*
    }
}

macro_rules! ignored_perft_tests {
    ($($test_name:ident: $depth:expr,)*) => {
    $(
        #[ignore]
        #[test]
        fn $test_name() -> Result<(), TestError> {
            let board = OthelloBoard::standard();
            let stone = Stone::Black;
            let target = perft_key($depth)?;
            let nodes = perft(&board, stone, false, $depth);
            if target != nodes {
                return Err(TestError::PerftTargetMissed);
            }
            Ok(())
        }
    )*
    }
}

perft_tests! {
    perft_1: 1,
    perft_2: 2,
    perft_3: 3,
    perft_4: 4,
    perft_5: 5,
    perft_6: 6,
    perft_7: 7,
}

ignored_perft_tests! {
    // Too expensive to run regularly.
    perft_8: 8,
    perft_9: 9,
    perft_10: 10,
    perft_11: 11,
    perft_12: 12,
    // Too ridiculously expensive to run at all.
    // perft_13: 13,
    // perft_14: 14,
}

fn perft_key(depth: u8) -> Result<u64, TestError> {
    Ok(match depth {
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
        _ => return Err(TestError::PerftDepthTooLarge),
    })
}

// https://web.archive.org/web/20120129063410/http://othello.dk/book/index.php/Aart_Bik
fn perft(board: &OthelloBoard, stone: Stone, passed: bool, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = board.legal_moves_for(stone);
    if moves == 0 {
        if passed {
            1
        } else {
            perft(board, stone.flip(), true, depth - 1)
        }
    } else {
        moves
            .positions()
            .map(|pos| {
                let mut new_board = board.clone();
                new_board.place_stone(stone, pos).unwrap();
                perft(&new_board, stone.flip(), false, depth - 1)
            })
            .sum()
    }
}

#[derive(Debug)]
enum TestError {
    PerftTargetMissed,
    PerftDepthTooLarge,
}
