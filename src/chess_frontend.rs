use crate::models::*;
use crate::*;
use std::io;

pub fn print(board: &Board) {
    // print!("\x1B[2J");
    let board = &board.board;
    for row in (1..=8).rev() {
        print!(" {} ", row);
        for column in 0..8 {
            let c = match &board[row - 1][column] {
                Some(piece) => piece_char_symbol(&piece),
                None => '·',
            };
            print!(" {} ", c);
        }
        print!("\n");
    }
    println!("    A  B  C  D  E  F  G  H ");
}

fn piece_char_symbol(piece: &Piece) -> char {
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

pub fn get_user_move() -> UserMove {
    loop {
        println!(
            "Enter the square of the piece you want to move and its destination. Example: A1 H8"
        );
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let inputs: Vec<_> = buffer.split(' ').collect();
        let move_struct;
        match handle_move_input(&inputs) {
            Ok(move_input) => move_struct = move_input,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
        return move_struct;
    }
}

fn handle_move_input(inputs: &Vec<&str>) -> Result<UserMove, String> {
    let mut origin = (0, 0);
    let mut destination = (0, 0);
    let mut promotion_request = None;
    match inputs.len() {
        2 => {
            for (i, square) in inputs.into_iter().enumerate() {
                match square_to_tuple(square.trim()) {
                    Ok(tuple) => {
                        if i == 0 {
                            origin = tuple;
                        } else {
                            destination = tuple
                        };
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
        }
        3 => {
            for (i, square) in inputs.into_iter().enumerate() {
                if i == 2 {
                    promotion_request = match square.to_ascii_lowercase().chars().next().unwrap() {
                        'q' => Some(PieceKind::Queen),
                        'r' => Some(PieceKind::Rook),
                        'b' => Some(PieceKind::Knight),
                        'n' => Some(PieceKind::Knight),
                        _ => None,
                    }
                } else {
                    match square_to_tuple(square) {
                        Ok(tuple) => {
                            if i == 0 {
                                origin = tuple;
                            } else {
                                destination = tuple
                            };
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
            }
        }
        _ => {
            return Err::<UserMove, String>("Invalid Input".to_owned());
        }
    }
    Ok(UserMove {
        origin,
        destination,
        promotion_request,
    })
}
