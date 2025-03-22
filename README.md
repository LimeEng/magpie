[![CI status](https://github.com/LimeEng/magpie/actions/workflows/ci.yaml/badge.svg)](https://github.com/LimeEng/magpie/actions/workflows/ci.yaml)
[![Latest version](https://img.shields.io/crates/v/magpie?color=blue)](https://crates.io/crates/magpie)

# Magpie

<img src="https://cdn.github.emileng.se/repo/magpie/logo.svg" width="200" alt="Magpie logo" align="right">

Magpie is a high-performance library for the classic board game [Othello](https://en.wikipedia.org/wiki/Reversi). It provides both a user-friendly API and a low-level interface suitable for AI engines.

### Key Features

- **Built with bitboards**: Uses bitboards for extremely fast board operations
- **Zero dependencies**: Core functionality has no external dependencies
- **Optional Serde support**: Serialization available through an optional feature flag

Furthermore, the library offers two abstraction levels:

- **Game API**: Ensures rule compliance, tracks turns, and maintains board consistency
- **Board API**: Provides raw board operations without validation, when performance is critical.

## Installation

```sh
cargo add magpie
# If serialization with Serde is desired, activate the serde feature flag.
cargo add magpie -F serde
```

## Examples

Examples are [described here](/examples).

Curious to play? One example features a functional Othello game with a random AI opponent. Run `cargo run --example human_vs_ai` to start a game!

## Benchmarks

Benchmarks are [described here](/benches)

Simply run `cargo bench` to run all benchmarks.
