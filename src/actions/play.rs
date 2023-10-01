use crate::interface::chessboard::piece::{board_to_fen, fen_to_board, ChessPiece, Color};

use super::{
    capture::in_check,
    path::{bishop_possible_squares, knight_possible_squares, rook_possible_squares},
};

#[derive(Debug)]
pub struct Move {
    pub from: i32,
    pub to: i32,
    pub piece: ChessPiece,
    pub moved: bool,
    pub fen: String,
    pub from_times_moved: i32,
    pub can_enpassant: Vec<i32>,
    pub can_castle: Vec<i32>,
}

// Function to check if en passant can be done
fn can_en_passant(move_: &Move) -> bool {
    let board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());

    // Check if there is a pawn in the 'from' square
    let piece = board_pieces[move_.from as usize];

    let from_row = move_.from / 8;
    let to_row = move_.to / 8;
    let from_col = move_.from % 8;
    let to_col = move_.to % 8;
    let row_diff = to_row as i32 - from_row as i32;
    let col_diff = to_col as i32 - from_col as i32;

    if piece == ChessPiece::BPawn {
        if row_diff == 1 && (col_diff == 1 || col_diff == -1) {
            let mut board_pieces_clone = board_pieces.clone();
            board_pieces_clone[move_.to as usize] = piece;
            board_pieces_clone[move_.from as usize] = ChessPiece::None;
            board_pieces_clone[(move_.to - 8) as usize] = ChessPiece::None;
            return !in_check(&board_pieces_clone, piece.color());
        }
    } else if piece == ChessPiece::WPawn {
        if row_diff == -1 && (col_diff == 1 || col_diff == -1) {
            let mut board_pieces_clone = board_pieces.clone();
            board_pieces_clone[move_.to as usize] = piece;
            board_pieces_clone[move_.from as usize] = ChessPiece::None;
            board_pieces_clone[(move_.to + 8) as usize] = ChessPiece::None;
            return !in_check(&board_pieces_clone, piece.color());
        }
    }

    false
}

// Function to perform en passant
fn do_en_passant(move_: &Move) -> (String, Option<i32>, bool) {
    println!("en passant");
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());

    // Check if there is a pawn in the 'from' square
    let piece = board_pieces[move_.from as usize];

    let from_row = move_.from / 8;
    let to_row = move_.to / 8;
    let from_col = move_.from % 8;
    let to_col = move_.to % 8;
    let row_diff = to_row as i32 - from_row as i32;
    let col_diff = to_col as i32 - from_col as i32;

    if piece == ChessPiece::BPawn {
        let replace_square = move_.to - 8;
        if !move_.can_enpassant.contains(&replace_square) {
            return (move_.fen.clone(), None, false);
        }
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        board_pieces[replace_square as usize] = ChessPiece::None;
        let fen = board_to_fen(&board_pieces);
        if row_diff == 1 && col_diff == 1 {
            let check = in_check(&board_pieces, piece.color());
            if check {
                return (move_.fen.clone(), None, false);
            }

            return (fen.clone(), None, true);
        }
        if row_diff == 1 && col_diff == -1 {
            let check = in_check(&board_pieces, piece.color());
            if check {
                return (move_.fen.clone(), None, false);
            }
            return (fen.clone(), None, true);
        }
    } else if piece == ChessPiece::WPawn {
        let replace_square = move_.to + 8;
        if !move_.can_enpassant.contains(&replace_square) {
            return (move_.fen.clone(), None, false);
        }
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        board_pieces[replace_square as usize] = ChessPiece::None;

        let fen = board_to_fen(&board_pieces);
        if row_diff == -1 && col_diff == 1 {
            let check = in_check(&board_pieces, piece.color());
            if check {
                return (move_.fen.clone(), None, false);
            }
            return (fen.clone(), None, true);
        }
        if row_diff == -1 && col_diff == -1 {
            let check = in_check(&board_pieces, piece.color());
            if check {
                return (move_.fen.clone(), None, false);
            }
            return (fen.clone(), None, true);
        }
    }
    return (move_.fen.clone(), None, false);
}

fn pawn_move(move_: &Move) -> (String, Option<i32>, bool) {
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen);

    let piece = board_pieces[move_.from as usize];

    // check if move being made is en passant
    // that is move is diagonal and to square is empty
    println!("move_: {:?}", move_.from_times_moved);
    if can_en_passant(move_) && move_.from_times_moved == 2 && move_.can_enpassant.len() > 0 {
        return do_en_passant(move_);
    }

    let posible_moves = super::path::pawn_possible_squares(&board_pieces, piece, move_.from);
    if posible_moves.contains(&move_.to) {
        let mut can_be_enpassanted: Option<i32> = None;
        // from_times_moved is 0 if pawn has not moved yet and must have moved 2 squares at start
        if move_.from_times_moved == 0 {
            let row_diff = (move_.to / 8) as i32 - (move_.from / 8) as i32;
            if row_diff.abs() == 2 {
                can_be_enpassanted = Some(move_.to);
            }
        }
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        let fen = board_to_fen(&board_pieces);
        let check = in_check(&board_pieces, piece.color());
        if check {
            return (move_.fen.clone(), None, false);
        }
        return (fen.clone(), can_be_enpassanted, true);
    }

    return (move_.fen.clone(), None, false);
}

fn bishop_move(move_: &Move) -> (String, Option<i32>, bool) {
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());

    let piece = board_pieces[move_.from as usize];
    let possible_moves = bishop_possible_squares(&board_pieces, piece, move_.from);
    if possible_moves.contains(&move_.to) {
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        let fen = board_to_fen(&board_pieces);
        let check = in_check(&board_pieces, piece.color());
        if check {
            return (move_.fen.clone(), None, false);
        }
        return (fen.clone(), None, true);
    }
    return (move_.fen.clone(), None, false);
}

fn knight_move(move_: &Move) -> (String, Option<i32>, bool) {
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());

    // check if knight is in index from
    let piece = board_pieces[move_.from as usize];

    let possible_moves = knight_possible_squares(&board_pieces, piece, move_.from);
    if possible_moves.contains(&move_.to) {
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        let fen = board_to_fen(&board_pieces);
        let check = in_check(&board_pieces, piece.color());
        if check {
            return (move_.fen.clone(), None, false);
        }
        return (fen.clone(), None, true);
    }

    return (move_.fen.clone(), None, false);
}

fn rook_move(move_: &Move) -> (String, Option<i32>, bool) {
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());

    let piece = board_pieces[move_.from as usize];

    let possible_moves = rook_possible_squares(&board_pieces, piece, move_.from);
    if possible_moves.contains(&move_.to) {
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        let fen = board_to_fen(&board_pieces);
        let check = in_check(&board_pieces, piece.color());
        if check {
            return (move_.fen.clone(), None, false);
        }
        return (fen.clone(), None, true);
    }
    return (move_.fen.clone(), None, false);
}

fn queen_move(move_: &Move) -> (String, Option<i32>, bool) {
    let as_bishop = bishop_move(move_);
    println!("as_bishop: {:?}", as_bishop);
    if as_bishop.2 {
        return as_bishop;
    }
    let as_rook = rook_move(move_);
    println!("as_rook: {:?}", as_rook);
    if as_rook.2 {
        return as_rook;
    }
    return (move_.fen.clone(), None, false);
}

fn castle_move(move_: &Move) -> (String, Option<i32>, bool) {
    println!("castle");
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());

    // Check if king is in the 'from' square
    let piece = board_pieces[move_.from as usize];

    // Determine the direction of castling
    let is_kingside_castle = move_.to > move_.from;

    // Get the position of the rook to castle with
    let (rook_from, rook_to) = if is_kingside_castle {
        (move_.to + 1, move_.to - 1) // Adjust for kingside castling
    } else {
        (move_.to - 2, move_.to + 1) // Adjust for queenside castling
    };

    println!("rook_position: {:?}", rook_from);

    // Check if rook is in the correct position
    let rook = board_pieces[rook_from as usize];

    // Determine the appropriate castling rights
    let (can_castle, king_castle_row, rook_castle_row, rook_side) =
        match (piece, move_.can_castle.clone()) {
            (ChessPiece::WKing, rights) if is_kingside_castle && rights.contains(&4) => {
                (true, 7, 7, 4)
            } // White kingside
            (ChessPiece::WKing, rights) if !is_kingside_castle && rights.contains(&3) => {
                (true, 7, 7, 3)
            } // White queenside
            (ChessPiece::BKing, rights) if is_kingside_castle && rights.contains(&2) => {
                (true, 0, 0, 2)
            } // Black kingside
            (ChessPiece::BKing, rights) if !is_kingside_castle && rights.contains(&1) => {
                (true, 0, 0, 1)
            } // Black queenside
            _ => (false, -1, -1, 0), // Invalid castling
        };

    // Check if castling is allowed
    if !can_castle {
        println!("can't castle");
        return (move_.fen.clone(), None, false);
    }

    // Check if all squares between the king and rook are empty
    for i in (rook_from + 1..move_.from).step_by(if is_kingside_castle { 1 } else { usize::MAX }) {
        if board_pieces[i as usize] != ChessPiece::None {
            println!("squares between king and rook are not empty");
            return (move_.fen.clone(), None, false);
        }
    }

    // Update the board after castling
    board_pieces[move_.from as usize] = ChessPiece::None;
    board_pieces[rook_from as usize] = ChessPiece::None;
    board_pieces[move_.to as usize] = piece;
    board_pieces[rook_to as usize] = rook;

    let fen = board_to_fen(&board_pieces);
    println!("fen: {:?}", fen);
    (fen.clone(), None, true)
}

fn king_move(move_: &Move) -> (String, Option<i32>, bool) {
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());

    // check if king is in index from
    let piece = board_pieces[move_.from as usize];

    // check if castling
    if move_.can_castle.len() > 0 {
        // check if king is tryin to move more than 1 horizontal square it is castling
        let col_diff = (move_.to % 8) as i32 - (move_.from % 8) as i32;
        if col_diff.abs() > 1 {
            return castle_move(move_);
        }
    }

    // kings can move 1 square in any direction
    let from_row = move_.from / 8;
    let to_row = move_.to / 8;
    let from_col = move_.from % 8;
    let to_col = move_.to % 8;
    let row_diff = to_row as i32 - from_row as i32;
    let col_diff = to_col as i32 - from_col as i32;

    if row_diff.abs() <= 1 && col_diff.abs() <= 1 {
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        let fen = board_to_fen(&board_pieces);
        let check = in_check(&board_pieces, piece.color());
        if check {
            return (move_.fen.clone(), None, false);
        }
        return (fen.clone(), None, true);
    }

    return (move_.fen.clone(), None, false);
}

pub fn make_move(move_: &Move) -> (String, Option<i32>, bool) {
    let board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());

    if move_.from == move_.to {
        return (move_.fen.clone(), None, false);
    }
    let piece = board_pieces[move_.from as usize];
    println!("move_: {:?}", move_);
    // choose correct move function
    let move_fn = match piece {
        ChessPiece::BPawn | ChessPiece::WPawn => pawn_move,
        ChessPiece::BBishop | ChessPiece::WBishop => bishop_move,
        ChessPiece::BKnight | ChessPiece::WKnight => knight_move,
        ChessPiece::BRook | ChessPiece::WRook => rook_move,
        ChessPiece::BQueen | ChessPiece::WQueen => queen_move,
        ChessPiece::BKing | ChessPiece::WKing => king_move,
        ChessPiece::None => return (move_.fen.clone(), None, false),
    };

    move_fn(move_)
}
