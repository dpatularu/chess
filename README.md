# Simple chess engine.
## Everything you need to play chess.

This is a simple chess engine with all the rules implemented. 

## How it works

1. Instantiate a Board object with a FEN string.
```rust
    let fen = "rnbkqbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBKQBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);
```

2. Create a UserMove struct by providing the origin square coordinates, destination square coordinates and an Optional promotion request (necessary for pawn promotions).
```rust
let user_move = UserMove::new((1, 0), (3, 0), None);
```

3. Invoke the make_move method on your board object and pass in your UserMove struct. The method will apply your move and return a MoveOutcome variant.
```rust
match board.make_move(&user_move) {
    MoveOutcome::InvalidMove(err) => {}
    MoveOutcome::Success => {}
    MoveOutcome::GameIsOver(status) => 
    match status {
        GameStatus::Checkmate(winner) => {}
        GameStatus::Draw => {}
        GameStatus::Ongoing => {}
    },
}```

4. Check the status of your game with the game_status method
```rust
match board.game_status() {
    GameStatus::Ongoing => {}
    GameStatus::Checkmate(winner) => {}
    GameStatus::Draw => {}
}```

5. Transform your board back into a FEN string with to_fen
```rust
let fen: String = board.to_fen();
```
