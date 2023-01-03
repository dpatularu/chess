#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

impl Piece {
    pub fn new(kind: PieceKind, color: Color) -> Self {
        Piece { kind, color }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn get_opposite_color(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
pub struct CastleRights {
    pub white_queen_side: bool,
    pub white_king_side: bool,
    pub black_queen_side: bool,
    pub black_king_side: bool,
}

impl CastleRights {
    pub fn from_fen(fen: &str) -> Self {
        let mut castle_rights = CastleRights {
            white_queen_side: false,
            white_king_side: false,
            black_queen_side: false,
            black_king_side: false,
        };
        for c in fen.chars() {
            match c {
                'q' => castle_rights.black_queen_side = true,
                'Q' => castle_rights.white_queen_side = true,
                'k' => castle_rights.black_king_side = true,
                'K' => castle_rights.white_king_side = true,
                '-' => break,
                unexpected_char => panic!("Expected q, Q, k or K; Found: {}", unexpected_char),
            }
        }
        castle_rights
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct UserMove {
    pub origin: (usize, usize),
    pub destination: (usize, usize),
    pub promotion_request: Option<PieceKind>,
}

#[derive(Debug)]
pub struct ValidMove {
    pub origin: (usize, usize),
    pub destination: (usize, usize),
}

pub struct Move {
    pub piece_move: UserMove,
    pub side_effects: Vec<SideEffect>,
}

impl Move {
    pub fn new(
        origin: (usize, usize),
        destination: (usize, usize),
        side_effects: Vec<SideEffect>,
    ) -> Self {
        Move {
            piece_move: UserMove {
                origin,
                destination,
                promotion_request: None,
            },
            side_effects,
        }
    }
}

pub enum MoveOutcome {
    InvalidMove(String),
    GameIsOver(GameStatus),
    Success,
}

#[derive(Debug)]
pub enum GameStatus {
    Ongoing,
    Checkmate(Color),
    Draw,
}
#[derive(Clone, Copy, PartialEq)]
pub enum SideEffect {
    EnPassantTake((usize, usize)),
    PawnMove,
    PieceTaken(Piece),
    DoublePawnMove((usize, usize)),
    Castle((usize, usize), (usize, usize), Color),
    KingMove(Color),
    InitialRookMove(RookType),
    Promotion,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RookType {
    WhiteKingSide,
    WhiteQueenSide,
    BlackKingSide,
    BlackQueenSide,
}
