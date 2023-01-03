use std::collections::HashSet;

use crate::{helpers::*, models::*, Board};

pub fn pawn_attacking_moves(board: &Board, color: &Color, origin: (usize, usize)) -> Vec<Move> {
    let offset: isize = match color {
        Color::White => 1,
        Color::Black => -1,
    };
    let promotion_row = match color {
        Color::White => 7,
        Color::Black => 0,
    };
    let (row, column) = origin;
    let mut side_effects: Vec<SideEffect> = vec![SideEffect::PawnMove];
    let mut moves: Vec<Move> = Vec::new();

    if (row as isize + offset) as usize == promotion_row {
        side_effects.push(SideEffect::Promotion);
    };
    if column != 0 {
        let destination = ((row as isize + offset) as usize, column - 1);
        match board.board[(row as isize + offset) as usize][column - 1] {
            Some(p) => {
                side_effects.push(SideEffect::PieceTaken(p));
                moves.push(Move::new(origin, destination, side_effects.clone()));
            }
            None => match board.en_passant_square {
                Some(square) => {
                    if square == destination {
                        side_effects.push(SideEffect::EnPassantTake((row, column - 1)));
                        moves.push(Move::new(origin, destination, side_effects.clone()));
                    }
                }
                None => {}
            },
        }
    }
    if column != 7 {
        let destination = ((row as isize + offset) as usize, column + 1);
        match board.board[(row as isize + offset) as usize][column + 1] {
            Some(p) => {
                side_effects.push(SideEffect::PieceTaken(p));
                moves.push(Move::new(origin, destination, side_effects.clone()));
            }
            None => match board.en_passant_square {
                Some(square) => {
                    if square == destination {
                        side_effects.push(SideEffect::EnPassantTake((row, column + 1)));
                        moves.push(Move::new(origin, destination, side_effects.clone()));
                    }
                }
                None => {}
            },
        }
    }
    moves
}

pub fn pawn_moves(board: &Board, color: &Color, origin: (usize, usize)) -> Vec<Move> {
    let offset: isize = match color {
        Color::White => 1,
        Color::Black => -1,
    };
    let pawn_starting_row = match color {
        Color::White => 1,
        Color::Black => 6,
    };
    let promotion_row = match color {
        Color::White => 7,
        Color::Black => 0,
    };
    let (row, column) = origin;
    let mut side_effects: Vec<SideEffect> = vec![];
    let mut moves: Vec<Move> = Vec::new();

    match board.board[(row as isize + offset) as usize][column] {
        Some(_) => {}
        None => {
            if (row as isize + offset) as usize == promotion_row {
                side_effects.push(SideEffect::Promotion);
            };
            let destination = ((row as isize + offset) as usize, column);
            side_effects.push(SideEffect::PawnMove);
            moves.push(Move::new(origin, destination, side_effects.clone()));
        }
    }

    if row == pawn_starting_row {
        match board.board[(row as isize + offset * 2) as usize][column] {
            Some(_) => {}
            None => {
                let destination = ((row as isize + offset * 2) as usize, column);
                side_effects.push(SideEffect::DoublePawnMove((
                    (row as isize + offset) as usize,
                    column,
                )));
                moves.push(Move::new(origin, destination, side_effects.clone()));
            }
        }
    }
    moves
}

pub fn king_moves(board: &Board, color: &Color, origin: (usize, usize)) -> Vec<Move> {
    let offsets: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];
    let mut moves: Vec<Move> = Vec::new();
    for offset in offsets {
        let square = (offset.0 + origin.0 as isize, offset.1 + origin.1 as isize);
        if is_square_within_bounds(square) {
            let square = (square.0 as usize, square.1 as usize);
            match &board.board[square.0][square.1] {
                Some(piece) => {
                    if piece.color != *color {
                        moves.push(Move::new(
                            origin,
                            square,
                            vec![SideEffect::KingMove(*color), SideEffect::PieceTaken(*piece)],
                        ));
                    }
                }
                None => moves.push(Move::new(
                    origin,
                    square,
                    vec![SideEffect::KingMove(*color)],
                )),
            }
        }
    }
    moves
}

pub fn knight_moves(board: &Board, color: &Color, origin: (usize, usize)) -> Vec<Move> {
    let offsets: [(isize, isize); 8] = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];
    let mut moves: Vec<Move> = Vec::new();
    for offset in offsets {
        let square = (offset.0 + origin.0 as isize, offset.1 + origin.1 as isize);
        if is_square_within_bounds(square) {
            let square = (square.0 as usize, square.1 as usize);
            match &board.board[square.0][square.1] {
                Some(piece) => {
                    if piece.color != *color {
                        moves.push(Move::new(
                            origin,
                            square,
                            vec![SideEffect::PieceTaken(*piece)],
                        ));
                    }
                }
                None => moves.push(Move::new(origin, square, vec![])),
            }
        }
    }
    moves
}

pub fn rook_moves(board: &Board, color: &Color, origin: (usize, usize)) -> Vec<Move> {
    let offsets: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut moves: Vec<Move> = Vec::new();
    let side_effect = match color {
        Color::White => match origin {
            (0, 0) => Some(SideEffect::InitialRookMove(RookType::WhiteKingSide)),
            (0, 7) => Some(SideEffect::InitialRookMove(RookType::WhiteQueenSide)),
            _ => None,
        },
        Color::Black => match origin {
            (7, 0) => Some(SideEffect::InitialRookMove(RookType::BlackKingSide)),
            (7, 7) => Some(SideEffect::InitialRookMove(RookType::BlackQueenSide)),
            _ => None,
        },
    };
    for offset in offsets {
        let mut offset_multiplier = 1;
        let mut offsetted_square = (
            offset_multiplier * offset.0 + origin.0 as isize,
            offset_multiplier * offset.1 + origin.1 as isize,
        );
        while is_square_within_bounds(offsetted_square) {
            let mut side_effects: Vec<SideEffect> = vec![];
            if let Some(effect) = side_effect {
                side_effects.push(effect);
            }
            let square = (offsetted_square.0 as usize, offsetted_square.1 as usize);
            match &board.board[square.0][square.1] {
                Some(piece) => {
                    if piece.color != *color {
                        side_effects.push(SideEffect::PieceTaken(*piece));
                        moves.push(Move::new(origin, square, side_effects));
                    } else {
                        break;
                    }
                }
                None => moves.push(Move::new(origin, square, side_effects)),
            }
            offset_multiplier += 1;
            offsetted_square = (
                offset_multiplier * offset.0 + origin.0 as isize,
                offset_multiplier * offset.1 + origin.1 as isize,
            );
        }
    }
    moves
}

pub fn bishop_moves(board: &Board, color: &Color, origin: (usize, usize)) -> Vec<Move> {
    let offsets: [(isize, isize); 4] = [(-1, -1), (1, 1), (1, -1), (1, -1)];
    let mut moves: Vec<Move> = Vec::new();
    for offset in offsets {
        let mut offset_multiplier = 1;
        let mut offsetted_square = (
            offset_multiplier * offset.0 + origin.0 as isize,
            offset_multiplier * offset.1 + origin.1 as isize,
        );
        while is_square_within_bounds(offsetted_square) {
            let square = (offsetted_square.0 as usize, offsetted_square.1 as usize);
            match &board.board[square.0][square.1] {
                Some(piece) => {
                    if piece.color != *color {
                        moves.push(Move::new(
                            origin,
                            square,
                            vec![SideEffect::PieceTaken(*piece)],
                        ));
                    } else {
                        break;
                    }
                }
                None => moves.push(Move::new(origin, square, vec![])),
            }
            offset_multiplier += 1;
            offsetted_square = (
                offset_multiplier * offset.0 + origin.0 as isize,
                offset_multiplier * offset.1 + origin.1 as isize,
            );
        }
    }
    moves
}

pub fn queen_moves(board: &Board, color: &Color, origin: (usize, usize)) -> Vec<Move> {
    let offsets: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];
    let mut moves: Vec<Move> = Vec::new();
    for offset in offsets {
        let mut offset_multiplier = 1;
        let mut offsetted_square = (
            offset_multiplier * offset.0 + origin.0 as isize,
            offset_multiplier * offset.1 + origin.1 as isize,
        );
        while is_square_within_bounds(offsetted_square) {
            let square = (offsetted_square.0 as usize, offsetted_square.1 as usize);
            match &board.board[square.0][square.1] {
                Some(piece) => {
                    if piece.color != *color {
                        moves.push(Move::new(
                            origin,
                            square,
                            vec![SideEffect::PieceTaken(*piece)],
                        ));
                    } else {
                        break;
                    }
                }
                None => moves.push(Move::new(origin, square, vec![])),
            }
            offset_multiplier += 1;
            offsetted_square = (
                offset_multiplier * offset.0 + origin.0 as isize,
                offset_multiplier * offset.1 + origin.1 as isize,
            );
        }
    }
    moves
}

pub fn castle_moves(board: &Board, color: &Color, origin: (usize, usize)) -> Vec<Move> {
    let mut valid_moves: Vec<Move> = Vec::new();
    let attacked_squares = get_attacked_squares(board, board.player_turn);
    let king_starting_square = match color {
        Color::White => (0, 3),
        Color::Black => (7, 3),
    };
    if origin != king_starting_square {
        return valid_moves;
    }
    match color {
        Color::White => {
            if board.castle_rights.white_king_side {
                if king_has_clear_path_to_rook(board.board, vec![(0, 2), (0, 1)], &attacked_squares)
                {
                    valid_moves.push(Move::new(
                        king_starting_square,
                        (0, 1),
                        vec![SideEffect::Castle((0, 0), (0, 2), *color)],
                    ))
                }
            }
            if board.castle_rights.white_queen_side {
                if king_has_clear_path_to_rook(
                    board.board,
                    vec![(0, 4), (0, 5), (0, 6)],
                    &attacked_squares,
                ) {
                    valid_moves.push(Move::new(
                        king_starting_square,
                        (0, 5),
                        vec![SideEffect::Castle((0, 7), (0, 4), *color)],
                    ))
                }
            }
        }
        Color::Black => {
            if board.castle_rights.black_king_side {
                if king_has_clear_path_to_rook(board.board, vec![(7, 2), (7, 1)], &attacked_squares)
                {
                    valid_moves.push(Move::new(
                        king_starting_square,
                        (7, 1),
                        vec![SideEffect::Castle((7, 0), (7, 2), *color)],
                    ))
                }
            }
            if board.castle_rights.black_queen_side {
                if king_has_clear_path_to_rook(
                    board.board,
                    vec![(7, 4), (7, 5), (7, 6)],
                    &attacked_squares,
                ) {
                    valid_moves.push(Move::new(
                        king_starting_square,
                        (7, 5),
                        vec![SideEffect::Castle((7, 7), (7, 4), *color)],
                    ))
                }
            }
        }
    }
    valid_moves
}
fn king_has_clear_path_to_rook(
    board: [[Option<Piece>; 8]; 8],
    squares: Vec<(usize, usize)>,
    attacked_squares: &HashSet<(usize, usize)>,
) -> bool {
    for square in squares {
        match board[square.0][square.1] {
            Some(_) => {
                return false;
            }
            None => {
                if attacked_squares.contains(&square) {
                    return false;
                }
            }
        }
    }
    true
}

fn is_square_within_bounds(square: (isize, isize)) -> bool {
    square.0 >= 0 && square.0 <= 7 && square.1 >= 0 && square.1 <= 7
}
