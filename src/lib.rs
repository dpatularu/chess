pub mod chess_frontend;
pub mod models;

use std::collections::HashSet;

use models::*;

pub struct Board {
    // pub board: [[Option<Piece>; 8]; 8],
    pub board: Vec<Vec<Option<Piece>>>,
    pub player_turn: Color,
    pub castle_rights: CastleRights,
    pub en_passant_square: Option<(usize, usize)>,
    pub num_half_moves: usize,
    pub num_moves: usize,
}

impl Board {
    fn generate_empty_board() -> Vec<Vec<Option<Piece>>> {
        let mut empty_board: Vec<Vec<Option<Piece>>> = Vec::new();
        for _ in 0..8 {
            let mut row = Vec::new();
            for _ in 0..8 {
                row.push(None);
            }
            empty_board.push(row);
        }
        empty_board
    }

    pub fn from_fen(fen: &str) -> Self {
        let fen: Vec<_> = fen.split(' ').collect();

        let mut board = Self::generate_empty_board();
        Self::populate_board_from_fen(&mut board, fen[0]);

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

    fn populate_board_from_fen(board: &mut Vec<Vec<Option<Piece>>>, fen: &str) {
        let mut row = 0;
        let mut column = 0;
        for c in fen.chars() {
            match c {
                'r' => board[row][column] = Some(Piece::new(PieceKind::Rook, Color::Black)),
                'R' => board[row][column] = Some(Piece::new(PieceKind::Rook, Color::White)),
                'n' => board[row][column] = Some(Piece::new(PieceKind::Knight, Color::Black)),
                'N' => board[row][column] = Some(Piece::new(PieceKind::Knight, Color::White)),
                'b' => board[row][column] = Some(Piece::new(PieceKind::Bishop, Color::Black)),
                'B' => board[row][column] = Some(Piece::new(PieceKind::Bishop, Color::White)),
                'q' => board[row][column] = Some(Piece::new(PieceKind::Queen, Color::Black)),
                'Q' => board[row][column] = Some(Piece::new(PieceKind::Queen, Color::White)),
                'k' => board[row][column] = Some(Piece::new(PieceKind::King, Color::Black)),
                'K' => board[row][column] = Some(Piece::new(PieceKind::King, Color::White)),
                'p' => board[row][column] = Some(Piece::new(PieceKind::Pawn, Color::Black)),
                'P' => board[row][column] = Some(Piece::new(PieceKind::Pawn, Color::White)),
                '1'..='8' => {
                    column += c.to_digit(10).unwrap() as usize;
                    continue;
                }
                '/' => {
                    column = 0;
                    row += 1;
                    continue;
                }
                _ => {
                    panic!("Unknown symbol {}", c);
                }
            }
            column += 1;
        }
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        let board = &Self::board_to_fen(&self.board);
        fen.push_str(&format!("{} ", board));
        let player_turn = match self.player_turn {
            Color::White => "w",
            Color::Black => "b",
        };
        fen.push_str(&format!("{} ", player_turn));
        let castle_rights = &Self::castle_rights_to_fen(&self.castle_rights);
        fen.push_str(&format!("{} ", castle_rights));
        let en_passant = tuple_to_square(self.en_passant_square);
        fen.push_str(&format!("{} ", en_passant));
        fen.push_str(&format!("{} ", self.num_half_moves));
        fen.push_str(&format!("{}", self.num_moves));

        fen
    }

    fn board_to_fen(board: &Vec<Vec<Option<Piece>>>) -> String {
        let mut fen = String::new();
        for (i, row) in board.into_iter().enumerate() {
            let mut num_empty_squares = 0;
            for column in row {
                match column {
                    Some(piece) => {
                        if num_empty_squares != 0 {
                            fen.push(char::from_digit(num_empty_squares, 10).unwrap());
                            num_empty_squares = 0;
                        }
                        match piece.color {
                            Color::White => match piece.kind {
                                PieceKind::King => fen.push('K'),
                                PieceKind::Queen => fen.push('Q'),
                                PieceKind::Rook => fen.push('R'),
                                PieceKind::Bishop => fen.push('B'),
                                PieceKind::Knight => fen.push('N'),
                                PieceKind::Pawn => fen.push('P'),
                            },
                            Color::Black => match piece.kind {
                                PieceKind::King => fen.push('k'),
                                PieceKind::Queen => fen.push('q'),
                                PieceKind::Rook => fen.push('r'),
                                PieceKind::Bishop => fen.push('b'),
                                PieceKind::Knight => fen.push('n'),
                                PieceKind::Pawn => fen.push('p'),
                            },
                        }
                    }
                    None => num_empty_squares += 1,
                }
            }
            if num_empty_squares != 0 {
                fen.push(char::from_digit(num_empty_squares, 10).unwrap());
            }
            if i != board.len() - 1 {
                fen.push('/');
            }
        }
        fen
    }

    fn castle_rights_to_fen(castle_rights: &CastleRights) -> String {
        let mut fen = String::new();
        if castle_rights.white_king_side {
            fen.push('K');
        }
        if castle_rights.white_queen_side {
            fen.push('Q');
        }
        if castle_rights.black_king_side {
            fen.push('k');
        }
        if castle_rights.black_queen_side {
            fen.push('q');
        }
        if fen.len() == 0 {
            fen.push('-');
            return fen;
        }
        fen
    }

    pub fn get_attacked_squares(&self) -> HashSet<(usize, usize)> {
        let mut attacked_squares: HashSet<(usize, usize)> = HashSet::new();
        for row in 0..8 {
            for column in 0..8 {
                match &self.board[row][column] {
                    Some(piece) => {
                        if piece.color != self.player_turn {
                            let moves: Vec<Move> = match piece.kind {
                                PieceKind::King => todo!(),
                                PieceKind::Queen => todo!(),
                                PieceKind::Rook => todo!(),
                                PieceKind::Bishop => todo!(),
                                PieceKind::Knight => todo!(),
                                PieceKind::Pawn => {
                                    Self::pawn_attacking_moves(&self, piece, (row, column))
                                }
                            };
                            moves.iter().for_each(|m| {
                                attacked_squares.insert(m.piece_move.destination);
                            });
                        }
                    }
                    None => {}
                }
            }
        }
        attacked_squares
    }

    fn get_all_moves_list(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for row in 0..8 {
            for column in 0..8 {
                match &self.board[row][column] {
                    Some(piece) => {
                        if piece.color == self.player_turn {
                            match piece.kind {
                                PieceKind::King => todo!(),
                                PieceKind::Queen => todo!(),
                                PieceKind::Rook => todo!(),
                                PieceKind::Bishop => todo!(),
                                PieceKind::Knight => todo!(),
                                PieceKind::Pawn => {
                                    moves.append(&mut Self::pawn_moves(
                                        &self,
                                        piece,
                                        (row, column),
                                    ));
                                    moves.append(&mut Self::pawn_attacking_moves(
                                        &self,
                                        piece,
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

    pub fn make_move(&mut self, piece_move: &PieceMove) -> MoveOutcome {
        let all_valid_moves = Self::get_all_moves_list(&self);

        match get_move_from_vec(all_valid_moves, piece_move) {
            Ok(m) => {
                Self::move_piece(self, &m);
                self.player_turn = self.player_turn.get_opposite_color()
            }
            Err(outcome) => {
                return outcome;
            }
        }
        MoveOutcome::Success
    }

    fn move_piece(&mut self, move_struct: &Move) {
        let (origin_row, origin_column) = move_struct.piece_move.origin;
        let (destination_row, destination_column) = move_struct.piece_move.destination;
        if let Some(p) = &self.board[origin_row][origin_column] {
            let piece = Piece::new(p.kind, p.color);
            self.board[destination_row][destination_column] = Some(piece);
            self.board[origin_row][origin_column] = None;
        }
        match &move_struct.side_effect {
            Some(effect) => match effect {
                SideEffect::EnPassantTake(enemy_pawn_coords) => {
                    self.board[enemy_pawn_coords.0][enemy_pawn_coords.1] = None;
                }
                SideEffect::EnPassantMove(square) => {
                    self.en_passant_square = Some(*square);
                }
                SideEffect::Castle(_) => todo!(),
            },
            None => {}
        }
    }

    fn pawn_attacking_moves(&self, piece: &Piece, origin: (usize, usize)) -> Vec<Move> {
        let offset: isize = match piece.color {
            Color::White => -1,
            Color::Black => 1,
        };
        let (row, column) = origin;
        let mut moves: Vec<Move> = Vec::new();
        if column != 0 {
            let destination = ((row as isize + offset) as usize, column - 1);
            match self.board[(row as isize + offset) as usize][column - 1] {
                Some(_) => {
                    moves.push(Move::new(origin, destination, None));
                }
                None => match self.en_passant_square {
                    Some(square) => {
                        if square == destination {
                            moves.push(Move::new(
                                origin,
                                destination,
                                Some(SideEffect::EnPassantTake((row, column - 1))),
                            ));
                        }
                    }
                    None => {}
                },
            }
        }
        if column != 7 {
            let destination = ((row as isize + offset) as usize, column + 1);
            match self.board[(row as isize + offset) as usize][column + 1] {
                Some(_) => {
                    moves.push(Move::new(origin, destination, None));
                }
                None => match self.en_passant_square {
                    Some(square) => {
                        if square == destination {
                            moves.push(Move::new(
                                origin,
                                destination,
                                Some(SideEffect::EnPassantTake((row, column + 1))),
                            ));
                        }
                    }
                    None => {}
                },
            }
        }
        moves
    }

    fn pawn_moves(&self, piece: &Piece, origin: (usize, usize)) -> Vec<Move> {
        let offset: isize = match piece.color {
            Color::White => -1,
            Color::Black => 1,
        };
        let pawn_starting_row = match piece.color {
            Color::White => 6,
            Color::Black => 1,
        };
        let (row, column) = origin;
        let mut moves: Vec<Move> = Vec::new();
        match self.board[(row as isize + offset) as usize][column] {
            Some(_) => {}
            None => {
                let destination = ((row as isize + offset) as usize, column);
                moves.push(Move::new(origin, destination, None));
            }
        }

        if row == pawn_starting_row {
            match self.board[(row as isize + offset * 2) as usize][column] {
                Some(_) => {}
                None => {
                    let destination = ((row as isize + offset * 2) as usize, column);
                    moves.push(Move::new(
                        origin,
                        destination,
                        Some(SideEffect::EnPassantMove((
                            (row as isize + offset) as usize,
                            column,
                        ))),
                    ));
                }
            }
        }
        moves
    }
}

fn get_move_from_vec(move_vec: Vec<Move>, move_struct: &PieceMove) -> Result<Move, MoveOutcome> {
    for m in move_vec {
        if m.piece_move == *move_struct {
            return Ok(m);
        }
    }
    Err(MoveOutcome::InvalidMove)
}

fn parse_en_passant_fen(square: &str) -> Option<(usize, usize)> {
    match square_to_tuple(square) {
        Ok(tuple) => Some(tuple),
        _ => None,
    }
}

fn tuple_to_square(tuple: Option<(usize, usize)>) -> String {
    match tuple {
        Some(t) => {
            let mut square = String::new();
            let row: char = char::from_u32(t.0 as u32 + 48).unwrap();
            square.push(row);
            let column = char::from_u32(t.1 as u32 + 65).unwrap();
            square.push(column);
            square
        }
        None => "-".to_owned(),
    }
}

fn square_to_tuple(square: &str) -> Result<(usize, usize), String> {
    let mut iter = square.chars();
    let row;
    let column;
    match iter.next() {
        Some(c) => match c.to_ascii_uppercase() {
            'A'..='H' => column = c.to_ascii_uppercase() as usize - 65,
            _ => return Err("Invalid Input".to_owned()),
        },
        None => return Err("Invalid Input".to_owned()),
    };

    match iter.next() {
        Some(c) => match c {
            '1'..='8' => {
                row = (c.to_digit(10).unwrap() - 1) as usize;
            }
            _ => return Err("Invalid Input".to_owned()),
        },
        None => return Err("Invalid Input".to_owned()),
    };

    Ok((row, column))
}
