use crate::interface::chessboard::piece::{fen_to_board, ChessPiece, Color};

use super::capture::{get_pieces_by_color, in_check};

pub fn bishop_possible_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> Vec<i32> {
    let color = piece.color();
    let mut squares: Vec<i32> = Vec::new();

    // Define the possible directions for a bishop (diagonal)
    let directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let x = position / 8;
    let y = position % 8;

    for &(dx, dy) in &directions {
        let mut new_x = x;
        let mut new_y = y;

        while new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
            new_x += dx;
            new_y += dy;
            let new_position = new_x * 8 + new_y;

            // Check if the new position is within bounds
            if new_position >= 0 && new_position < 64 {
                let target_piece = board_pieces[new_position as usize];

                if target_piece == ChessPiece::None {
                    squares.push(new_position);
                } else {
                    if target_piece.color() != color {
                        squares.push(new_position);
                    }
                    break; // Stop searching in this direction if an opponent's piece is encountered
                }
            } else {
                break; // Stop searching in this direction if out of bounds
            }
        }
    }

    squares
}

pub fn rook_possible_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> Vec<i32> {
    let color = piece.color();
    let mut squares: Vec<i32> = Vec::new();

    // Define the possible directions for a rook (vertical and horizontal)
    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let x = position / 8;
    let y = position % 8;

    for &(dx, dy) in &directions {
        let mut new_x = x;
        let mut new_y = y;

        while new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
            new_x += dx;
            new_y += dy;
            let new_position = new_x * 8 + new_y;

            // Check if the new position is within bounds
            if new_position >= 0 && new_position < 64 {
                let target_piece = board_pieces[new_position as usize];

                if target_piece == ChessPiece::None {
                    squares.push(new_position);
                } else {
                    if target_piece.color() != color {
                        squares.push(new_position);
                    }
                    break; // Stop searching in this direction if an opponent's piece is encountered
                }
            } else {
                break; // Stop searching in this direction if out of bounds
            }
        }
    }

    squares
}


pub fn queen_attacking_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> Vec<i32> {
    let mut squares: Vec<i32> = vec![];

    let bishop_squares = bishop_possible_squares(board_pieces, piece, position);
    let rook_squares = rook_possible_squares(board_pieces, piece, position);

    // combine the two vectors
    squares.extend(bishop_squares);
    squares.extend(rook_squares);

    squares
}

pub fn knight_possible_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> Vec<i32> {
    let mut squares: Vec<i32> = Vec::new();

    // Define all possible knight moves
    let moves: [(i32, i32); 8] = [
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
    ];

    let x = position / 8;
    let y = position % 8;

    for &(dx, dy) in &moves {
        let new_x = x + dx;
        let new_y = y + dy;

        // Check if the new position is within the bounds of the board
        if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
            let new_position = new_x * 8 + new_y;
            let target_piece = board_pieces[new_position as usize];

            if target_piece == ChessPiece::None || target_piece.color() != piece.color() {
                squares.push(new_position);
            }
        }
    }

    squares
}

pub fn pawn_possible_squares(
    board: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
    fen: &str,
) -> Vec<i32> {
    let mut squares: Vec<i32> = Vec::new();
    let color = piece.color();
    let direction = if color == Color::White { -1 } else { 1 };

    let x = position / 8;
    let y = position % 8;

    // Check one square forward
    let target_square = (x + direction) * 8 + y;
    if target_square >= 0 && target_square < 64 && board[target_square as usize] == ChessPiece::None
    {
        squares.push(target_square);
    }

    // Check two squares forward if it's the pawn's initial move
    if (color == Color::White && x == 6) || (color == Color::Black && x == 1) {
        let target_square_2 = (x + 2 * direction) * 8 + y;
        if target_square_2 >= 0
            && target_square_2 < 64
            && board[target_square_2 as usize] == ChessPiece::None
        {
            squares.push(target_square_2);
        }
    }

    let en_passant_sqr = enpassant_moves(position, piece, fen);
    let capture_squares = pawn_capture_squares(board, piece, position);
    squares.extend(capture_squares);
    squares.extend(en_passant_sqr);

    squares
}

pub fn pawn_capture_squares(
    board: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> Vec<i32> {
    let color = piece.color();
    let direction = if color == Color::White { -1 } else { 1 };

    let x = position / 8;
    let y = position % 8;

    let mut squares: Vec<i32> = Vec::new();

    // Check both diagonal squares
    for dy in [-1, 1].iter() {
        let new_y = y + dy;
        let target_square = (x + direction) * 8 + new_y;

        if new_y >= 0 && new_y < 8 && target_square >= 0 && target_square < 64 {
            let target_piece = board[target_square as usize];

            if target_piece != ChessPiece::None && target_piece.color() != color {
                squares.push(target_square);
            }
        }
    }

    squares
}

pub fn enpassant_moves(from: i32, piece: ChessPiece, fen: &str) -> Vec<i32> {
    let color = piece.color();

    // Split the FEN string into parts
    let fen_parts: Vec<&str> = fen.split_whitespace().collect();

    // Check if en passant is possible
    if fen_parts.len() != 6 || fen_parts[3] == "-" {
        return vec![];
    }

    // Parse the en passant square from FEN
    let enpassant_part = fen_parts[3];
    let enpassant_sqr = (enpassant_part.chars().nth(0).unwrap() as i32 - 97)
        + 8 * (7 - (enpassant_part.chars().nth(1).unwrap() as i32 - 49));

    // Check if the piece is a pawn and the move is valid
    if (color == Color::White && from == enpassant_sqr - 8)
        || (color == Color::Black && from == enpassant_sqr + 8)
    {
        return vec![enpassant_sqr];
    }

    vec![]
}

pub fn king_adjacent_squares(position: i32) -> Vec<i32> {
    let mut squares: Vec<i32> = vec![];

    // Define all possible king moves
    let moves: [(i32, i32); 8] = [
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
    ];

    for &(dx, dy) in &moves {
        let x = position / 8;
        let y = position % 8;
        let new_x = x + dx;
        let new_y = y + dy;

        // Check if the new position is within the bounds of the board
        if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
            let new_position = new_x * 8 + new_y;
            squares.push(new_position);
        }
    }
    squares
}

pub fn king_possible_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
    castle_rules: &str, // "KQkq"
) -> Vec<i32> {
    let mut squares: Vec<i32> = Vec::new();
    let color = piece.color();
    let opp_king = if color == Color::White {
        ChessPiece::BKing
    } else {
        ChessPiece::WKing
    };

    // Find opponent's king position
    let opp_king_pos = board_pieces
        .iter()
        .position(|&p| p == opp_king)
        .unwrap_or_default();
    let opp_king_attacking_squares = king_adjacent_squares(opp_king_pos as i32);

    // Define all possible king moves
    let moves: [(i32, i32); 8] = [
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
    ];

    for &(dx, dy) in &moves {
        let x = position / 8;
        let y = position % 8;
        let new_x = x + dx;
        let new_y = y + dy;

        if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
            let new_position = new_x * 8 + new_y;

            // Opponent's king should not be able to attack the king's new position
            if !opp_king_attacking_squares.contains(&new_position) {
                let target_piece = board_pieces[new_position as usize];
                if target_piece == ChessPiece::None || target_piece.color() != color {
                    let mut new_board = board_pieces.clone();
                    new_board[position as usize] = ChessPiece::None;
                    new_board[new_position as usize] = piece;
                    if !in_check(&mut new_board, color) {
                        squares.push(new_position);
                    }
                }
            }
        }
    }

    // Check for castling moves
    if color == Color::Black && position == 4 && castle_rules.contains('k') {
        let skipping_square = 5;
        if board_pieces[5] == ChessPiece::None && board_pieces[6] == ChessPiece::None {
            let mut updated_board = board_pieces.clone();
            updated_board[skipping_square as usize] = piece;
            updated_board[position as usize] = ChessPiece::None;
            if !in_check(&mut updated_board, color) {
                squares.push(6);
            }
        }
    } else if color == Color::White && position == 60 && castle_rules.contains('K') {
        let skipping_square = 61;
        if board_pieces[61] == ChessPiece::None && board_pieces[62] == ChessPiece::None {
            let mut updated_board = board_pieces.clone();
            updated_board[skipping_square as usize] = piece;
            updated_board[position as usize] = ChessPiece::None;
            if !in_check(&mut updated_board, color) {
                squares.push(62);
            }
        }
    }

    squares
}

pub fn is_insufficient_material(
    black_rem_pieces: Vec<&ChessPiece>,
    white_rem_pieces: Vec<&ChessPiece>,
) -> bool {
    if black_rem_pieces.is_empty() || white_rem_pieces.is_empty() {
        return false;
    }

    // Check for a single knight on each side.
    let black_knight_count = black_rem_pieces
        .iter()
        .filter(|&&piece| piece == &ChessPiece::BKnight)
        .count();
    let white_knight_count = white_rem_pieces
        .iter()
        .filter(|&&piece| piece == &ChessPiece::WKnight)
        .count();
    if black_knight_count == 1 && white_knight_count == 1 {
        return true;
    }

    // More than a single knight.
    if black_rem_pieces
        .iter()
        .any(|piece| piece == &&ChessPiece::BKnight)
        || white_rem_pieces
            .iter()
            .any(|piece| piece == &&ChessPiece::WKnight)
    {
        return false;
    }

    // All bishops on the same color.
    let black_bishops_dark = black_rem_pieces
        .iter()
        .any(|piece| piece == &&ChessPiece::BBishop);
    let black_bishops_light = black_rem_pieces
        .iter()
        .any(|piece| piece == &&ChessPiece::BBishop);
    let white_bishops_dark = white_rem_pieces
        .iter()
        .any(|piece| piece == &&ChessPiece::WBishop);
    let white_bishops_light = white_rem_pieces
        .iter()
        .any(|piece| piece == &&ChessPiece::WBishop);

    if (black_bishops_dark && !black_bishops_light)
        || (!black_bishops_dark && black_bishops_light)
        || (white_bishops_dark && !white_bishops_light)
        || (!white_bishops_dark && white_bishops_light)
    {
        return true;
    }

    false
}

#[derive(Debug, Clone, PartialEq)]
pub struct PossibleMoves {
    pub from: i32,
    pub to: i32,
    pub piece: ChessPiece,
    pub capture: bool,
    pub promote: bool,
    pub promote_to: Option<String>,
}

pub fn color_possible_moves(fen: &str, color: Color) -> Vec<Option<PossibleMoves>> {
    let board = fen_to_board(&fen);
    // return from and to squares

    let color_pieces = get_pieces_by_color(&board, color);
    let poss_moves: Vec<Option<PossibleMoves>> = color_pieces
        .into_iter()
        .flat_map(|(piece, position)| {
            let poss_squares = match piece {
                ChessPiece::BPawn | ChessPiece::WPawn => {
                    pawn_possible_squares(&board, piece, position, fen)
                }
                ChessPiece::BBishop | ChessPiece::WBishop => {
                    bishop_possible_squares(&board, piece, position)
                }
                ChessPiece::BRook | ChessPiece::WRook => {
                    rook_possible_squares(&board, piece, position)
                }
                ChessPiece::BQueen | ChessPiece::WQueen => {
                    queen_attacking_squares(&board, piece, position)
                }
                ChessPiece::BKnight | ChessPiece::WKnight => {
                    knight_possible_squares(&board, piece, position)
                }
                ChessPiece::BKing | ChessPiece::WKing => king_possible_squares(
                    &board,
                    piece,
                    position,
                    &fen.split(" ").collect::<Vec<&str>>()[2],
                ),
                _ => vec![],
            };

            poss_squares.into_iter().map(move |to| {
                let promote = match (piece, to) {
                    (ChessPiece::BPawn, 0..=7) | (ChessPiece::WPawn, 56..=63) => true,
                    _ => false,
                };
                Some(PossibleMoves {
                    from: position,
                    to,
                    piece,
                    capture: board[to as usize] != ChessPiece::None,
                    promote,
                    promote_to: None,
                })
            })
        })
        .collect();

    poss_moves
}
