# Benchmarks

Run the benchmarks with `cargo bench`.

- [Clone](#clone)
- [Legal moves](#legal-moves)
- [Play](#play)
- [Legal move check](#legal-move-check)
- [Individual bitboard extraction](#individual-bitboard-extraction)
- [Legal moves extraction](#legal-moves-extraction)
- [Perft](#perft)

## Clone

Measures the performance when cloning the standard opening position of Othello.

## Legal moves

Measures the performance when calculating all legal moves for black given the following board:
```
   ABCDEFGH
  +--------+
1 |........|
2 |.WWB.WWB|
3 |.BB..BB.|
4 |.W.WBBW.|
5 |.WBBBB..|
6 |.WBWBBW.|
7 |.W.W.WW.|
8 |........|
  +--------+
```

Black has an impressive 34 legal moves to make from this position. The board configuration was discovered by [Dmitry Kamenetsky](https://puzzling.stackexchange.com/a/102017).

## Play

Measures the performance when playing the move E5 as black given the following board:

```
   ABCDEFGH
  +--------+
1 |B...B...|
2 |.W..W..B|
3 |..W.W.W.|
4 |...WWW..|
5 |BWWW.WWB|
6 |...WWW..|
7 |..W.W.W.|
8 |.B..B..B|
  +--------+
```

Playing E5 as black will flip 19 white stones.

## Legal move check

Measures the performance of checking if playing E5 as black is legal given the same board configuration as used in the [play benchmark](#play).

## Individual bitboard extraction

Measures the performance of extracting each position of a bitboard, as a bitboard, given the same board configuration used in the [legal moves benchmark](#legal-moves).

## Legal moves extraction

Measures the performance of extracting all individual legal moves as black given the same board configuration used in the [legal moves benchmark](#legal-moves).

## Perft

Measures the performance of a [perft](https://www.chessprogramming.org/Perft) (performance test) calculation from the standard opening position at depths 1 through 9. Perft recursively counts all possible game states at a given depth, exercising move generation, board cloning, and stone placement in aggregate.
