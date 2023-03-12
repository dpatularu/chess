use chess::{Board, GameStatus, MoveError, MoveOutcome, UserMove};

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    match board.game_status() {
        GameStatus::Ongoing => {
            let user_move = UserMove::new((1, 0), (3, 0), None);
            match board.make_move(&user_move) {
                MoveOutcome::Error(err) => match err {
                    MoveError::InvalidPromotion(piece_kind) => {}
                    MoveError::PromotionNotGiven => {}
                    MoveError::InvalidMove(UserMove) => {}
                },
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
    println!("{}", Board::from_fen(fen).to_fen());
}
