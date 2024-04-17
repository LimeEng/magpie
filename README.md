[![CI status](https://github.com/LimeEng/magpie/actions/workflows/ci.yml/badge.svg)](https://github.com/LimeEng/magpie/actions/workflows/ci.yml)
[![Latest version](https://img.shields.io/crates/v/magpie.svg)](https://crates.io/crates/magpie)

# Magpie

<img src="https://cdn.github.emileng.se/repo/magpie/logo.svg" width="200" align="right">

Magpie is a simple [Othello](https://en.wikipedia.org/wiki/Reversi) library. Othello is a perfect information, zero-sum game for two players.

Magpie is built with bitboards which allows for extremely fast updates and queries. Two abstraction levels are available, the higher level [`Game`] and lower-level [`Board`]. The [`Game`]-struct guarantees that only legal moves will be made and that the board will be kept consistent. The drawback is that it is not as flexible as the alternative, or as performant. The [`Board`]-struct does not keep track of whose turn it is, whether a player passed their turn, or validates inputs, which makes it better suited for engines.

## Table of Contents
- [Documentation](#documentation)
- [Usage](#usage)
- [Crate Features](#crate-features)
- [Examples](#examples)
- [Benchmarks](#benchmarks)

## Documentation

Documentation is hosted on [docs.rs](https://docs.rs/magpie/)

## Usage

Simply run:

```
$ cargo add magpie
```

Alternatively, add this to your `Cargo.toml`:

```toml
[dependencies]
magpie = "0.11"
```

## Crate features

Serialization with [Serde](https://serde.rs/) is not supported by default. If you want to opt into using magpie with Serde you can enable a feature flag. Simply change your magpie dependency to the following:

```toml
[dependencies]
magpie = {version = "0.11", features = ["serde"]}
```

## Examples

Examples are [found here](/examples).

Included as an example is a functional game which allows you to play Othello against a random AI. To start the game, run the following command:

```
$ cargo run --example human_vs_ai
```

## Benchmarks

Benchmarks are [found here](/benches)

These benchmarks are here to guide improvements of the current algorithms as well as the implementation of new features.

Simply run `cargo bench` to run all benchmarks.
