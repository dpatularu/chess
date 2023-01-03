use chess::chess_frontend;
use chess::models::{GameStatus, MoveOutcome};
use chess::Board;

fn main() {
    let fen = "rnbkqbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBKQBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    loop {
        match board.game_status() {
            GameStatus::Ongoing => {
                let user_move = chess_frontend::get_user_move();
                match board.make_move(&user_move) {
                    MoveOutcome::InvalidMove(err) => println!("{}", err),
                    MoveOutcome::Success => {}
                    MoveOutcome::GameIsOver(status) => match status {
                        GameStatus::Checkmate(winner) => {
                            println!("Game ended. {:?} won.", winner);
                        }
                        GameStatus::Draw => {
                            println!("Game ended in draw.")
                        }
                        _ => {}
                    },
                }
            }
            GameStatus::Checkmate(winner) => {
                println!("Checkmate! {:?} won!", winner);
                break;
            }
            GameStatus::Draw => {
                println!("Draw!");
                break;
            }
        }
    }
}
