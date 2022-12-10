#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
    pub position: Position,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Piece {}

#[derive(Clone, Copy)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub struct ChessBoardModel {
    pub board: [[Option<Piece>; 8]; 8],
    pub game_state: GameState,
}

impl ChessBoardModel {
    pub fn new(game_state: GameState) -> Self {
        let empty_square: Option<Piece> = None;
        let mut board: [[Option<Piece>; 8]; 8] = [[empty_square; 8]; 8];
        for piece in &game_state.pieces {
            board[piece.position.column][piece.position.row] = Some(*piece);
        }
        ChessBoardModel { board, game_state }
    }

    pub fn pieces(&self) -> &Vec<Piece> {
        &self.game_state.pieces
    }
}

pub struct GameState {
    pub pieces: Vec<Piece>,
}

pub struct TerminalFrontend {
    chess_board_model: ChessBoardModel,
}

impl TerminalFrontend {
    pub fn new(chess_board_model: ChessBoardModel) -> Self {
        TerminalFrontend { chess_board_model }
    }

    pub fn print(&self) {
        let board = &self.chess_board_model;
        for row in 0..8 {
            for column in 0..8 {
                let c = match &self.chess_board_model.board[column][row] {
                    Some(piece) => self.piece_char_symbol(piece),
                    None => '.',
                };
                print!(" {} ", c);
            }
            print!("\n");
        }
    }

    fn piece_char_symbol(&self, piece: &Piece) -> char {
        match piece.color {
            Color::White => match piece.kind {
                PieceKind::King => '♔',
                PieceKind::Queen => '♕',
                PieceKind::Rook => '♖',
                PieceKind::Bishop => '♗',
                PieceKind::Knight => '♘',
                PieceKind::Pawn => '♙',
            },
            Color::Black => match piece.kind {
                PieceKind::King => '♚',
                PieceKind::Queen => '♛',
                PieceKind::Rook => '♜',
                PieceKind::Bishop => '♝',
                PieceKind::Knight => '♞',
                PieceKind::Pawn => '♟',
            },
        }
    }
}
