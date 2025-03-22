# Examples

## Human vs AI

```sh
cargo run --example human_vs_ai
```

This example allows you to play Othello against an AI that plays random legal moves. It demonstrates a possible use for the higher-level `Game`-struct.

## Board operations

```sh
cargo run --example board_operations
```

Usage of the lower-level `Board`-struct is demonstrated here, suitable for use with engines.

## Serde

```sh
cargo run --example serde --features serde
```

Demonstrates serialization and deserialization of game state using [Serde](https://serde.rs/). Enable Serde support by running `cargo add magpie -F serde`.
