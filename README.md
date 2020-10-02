![CI status](https://github.com/LimeEng/othello/workflows/CI/badge.svg)
[![Latest version](https://img.shields.io/crates/v/magpie.svg)](https://crates.io/crates/magpie)

# Magpie

Magpie is a simple Othello library written in Rust. [Othello](https://en.wikipedia.org/wiki/Reversi) is a perfect information, zero-sum game for two players. Do note that Reversi is a very similar game but with slightly different rules, currently not implemented in this library.

Magpie is intentionally minimalistic and delegates some basic functionality to the user. For example, keeping track of the next player to play is handled by the user. The library is responsible for both calculating legal moves and updating the state of the board when one is applied. It does this reasonably efficiently by using bitboards to store the state of the board.
