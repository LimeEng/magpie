# Benchmarks

Simply run `cargo bench` to run all benchmarks.

- [Clone](#clone)
- [Legal moves](#legal-moves)
- [Place stone](#place-stone)
- [Legal move check](#legal-move-check)
- [Legal moves extraction](#legal-moves-extraction)

## Clone

Simply measures the performance when cloning the standard opening position of Othello.

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

## Legal moves extraction

Measures the performance of extracting all individual legal moves as black given the same board configuration used in the [legal moves benchmark](#legal-moves).
