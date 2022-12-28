use chess::chess_frontend;
use chess::Board;

fn main() {
    let fen = "8/8/8/8/3p3p/8/PPPPPPPP/8 w - - 0 0";
    let mut board = Board::from_fen(fen);
    loop {
        chess_frontend::print(&board);
        let user_move = chess_frontend::get_user_move();
        match board.make_move(&user_move) {
            chess::models::MoveOutcome::InvalidMove => println!("cant do that"),
            chess::models::MoveOutcome::Success => {}
        }
    }
}
