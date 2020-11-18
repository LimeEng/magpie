![CI status](https://github.com/LimeEng/othello/workflows/CI/badge.svg)
[![Latest version](https://img.shields.io/crates/v/magpie.svg)](https://crates.io/crates/magpie)

# Magpie

Magpie is a simple Othello library written in Rust. [Othello](https://en.wikipedia.org/wiki/Reversi) is a perfect information, zero-sum game for two players. Do note that Reversi is a very similar game but with slightly different rules, currently not implemented in this library. However, magpie is flexible enough that it is possible to play Reversi as well, if the user is willing to do some additional bookkeeping.

Magpie is intentionally minimalistic and delegates some basic functionality to the user. For example, keeping track of the next player to play is handled by the user. The library is responsible for both calculating legal moves and updating the state of the board when one is applied. It does this reasonably efficiently by using bitboards to store the state of the board.

The library is intended to be consumed behind another abstraction which may keep track of the next player to play or cache various calculations.

## Table of Contents
- [Documentation](#documentation)
- [Usage](#usage)
- [Crate Features](#crate-features)

## Documentation

Documentation is hosted on [docs.rs](https://docs.rs/magpie/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
magpie = "0.2.0"
```

With magpie installed it is possible to play a game of Othello. Below is an example of how it can be implemented. Do note that error-handling has been omitted for brevity, and the algorithm for choosing the next move is left as an exercise to the reader.

```rust
pub fn play(...) {
    let mut board = OthelloBoard::standard();
    let mut passed_last_turn = false;
    let mut player = Stone::Black;
    while board.free_spaces().count_ones() > 0 {
        let legal_moves = board.legal_moves_for(player);
        if legal_moves == 0 {
            if passed_last_turn {
                break;
            }
            passed_last_turn = true;
        } else {
            let chosen_move = choose_move(...);
            board.place_stone(player, chosen_move).unwrap();
            passed_last_turn = false;
        }
        player = player.flip();
    }
    let black = board.bits_for(Stone::Black).count_ones();
    let white = board.bits_for(Stone::White).count_ones();
    println!("Game finished with {} - {} (black - white)", black, white);
}
```

## Crate features

Serialization with [Serde](https://serde.rs/) is not supported by default. If you want to opt into using magpie with Serde you can enable a feature flag. Simply change your magpie dependency to the following:

```toml
[dependencies]
magpie = {version = "0.2.0", features = ["serde"]}
```
