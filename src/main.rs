use chess::{Board, GameStatus, MoveOutcome, UserMove};

fn main() {
    let fen = "rnbkqbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBKQBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    match board.game_status() {
        GameStatus::Ongoing => {
            let user_move = UserMove::new((1, 0), (3, 0), None);
            match board.make_move(&user_move) {
                MoveOutcome::InvalidMove(err) => println!("{}", err),
                MoveOutcome::Success => {}
                MoveOutcome::GameIsOver(status) => match status {
                    GameStatus::Checkmate(winner) => {}
                    GameStatus::Draw => {}
                    _ => {}
                },
            }
        }
        GameStatus::Checkmate(winner) => {}
        GameStatus::Draw => {}
    }
    println!("{}", board.to_fen());
}
