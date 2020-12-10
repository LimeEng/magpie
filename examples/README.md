# Examples

## Human vs AI

```
cargo run --example human_vs_ai
```

This example is by far the most complicated. It allows you to play Othello against a random AI and as a result, a lot of the logic is actually not focused on the library. Magpie is intentionally minimalistic which allows it to give the user complete control over the game. The tradeoff is that the user must do some bookkeeping to set up a fully featured game.

## Serde

```
cargo run --example serde --features serde
```

Serialization with [Serde](https://serde.rs/) is not supported by default but it is possible to opt into using magpie with serde by enabling the appropriately named feature flag. This example demonstrates how serde can be used to both serialize and deserialize a custom struct containing magpie-structures.
