use indoc::indoc;
use magpie::othello::{Format, OthelloBoard, Stone};

#[test]
fn display_opening_with_stone_format_standard() {
    let board = OthelloBoard::standard();
    let result = board
        .display()
        .with_format(Format::Standard)
        .with_stone(Stone::Black)
        .to_string();
    let expected = indoc! {"
          A   B   C   D   E   F   G   H
        +---+---+---+---+---+---+---+---+
      1 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      2 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      3 |   |   |   | * |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      4 |   |   | * | W | B |   |   |   |
        +---+---+---+---+---+---+---+---+
      5 |   |   |   | B | W | * |   |   |
        +---+---+---+---+---+---+---+---+
      6 |   |   |   |   | * |   |   |   |
        +---+---+---+---+---+---+---+---+
      7 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      8 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
     "};
    assert_eq!(expected, result);
}

#[test]
fn display_opening_with_stone_format_compact() {
    let board = OthelloBoard::standard();
    let result = board
        .display()
        .with_format(Format::Compact)
        .with_stone(Stone::Black)
        .to_string();
    let expected = indoc! {"
         ABCDEFGH
        +--------+
      1 |........|
      2 |........|
      3 |...*....|
      4 |..*WB...|
      5 |...BW*..|
      6 |....*...|
      7 |........|
      8 |........|
        +--------+
     "};
    assert_eq!(expected, result);
}

#[test]
fn display_opening_format_standard() {
    let board = OthelloBoard::standard();
    let result = board.display().with_format(Format::Standard).to_string();
    let expected = indoc! {"
          A   B   C   D   E   F   G   H
        +---+---+---+---+---+---+---+---+
      1 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      2 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      3 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      4 |   |   |   | W | B |   |   |   |
        +---+---+---+---+---+---+---+---+
      5 |   |   |   | B | W |   |   |   |
        +---+---+---+---+---+---+---+---+
      6 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      7 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      8 |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
      "};
    assert_eq!(expected, result);
}

#[test]
fn display_opening_format_compact() {
    let board = OthelloBoard::standard();
    let result = board.display().with_format(Format::Compact).to_string();
    let expected = indoc! {"
         ABCDEFGH
        +--------+
      1 |........|
      2 |........|
      3 |........|
      4 |...WB...|
      5 |...BW...|
      6 |........|
      7 |........|
      8 |........|
        +--------+
      "};
    assert_eq!(expected, result);
}
