# Simple chess engine.
## Everything you need to play chess.

This is a simple chess engine with all the rules implemented. 

## How it works

1. Instantiate a Board object with a FEN string.
```rust
    let fen = "rnbkqbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBKQBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);
```
