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
#[derive(Clone, Copy)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

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

#[derive(Debug, PartialEq)]
pub struct PieceMove {
    pub origin: (usize, usize),
    pub destination: (usize, usize),
}

#[derive(Debug)]
pub struct Move {
    pub piece_move: PieceMove,
    pub side_effect: Option<SideEffect>,
}

impl Move {
    pub fn new(
        origin: (usize, usize),
        destination: (usize, usize),
        side_effect: Option<SideEffect>,
    ) -> Self {
        Move {
            piece_move: PieceMove {
                origin,
                destination,
            },
            side_effect,
        }
    }
}

pub enum MoveOutcome {
    InvalidMove,
    Success,
}

#[derive(Debug)]
pub enum SideEffect {
    EnPassantTake((usize, usize)),
    EnPassantMove((usize, usize)),
    Castle(Color),
}
