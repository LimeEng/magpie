use magpie::othello::Stone;

#[test]
fn stone_flip_equality() {
    let stone = Stone::Black;
    assert_eq!(stone, stone.flip().flip());
}
