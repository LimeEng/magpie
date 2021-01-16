use criterion::{black_box, criterion_group, criterion_main, Criterion};
use magpie::othello::{OthelloBoard, SquareExt, Stone, StoneExt};
use std::convert::TryFrom;

fn bench_clone(c: &mut Criterion) {
    let board = OthelloBoard::standard();
    c.bench_function("clone", |b| b.iter(|| board.clone()));
}

fn bench_legal_moves(c: &mut Criterion) {
    let board = board_for_legal_moves();
    c.bench_function("legal_moves", |b| {
        b.iter(|| board.legal_moves_for(black_box(Stone::Black)))
    });
}

fn bench_place_stone(c: &mut Criterion) {
    let board = board_for_place_stone();
    let pos = 0x00_00_00_00_08_00_00_00;
    c.bench_function("place_stone", |b| {
        b.iter(|| {
            board
                .clone()
                .place_stone(black_box(Stone::Black), black_box(pos))
                .unwrap()
        })
    });
}

fn bench_legal_move_check(c: &mut Criterion) {
    let board = board_for_place_stone();
    let pos = 0x00_00_00_00_08_00_00_00;
    c.bench_function("legal_move_check", |b| {
        b.iter(|| board.is_legal_move(black_box(Stone::Black), black_box(pos)))
    });
}

fn bench_legal_moves_extraction(c: &mut Criterion) {
    let board = board_for_legal_moves();
    let moves = board.legal_moves_for(Stone::Black);
    c.bench_function("legal_moves_extraction", |b| {
        b.iter(|| moves.stones().collect::<Vec<u64>>())
    });
}

fn bench_square_extraction(c: &mut Criterion) {
    let board = board_for_legal_moves();
    let moves = board.legal_moves_for(Stone::Black);
    c.bench_function("square_extraction", |b| {
        b.iter(|| moves.squares().collect::<Vec<u64>>())
    });
}

criterion_group!(
    benches,
    bench_clone,
    bench_legal_moves,
    bench_place_stone,
    bench_legal_move_check,
    bench_legal_moves_extraction,
    bench_square_extraction
);
criterion_main!(benches);

fn board_for_place_stone() -> OthelloBoard {
    let black_pos = 0x88_01_00_00_81_00_00_49;
    let white_pos = 0x00_48_2a_1c_76_1c_2a_00;

    OthelloBoard::try_from((black_pos, white_pos)).unwrap()
}

fn board_for_legal_moves() -> OthelloBoard {
    let black_pos = 0x00_11_66_0c_3c_2c_00_00;
    let white_pos = 0x00_66_00_52_40_52_56_00;

    OthelloBoard::try_from((black_pos, white_pos)).unwrap()
}
