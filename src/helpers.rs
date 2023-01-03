use crate::models::*;
use crate::moves;
use crate::Board;
use std::collections::HashSet;

pub fn get_attacked_squares(board: &Board, player_turn: Color) -> HashSet<(usize, usize)> {
    let mut attacked_squares: HashSet<(usize, usize)> = HashSet::new();
    for row in 0..8 {
        for column in 0..8 {
            match board.board[row][column] {
                Some(piece) => {
                    if piece.color != player_turn {
                        let moves: Vec<Move> = match piece.kind {
                            PieceKind::King => {
                                moves::king_moves(&board, &piece.color, (row, column))
                            }
                            PieceKind::Queen => {
                                moves::queen_moves(&board, &piece.color, (row, column))
                            }
                            PieceKind::Rook => {
                                moves::rook_moves(&board, &piece.color, (row, column))
                            }
                            PieceKind::Bishop => {
                                moves::bishop_moves(&board, &piece.color, (row, column))
                            }
                            PieceKind::Knight => {
                                moves::knight_moves(&board, &piece.color, (row, column))
                            }
                            PieceKind::Pawn => {
                                moves::pawn_attacking_moves(&board, &piece.color, (row, column))
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

pub fn get_move_from_vec(move_vec: Vec<Move>, move_struct: &UserMove) -> Result<Move, MoveOutcome> {
    for m in move_vec {
        if m.piece_move.origin == move_struct.origin
            && m.piece_move.destination == move_struct.destination
        {
            return Ok(m);
        }
    }
    Err(MoveOutcome::InvalidMove(
        format! {"{} to {} is not a valid move.", tuple_to_square(Some(move_struct.origin)).to_ascii_uppercase(), tuple_to_square(Some(move_struct.destination)).to_ascii_uppercase()},
    ))
}

pub fn parse_en_passant_fen(square: &str) -> Option<(usize, usize)> {
    match square_to_tuple(square) {
        Ok(tuple) => Some(tuple),
        _ => None,
    }
}

pub fn tuple_to_square(tuple: Option<(usize, usize)>) -> String {
    match tuple {
        Some(t) => {
            let mut square = String::new();
            let row: char = char::from_u32(t.0 as u32 + 48).unwrap();
            let column = char::from_u32(t.1 as u32 + 97).unwrap();
            square.push(column);
            square.push(row);
            square
        }
        None => "-".to_owned(),
    }
}

pub fn square_to_tuple(square: &str) -> Result<(usize, usize), String> {
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
pub fn populate_board_from_fen(board: &mut [[Option<Piece>; 8]; 8], fen: &str) {
    let mut row = 0;
    let mut column = 0;
    for c in fen.chars().rev() {
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

pub fn board_to_fen(board: [[Option<Piece>; 8]; 8]) -> String {
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

pub fn castle_rights_to_fen(castle_rights: &CastleRights) -> String {
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

pub fn move_causes_check_on_self(mut board: Board, simulated_move: &Move) -> bool {
    board.handle_move(simulated_move);
    is_checked(&board)
}

pub fn is_checked(board: &Board) -> bool {
    let attacked_squares = get_attacked_squares(&board, board.player_turn);
    let king_square = get_king_square(board.board, &board.player_turn);
    attacked_squares.contains(&king_square)
}

pub fn get_king_square(board: [[Option<Piece>; 8]; 8], color: &Color) -> (usize, usize) {
    for row in 0..8 {
        for column in 0..8 {
            if let Some(piece) = board[row][column] {
                if piece.kind == PieceKind::King && piece.color == *color {
                    return (row, column);
                }
            }
        }
    }
    (0, 0)
}
