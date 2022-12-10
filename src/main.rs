use chess::*;

fn main() {
    let white_knight = Piece {
        kind: PieceKind::Knight,
        color: Color::White,
        position: Position { row: 0, column: 0 },
    };
    let game_state = GameState {
        pieces: vec![white_knight],
    };
    let chess_board_model = ChessBoardModel::new(game_state);

    let terminal_frontend = TerminalFrontend::new(chess_board_model);

    terminal_frontend.print()
}
