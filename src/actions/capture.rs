use crate::{
    actions::path::{pawn_capture_squares, rook_possible_squares},
    interface::chessboard::piece::{ChessPiece, Color},
};

use super::path::{bishop_possible_squares, knight_possible_squares, queen_attacking_squares};

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
    let mut attacking_squares: Vec<i32> = vec![];
    for &(piece, piece_pos) in &opponent_pieces {
        match piece {
            ChessPiece::BPawn | ChessPiece::WPawn => {
                attacking_squares.extend(pawn_capture_squares(board, piece, piece_pos));
            }
            ChessPiece::BBishop | ChessPiece::WBishop => {
                let ba: (Vec<i32>, Vec<i32>) = bishop_possible_squares(board, piece, piece_pos);
                attacking_squares.extend(ba.0);
                attacking_squares.extend(ba.1);
            }
            ChessPiece::BRook | ChessPiece::WRook => {
                let rook_possible_squares = rook_possible_squares(board, piece, piece_pos);
                attacking_squares.extend(rook_possible_squares.0);
                attacking_squares.extend(rook_possible_squares.1);
            }
            ChessPiece::BQueen | ChessPiece::WQueen => {
                let qa: (Vec<i32>, Vec<i32>) = queen_attacking_squares(board, piece, piece_pos);
                attacking_squares.extend(qa.0);
                attacking_squares.extend(qa.1);
            }
            ChessPiece::BKnight | ChessPiece::WKnight => {
                let knight_possible_squares = knight_possible_squares(board, piece, piece_pos);
                attacking_squares.extend(knight_possible_squares.0);
                attacking_squares.extend(knight_possible_squares.1);
            }
            _ => continue, // Skip other pieces
        }
    }
    attacking_squares.sort();
    attacking_squares.dedup();
    if attacking_squares.contains(&king_pos) {
        return true;
    }
    false
}

// Helper function to get all pieces of a specific color
pub fn get_pieces_by_color(board: &[ChessPiece; 64], color: Color) -> Vec<(ChessPiece, i32)> {
    // get all pieces of a specific color with their positions
    let mut pieces: Vec<(ChessPiece, i32)> = vec![];
    for (i, piece) in board.iter().enumerate() {
        if piece.color() == color && piece != &ChessPiece::None {
            pieces.push((*piece, i as i32));
        }
    }
    pieces
}
