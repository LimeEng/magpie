use magpie::stone::Stone;

#[test]
fn stone_flip_equality() -> Result<(), TestError> {
    let stone = Stone::Black;
    if stone.flip().flip() != stone {
        return Err(TestError::StoneNotEqual);
    }
    Ok(())
}

#[derive(Debug)]
enum TestError {
    StoneNotEqual,
}
