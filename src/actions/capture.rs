use crate::interface::chessboard::piece::{board_to_fen, fen_to_board, ChessPiece, Color};

use super::{
    path::{knight_possible_squares, queen_attacking_squares},
    play::Move,
};

pub fn capture(move_: &Move) -> (String, bool) {
    let board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());
    let piece = board_pieces[move_.from as usize];
    let color = piece.color();

    // get color of piece to be captured
    let capture_piece = board_pieces[move_.to as usize];
    let capture_color = capture_piece.color();

    if color == capture_color {
        return (move_.fen.clone(), false);
    }

    let mut new_board = board_pieces.clone();
    new_board[move_.to as usize] = piece;
    new_board[move_.from as usize] = ChessPiece::None;
    let new_fen = board_to_fen(&new_board);
    (new_fen, true)
}

pub fn in_check(board: &[ChessPiece; 64], color: Color) -> bool {
    let king = if color == Color::White {
        ChessPiece::WKing
    } else {
        ChessPiece::BKing
    };

    let mut king_pos = 0;
    for (i, piece) in board.iter().enumerate() {
        if *piece == king {
            king_pos = i;
            break;
        }
    }
    let king_pos = king_pos as i32;
    // Check if the king is attacked by any opponent's piece
    let opponent_color = if color == Color::White {
        Color::Black
    } else {
        Color::White
    };

    // Check if any opponent's pawn, bishop, rook, queen, or knight can attack the king's position
    let opponent_pieces = get_pieces_by_color(board, opponent_color);

    for &(piece, piece_pos) in &opponent_pieces {
        let attacking_squares: Vec<i32>;

        match piece {
            ChessPiece::BPawn | ChessPiece::WPawn => {
                attacking_squares = super::path::pawn_possible_squares(board, piece, piece_pos);
            }
            ChessPiece::BBishop | ChessPiece::WBishop => {
                attacking_squares = super::path::bishop_possible_squares(board, piece, piece_pos);
            }
            ChessPiece::BRook | ChessPiece::WRook => {
                attacking_squares = super::path::rook_possible_squares(board, piece, piece_pos);
            }
            ChessPiece::BQueen | ChessPiece::WQueen => {
                attacking_squares = queen_attacking_squares(board, piece, piece_pos);
            }
            ChessPiece::BKnight | ChessPiece::WKnight => {
                attacking_squares = knight_possible_squares(board, piece, piece_pos);
            }
            _ => continue, // Skip other pieces
        }
        println!(
            "{:?} attacking_squares: {:?} kibng pos : {:?}",
            piece, attacking_squares, king_pos
        );
        if attacking_squares.contains(&king_pos) {
            return true;
        }
    }

    // If no opponent piece can attack the king's position, the king is not in check
    false
}

// Helper function to get all pieces of a specific color
fn get_pieces_by_color(board: &[ChessPiece; 64], color: Color) -> Vec<(ChessPiece, i32)> {
    // get all pieces of a specific color with their positions
    let mut pieces: Vec<(ChessPiece, i32)> = vec![];
    for (i, piece) in board.iter().enumerate() {
        if piece.color() == color {
            pieces.push((*piece, i as i32));
        }
    }
    pieces
}


// get king by color
pub fn get_king(board: &[ChessPiece; 64], color: Color) -> i32 {
    let king = if color == Color::White {
        ChessPiece::WKing
    } else {
        ChessPiece::BKing
    };

    let mut king_pos = 0;
    for (i, piece) in board.iter().enumerate() {
        if *piece == king {
            king_pos = i;
            break;
        }
    }
    king_pos as i32
}