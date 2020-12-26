![CI status](https://github.com/LimeEng/magpie/workflows/CI/badge.svg)
[![Latest version](https://img.shields.io/crates/v/magpie.svg)](https://crates.io/crates/magpie)

# Magpie

Magpie is a simple Othello library written in Rust. [Othello](https://en.wikipedia.org/wiki/Reversi) is a perfect information, zero-sum game for two players. Do note that Reversi is a very similar game but with slightly different rules, currently not implemented in this library. However, magpie is flexible enough that it is possible to play Reversi as well, if the user is willing to do some additional bookkeeping.

Magpie is intentionally minimalistic and delegates some basic functionality to the user. For example, keeping track of the next player to play is handled by the user. The library is responsible for both calculating legal moves and updating the state of the board when one is applied. It does this reasonably efficiently by using bitboards to store the state of the board.

The library is intended to be consumed behind another abstraction which may keep track of the next player to play or cache various calculations.

## Table of Contents
- [Documentation](#documentation)
- [Usage](#usage)
- [Crate Features](#crate-features)
- [Examples](#examples)

## Documentation

Documentation is hosted on [docs.rs](https://docs.rs/magpie/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
magpie = "0.4.0"
```

## Crate features

Serialization with [Serde](https://serde.rs/) is not supported by default. If you want to opt into using magpie with Serde you can enable a feature flag. Simply change your magpie dependency to the following:

```toml
[dependencies]
magpie = {version = "0.4.0", features = ["serde"]}
```

## Examples

Examples are [found here](/examples).

Included as an example is a functional game which allows you to play Othello against a random AI. To start the game, run the following command:

```
cargo run --example human_vs_ai
```
