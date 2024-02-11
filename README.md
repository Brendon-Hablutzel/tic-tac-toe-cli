# Tic Tac Toe CLI app

A simple implementation of Tic Tac Toe to play on a command line, written in Rust.

## Gameplay

To start the game with cargo, navigate to the project directory and run `cargo run <game_mode>` where `game_mode` is one of the following:
- `player` - starts a Tic Tac Toe game for two players who alternate putting in moves on the same terminal
- `computer` - starts a Tic Tac Toe game where one player plays against a computer, which automatically makes the best move on its turn

## Implementation

The computer uses the minimax algorithm to compute the best move for a given board state.