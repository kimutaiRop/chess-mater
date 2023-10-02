use crate::interface::chessboard::piece::{fen_to_board, ChessPiece, Color};

use super::capture::in_check;

pub fn bishop_possible_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> Vec<i32> {
    let color = piece.color();
    let mut squares: Vec<i32> = vec![];

    // Define the possible directions for a bishop (diagonal)
    let directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    // Loop through each direction
    for &(dx, dy) in &directions {
        let mut x = position / 8;
        let mut y = position % 8;

        // Move along the diagonal until we reach the edge of the board or an occupied square
        loop {
            x += dx;
            y += dy;

            // Check if the new position is out of bounds
            if x < 0 || x >= 8 || y < 0 || y >= 8 {
                break;
            }

            let new_position = x * 8 + y;
            let target_piece = board_pieces[new_position as usize];

            // If the square is empty, it's a valid attack square
            if target_piece == ChessPiece::None {
                squares.push(new_position);
            } else {
                // If the square is occupied, we can't go further in this direction
                // If it's occupied by an opponent's piece, it's a valid attack square
                if target_piece.color() != color {
                    squares.push(new_position);
                }
                break; // Stop searching in this direction
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
    let mut squares: Vec<i32> = vec![];

    // Define the possible directions for a rook (vertical and horizontal)
    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    // Loop through each direction
    for &(dx, dy) in &directions {
        let mut x = position / 8;
        let mut y = position % 8;

        // Move along the direction until we reach the edge of the board or an occupied square
        loop {
            x += dx;
            y += dy;

            // Check if the new position is out of bounds
            if x < 0 || x >= 8 || y < 0 || y >= 8 {
                break;
            }

            let new_position = x * 8 + y;
            let target_piece = board_pieces[new_position as usize];

            // If the square is empty, it's a valid attack square
            if target_piece == ChessPiece::None {
                squares.push(new_position);
            } else {
                // If the square is occupied, we can't go further in this direction
                // If it's occupied by an opponent's piece, it's a valid attack square
                if target_piece.color() != color {
                    squares.push(new_position);
                }
                break; // Stop searching in this direction
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
    let mut squares: Vec<i32> = vec![];

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

    for &(dx, dy) in &moves {
        let x = position / 8;
        let y = position % 8;
        let new_x = x + dx;
        let new_y = y + dy;

        // Check if the new position is within the bounds of the board
        if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
            let new_position = new_x * 8 + new_y;
            let target_piece = board_pieces[new_position as usize];

            if target_piece == ChessPiece::None {
                squares.push(new_position);
            }
            if target_piece != ChessPiece::None {
                // if the piece is an opponent's piece, it's a valid attack square
                if target_piece.color() != piece.color() {
                    squares.push(new_position);
                }
            }
        }
    }

    squares
}

pub fn pawn_possible_squares(
    board: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
    en_passant: Option<i32>,
) -> Vec<i32> {
    let mut squares: Vec<i32> = vec![];
    let color = piece.color();
    let mut direction = 1;
    if color == Color::White {
        direction = -1;
    }

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

    // Check for en passant captures
    if let Some(en_passant_square) = en_passant {
        let en_passant_x = en_passant_square / 8;
        let en_passant_y = en_passant_square % 8;

        if en_passant_x == x && (en_passant_y == y - 1 || en_passant_y == y + 1) {
            squares.push(en_passant_square + 8 * direction);
        }
    }

    let capture_squares = pawn_capture_squares(board, piece, position);
    squares.extend(capture_squares);
    squares
}

pub fn pawn_capture_squares(
    board: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> Vec<i32> {
    // get only the squares that are captures
    let mut squares: Vec<i32> = vec![];
    let color = piece.color();
    let mut direction = 1;
    if color == Color::White {
        direction = -1;
    }

    let x = position / 8;
    let y = position % 8;

    let mut diagonal_squares: Vec<i32> = vec![];
    let mut diagonal_squares_2: Vec<i32> = vec![];
    if y > 0 {
        diagonal_squares.push((x + direction) * 8 + y - 1);
    }
    if y < 7 {
        diagonal_squares_2.push((x + direction) * 8 + y + 1);
    }
    for square in diagonal_squares {
        if square >= 0 && square < 64 {
            let target_piece = board[square as usize];
            if target_piece == ChessPiece::None || target_piece.color() != color {
                squares.push(square);
            }
        }
    }

    for square in diagonal_squares_2 {
        if square >= 0 && square < 64 {
            let target_piece = board[square as usize];
            if target_piece == ChessPiece::None || target_piece.color() != color {
                squares.push(square);
            }
        }
    }

    squares
}

pub fn enpassant_moves(from: i32, piece: ChessPiece, fen: &str) -> Vec<i32> {
    let board = fen_to_board(fen);
    let color = piece.color();

    let from_piece = board[from as usize];
    if from_piece != ChessPiece::BPawn && from_piece != ChessPiece::WPawn {
        return vec![];
    }

    let enpassant_part = fen.split(" ").collect::<Vec<&str>>()[3];
    if enpassant_part == "-" {
        return vec![];
    }

    let mut enpassant_sqr: Option<i32> = None;
    if enpassant_part != "-" {
        enpassant_sqr = Some(
            (enpassant_part.chars().nth(0).unwrap() as i32 - 97) + // a-h
        (8 * (7-(enpassant_part.chars().nth(1).unwrap() as i32 - 49))),
        );
    }

    if enpassant_sqr == None {
        return vec![];
    }

    let enpassant_sqr = enpassant_sqr.unwrap();

    if from != enpassant_sqr - 1 && from != enpassant_sqr + 1 {
        return vec![];
    }

    let move_sqr = if color == Color::White {
        enpassant_sqr - 8
    } else {
        enpassant_sqr + 8
    };

    let emp_piece = board[enpassant_sqr as usize];
    if emp_piece == ChessPiece::None || emp_piece.color() == color {
        return vec![];
    }
    let squares: Vec<i32> = vec![move_sqr];

    squares
}
pub fn king_possible_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
    castle_rules: &str, // "KQkq"
) -> Vec<i32> {
    let mut squares: Vec<i32> = vec![];
    let color = piece.color();

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
            let target_piece = board_pieces[new_position as usize];
            if target_piece == ChessPiece::None || target_piece.color() != color {
                let mut new_board = board_pieces.clone();
                new_board[position as usize] = ChessPiece::None;
                new_board[new_position as usize] = piece;
                let is_check = in_check(&mut new_board, color);
                if !is_check {
                    squares.push(new_position);
                }
            }
        }
    }

    // Check for castling moves
    if color == Color::Black {
        // Check if the balck king is in its initial position
        if position == 4 {
            // Check if the balck king-side rook is in its initial position
            if castle_rules.contains('k') {
                // Check if the squares between the king and the rook are empty
                if board_pieces[5] == ChessPiece::None && board_pieces[6] == ChessPiece::None {
                    // Check if the king is not in check
                    let mut board = board_pieces.clone();
                    let is_check = in_check(&mut board, color);
                    if !is_check {
                        // Check if the king is not passing through a square that is attacked by an opponent's piece
                        let skipping_square = 5;
                        // see if king being at skipping_square is in check
                        let mut updated_board = board_pieces.clone();
                        updated_board[skipping_square as usize] = piece;
                        let is_check = in_check(&mut updated_board, color);
                        if !is_check {
                            let mut board = board_pieces.clone();
                            let is_check = in_check(&mut board, color);
                            if !is_check {
                                squares.push(6);
                            }
                        }
                    }
                }
            }
            // Check if the black queen-side rook is in its initial position
            if castle_rules.contains('q') {
                // Check if the squares between the king and the rook are empty
                if board_pieces[3] == ChessPiece::None
                    && board_pieces[2] == ChessPiece::None
                    && board_pieces[1] == ChessPiece::None
                {
                    // Check if the king is not in check
                    let mut board = board_pieces.clone();
                    let is_check = in_check(&mut board, color);
                    if !is_check {
                        // Check if the king is not passing through a square that is attacked by an opponent's piece
                        let skipping_square = 3;
                        // see if king being at skipping_square is in check
                        let mut updated_board = board_pieces.clone();
                        updated_board[skipping_square as usize] = piece;
                        let is_check = in_check(&mut updated_board, color);
                        if !is_check {
                            let mut board = board_pieces.clone();
                            let is_check = in_check(&mut board, color);
                            if !is_check {
                                squares.push(2);
                            }
                        }
                    }
                }
            }
        }
    } else {
        // Check if the white king is in its initial position
        if position == 60 {
            // Check if the white king-side rook is in its initial position
            if castle_rules.contains('K') {
                // Check if the squares between the king and the rook are empty
                if board_pieces[61] == ChessPiece::None && board_pieces[62] == ChessPiece::None {
                    // Check if the king is not in check
                    let mut board = board_pieces.clone();
                    let is_check = in_check(&mut board, color);
                    if !is_check {
                        // Check if the king is not passing through a square that is attacked by an opponent's piece

                        let skipping_square = 61;
                        // see if king being at skipping_square is in check
                        let mut updated_board = board_pieces.clone();
                        updated_board[skipping_square as usize] = piece;
                        let is_check = in_check(&mut updated_board, color);
                        if !is_check {
                            let mut board = board_pieces.clone();
                            let is_check = in_check(&mut board, color);
                            if !is_check {
                                squares.push(62);
                            }
                        }
                    }
                }
            }
            // Check if the white queen-side rook is in its initial position
            if castle_rules.contains('Q') {
                // Check if the squares between the king and the rook are empty
                if board_pieces[59] == ChessPiece::None
                    && board_pieces[58] == ChessPiece::None
                    && board_pieces[57] == ChessPiece::None
                {
                    // Check if the king is not in check
                    let mut board = board_pieces.clone();
                    let is_check = in_check(&mut board, color);
                    if !is_check {
                        // Check if the king is not passing through a square that is attacked by an opponent's piece
                        let skipping_square = 59;
                        // see if king being at skipping_square is in check
                        let mut updated_board = board_pieces.clone();
                        updated_board[skipping_square as usize] = piece;
                        let is_check = in_check(&mut updated_board, color);
                        if !is_check {
                            let mut board = board_pieces.clone();
                            let is_check = in_check(&mut board, color);
                            if !is_check {
                                squares.push(58);
                            }
                        }
                    }
                }
            }
        }
    }
    squares
}
