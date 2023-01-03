mod helpers;
mod models;
mod moves;

use helpers::*;
use models::*;
#[derive(Copy, Clone)]

pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub player_turn: Color,
    pub castle_rights: CastleRights,
    pub en_passant_square: Option<(usize, usize)>,
    pub num_half_moves: usize,
    pub num_moves: usize,
}

impl Board {
    fn generate_empty_board() -> [[Option<Piece>; 8]; 8] {
        let empty_board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        empty_board
    }

    pub fn from_fen(fen: &str) -> Self {
        let fen: Vec<_> = fen.split(' ').collect();

        let mut board = Self::generate_empty_board();
        populate_board_from_fen(&mut board, fen[0]);

        let player_turn = match fen[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Could not parse player turn from fen."),
        };

        let castle_rights = CastleRights::from_fen(fen[2]);
        let en_passant_square = parse_en_passant_fen(fen[3]);
        let num_half_moves = fen[4].parse::<usize>().unwrap();
        let num_moves = fen[5].parse::<usize>().unwrap();

        Board {
            board,
            player_turn,
            castle_rights,
            en_passant_square,
            num_half_moves,
            num_moves,
        }
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        let board = board_to_fen(self.board);
        fen.push_str(&format!("{} ", board));
        let player_turn = match self.player_turn {
            Color::White => "w",
            Color::Black => "b",
        };
        fen.push_str(&format!("{} ", player_turn));
        let castle_rights = &castle_rights_to_fen(&self.castle_rights);
        fen.push_str(&format!("{} ", castle_rights));
        let en_passant = tuple_to_square(self.en_passant_square);
        fen.push_str(&format!("{} ", en_passant));
        fen.push_str(&format!("{} ", self.num_half_moves));
        fen.push_str(&format!("{}", self.num_moves));

        fen
    }

    fn get_all_moves_list(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for row in 0..8 {
            for column in 0..8 {
                match &self.board[row][column] {
                    Some(piece) => {
                        if piece.color == self.player_turn {
                            match piece.kind {
                                PieceKind::King => {
                                    moves.append(&mut moves::king_moves(
                                        &self,
                                        &self.player_turn,
                                        (row, column),
                                    ));
                                    moves.append(&mut moves::castle_moves(
                                        &self,
                                        &self.player_turn,
                                        (row, column),
                                    ))
                                }
                                PieceKind::Queen => moves.append(&mut moves::queen_moves(
                                    &self,
                                    &self.player_turn,
                                    (row, column),
                                )),
                                PieceKind::Rook => moves.append(&mut moves::rook_moves(
                                    &self,
                                    &self.player_turn,
                                    (row, column),
                                )),
                                PieceKind::Bishop => moves.append(&mut moves::bishop_moves(
                                    &self,
                                    &self.player_turn,
                                    (row, column),
                                )),
                                PieceKind::Knight => moves.append(&mut moves::knight_moves(
                                    &self,
                                    &piece.color,
                                    (row, column),
                                )),
                                PieceKind::Pawn => {
                                    moves.append(&mut moves::pawn_moves(
                                        &self,
                                        &piece.color,
                                        (row, column),
                                    ));
                                    moves.append(&mut moves::pawn_attacking_moves(
                                        &self,
                                        &piece.color,
                                        (row, column),
                                    ));
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
        }
        moves
    }

    pub fn make_move(&mut self, piece_move: &UserMove) -> MoveOutcome {
        let moves = Self::get_valid_moves(&self, Self::get_all_moves_list(&self));
        match get_move_from_vec(moves, piece_move) {
            Ok(mut m) => {
                m.piece_move = *piece_move;
                match Self::handle_move(self, &m) {
                    MoveOutcome::InvalidMove(e) => return MoveOutcome::InvalidMove(e),
                    MoveOutcome::GameIsOver(e) => return MoveOutcome::GameIsOver(e),
                    MoveOutcome::Success => {
                        if self.player_turn == Color::Black {
                            self.num_moves += 1;
                        }
                        self.player_turn = self.player_turn.get_opposite_color()
                    }
                }
            }
            Err(outcome) => {
                return outcome;
            }
        }
        MoveOutcome::Success
    }

    pub fn game_status(&self) -> GameStatus {
        let moves = Self::get_valid_moves(&self, Self::get_all_moves_list(&self));
        if moves.len() == 0 {
            if is_checked(self) {
                return GameStatus::Checkmate(Color::get_opposite_color(&self.player_turn));
            } else {
                return GameStatus::Draw;
            }
        } else if self.num_half_moves >= 100 {
            return GameStatus::Draw;
        } else {
            return GameStatus::Ongoing;
        }
    }

    fn copy_board(&self) -> Self {
        self.clone()
    }

    fn get_valid_moves(&self, moves: Vec<Move>) -> Vec<Move> {
        let mut valid_moves: Vec<Move> = Vec::new();
        let board = Self::copy_board(&self);
        for m in moves {
            if !move_causes_check_on_self(board, &m) {
                valid_moves.push(m);
            }
        }
        valid_moves
    }

    pub fn valid_moves(&self) -> Vec<ValidMove> {
        let mut valid_move_vec: Vec<ValidMove> = vec![];
        let moves = self.get_valid_moves(Self::get_all_moves_list(&self));

        for m in moves {
            valid_move_vec.push(ValidMove {
                origin: m.piece_move.origin,
                destination: m.piece_move.destination,
            });
        }

        valid_move_vec
    }

    fn handle_move(&mut self, move_struct: &Move) -> MoveOutcome {
        let mut need_to_reset_half_moves = false;
        let mut double_pawn_move_made = false;
        let mut piece_promoted = false;
        for side_effect in &move_struct.side_effects {
            match side_effect {
                SideEffect::EnPassantTake(enemy_pawn_coords) => {
                    self.board[enemy_pawn_coords.0][enemy_pawn_coords.1] = None;
                }
                SideEffect::DoublePawnMove(square) => {
                    self.en_passant_square = Some(*square);
                    need_to_reset_half_moves = true;
                    double_pawn_move_made = true;
                }
                SideEffect::Castle(rook_origin, rook_destination, color) => {
                    Self::move_piece(self, *rook_origin, *rook_destination);
                    match color {
                        Color::White => {
                            self.castle_rights.white_king_side = false;
                            self.castle_rights.white_queen_side = false;
                        }
                        Color::Black => {
                            self.castle_rights.black_king_side = false;
                            self.castle_rights.black_queen_side = false;
                        }
                    }
                }
                SideEffect::KingMove(color) => match color {
                    Color::White => {
                        self.castle_rights.white_king_side = false;
                        self.castle_rights.white_queen_side = false;
                    }
                    Color::Black => {
                        self.castle_rights.black_king_side = false;
                        self.castle_rights.black_queen_side = false;
                    }
                },
                SideEffect::InitialRookMove(rook_type) => match rook_type {
                    RookType::WhiteKingSide => self.castle_rights.white_king_side = false,
                    RookType::WhiteQueenSide => self.castle_rights.white_queen_side = false,
                    RookType::BlackKingSide => self.castle_rights.black_king_side = false,
                    RookType::BlackQueenSide => self.castle_rights.black_queen_side = false,
                },
                SideEffect::PawnMove => need_to_reset_half_moves = true,
                SideEffect::PieceTaken(_) => need_to_reset_half_moves = true,
                SideEffect::Promotion => match move_struct.piece_move.promotion_request {
                    Some(promotion_piecekind) => match promotion_piecekind {
                        PieceKind::King | PieceKind::Pawn => {
                            return MoveOutcome::InvalidMove(format!(
                                "Cannot promote into {:?}.",
                                promotion_piecekind
                            ))
                        }
                        valid_piece => {
                            let (origin_row, origin_column) = move_struct.piece_move.origin;
                            let piece_color = self.board[origin_row][origin_column].unwrap().color;
                            let promoted_piece = Piece::new(valid_piece, piece_color);
                            let (destination_row, destination_column) =
                                move_struct.piece_move.destination;
                            self.board[origin_row][origin_column] = None;
                            self.board[destination_row][destination_column] = Some(promoted_piece);
                            piece_promoted = true;
                        }
                    },
                    None => {
                        return MoveOutcome::InvalidMove(
                            "Was not provided a piece to promote into.".to_owned(),
                        );
                    }
                },
            }
        }
        if !piece_promoted {
            Self::move_piece(
                self,
                move_struct.piece_move.origin,
                move_struct.piece_move.destination,
            );
        }

        if need_to_reset_half_moves {
            self.num_half_moves = 0;
        } else {
            self.num_half_moves += 1;
        }
        if !double_pawn_move_made {
            self.en_passant_square = None;
        }
        MoveOutcome::Success
    }

    fn move_piece(&mut self, origin: (usize, usize), destination: (usize, usize)) {
        let (origin_row, origin_column) = origin;
        let (destination_row, destination_column) = destination;
        if let Some(p) = &self.board[origin_row][origin_column] {
            let piece = Piece::new(p.kind, p.color);
            self.board[destination_row][destination_column] = Some(piece);
            self.board[origin_row][origin_column] = None;
        }
    }
}

pub enum MoveOutcome {
    InvalidMove(String),
    GameIsOver(GameStatus),
    Success,
}

pub enum GameStatus {
    Ongoing,
    Checkmate(Color),
    Draw,
}

#[derive(PartialEq, Clone, Copy)]
pub struct UserMove {
    pub origin: (usize, usize),
    pub destination: (usize, usize),
    pub promotion_request: Option<PieceKind>,
}

impl UserMove {
    pub fn new(
        origin: (usize, usize),
        destination: (usize, usize),
        promotion_request: Option<PieceKind>,
    ) -> Self {
        UserMove {
            origin,
            destination,
            promotion_request,
        }
    }
}
