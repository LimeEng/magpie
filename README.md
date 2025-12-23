[![CI status](https://github.com/LimeEng/magpie/actions/workflows/ci.yaml/badge.svg)](https://github.com/LimeEng/magpie/actions/workflows/ci.yaml)
[![Latest version](https://img.shields.io/crates/v/magpie?color=blue)](https://crates.io/crates/magpie)

# Magpie

<img src="https://cdn.github.emileng.se/repo/magpie/logo.svg" width="200" alt="Magpie logo" align="right">

Magpie is a high-performance [Othello](https://en.wikipedia.org/wiki/Reversi) library built with bitboards and zero dependencies.

Magpie offers two abstraction levels:

- **Game API**: rule-checked, turn-aware game logic and state management. Enforces legal moves and tracks turns.
- **Board API**: lower-level, unchecked board operations for maximum performance.  Useful when building engines.

## Getting Started

```rust
use magpie::othello::{Game, Status, Stone};

let mut game = Game::new();
// Black moves first in Othello
assert_eq!(game.current_turn(), Stone::Black);

// Pick the first available move and play it
let pos = game.moves().hot_bits().next().unwrap();
game.play(pos)?;

println!("{}", game.display());

assert_eq!(game.current_turn(), Stone::White);
assert_eq!(game.status(), Status::Progressing);
```

## Installation

```sh
cargo add magpie
# Serde support is available through the serde feature flag.
cargo add magpie -F serde
```

## Examples

Examples are [described here](/examples).

Curious to play? One example features a functional Othello game with a random AI opponent. Run `cargo run --example human_vs_ai` to start a game!

## Benchmarks

Benchmarks are [described here](/benches)

Run the benchmarks with `cargo bench`.
