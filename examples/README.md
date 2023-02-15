# Examples

## Human vs AI

```
cargo run --example human_vs_ai
```

This example allows you to play Othello against an AI that plays random legal moves. It demonstrates a possible use for the higher-level `Game`-struct.

## Board operations

```
cargo run --example board_operations
```

Usage of the lower-level `Board`-struct is demonstrated here, suitable for use with engines.

## Serde

```
cargo run --example serde --features serde
```

Serialization with [Serde](https://serde.rs/) is not supported by default but it is possible to opt into using magpie with serde by enabling the appropriately named feature flag. This example demonstrates how serde can be used to both serialize and deserialize a custom struct containing magpie-structures.
