![CI status](https://github.com/LimeEng/magpie/workflows/CI/badge.svg)
[![Latest version](https://img.shields.io/crates/v/magpie.svg)](https://crates.io/crates/magpie)

# Magpie

<img src="https://limeeng.github.io/cdn/repo/magpie/logo.svg" width="200" align="right">

Magpie is a simple Othello library written in Rust. [Othello](https://en.wikipedia.org/wiki/Reversi) is a perfect information, zero-sum game for two players. It is important to note that there is no explicit support for Reversi, a similar game. However, magpie is flexible enough that it is possible to implement Reversi as well, in exchange for some additional bookkeeping.

Magpie is built with bitboards which allows for extremely fast updates and queries. The library is intentionally minimalistic and requires the user to keep track of various aspects of the game, such as the next player to move. Magpie is used for calculating legal moves and applying them while still giving the user enough access to the internals to satisfy a wide array of applications in a safe way.

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
cargo add magpie
```

Alternatively, add this to your `Cargo.toml`:

```toml
[dependencies]
magpie = "0.10"
```

## Crate features

Serialization with [Serde](https://serde.rs/) is not supported by default. If you want to opt into using magpie with Serde you can enable a feature flag. Simply change your magpie dependency to the following:

```toml
[dependencies]
magpie = {version = "0.10", features = ["serde"]}
```

## Examples

Examples are [found here](/examples).

Included as an example is a functional game which allows you to play Othello against a random AI. To start the game, run the following command:

```
cargo run --example human_vs_ai
```

## Benchmarks

Benchmarks are [found here](/benches)

These benchmarks are here to guide improvements of the current algorithms as well as the implementation of new features.

Simply run `cargo bench` to run all benchmarks.
