use crate::{
    actions::play::name_moves,
    interface::chessboard::piece::{fen_to_board, ChessPiece, Color},
};

use super::{
    capture::{get_pieces_by_color, in_check},
    play::{Game, GameState, Move, MoveType},
};

pub fn bishop_possible_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> (Vec<i32>, Vec<i32>) {
    let color = piece.color();
    let mut capture_squares: Vec<i32> = vec![];
    let mut non_capture_squares: Vec<i32> = vec![];

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

            // If the square is empty, it's a valid move square (non-capture)
            if target_piece == ChessPiece::None {
                non_capture_squares.push(new_position);
            } else {
                // If the square is occupied, we can't go further in this direction
                // If it's occupied by an opponent's piece, it's a valid capture square
                if target_piece.color() != color {
                    capture_squares.push(new_position);
                }
                break; // Stop searching in this direction
            }
        }
    }

    (capture_squares, non_capture_squares)
}

pub fn rook_possible_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> (Vec<i32>, Vec<i32>) {
    let color = piece.color();
    let mut capture_squares: Vec<i32> = vec![];
    let mut non_capture_squares: Vec<i32> = vec![];

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

            // If the square is empty, it's a valid move square (non-capture)
            if target_piece == ChessPiece::None {
                non_capture_squares.push(new_position);
            } else {
                // If the square is occupied, we can't go further in this direction
                // If it's occupied by an opponent's piece, it's a valid capture square
                if target_piece.color() != color {
                    capture_squares.push(new_position);
                }
                break; // Stop searching in this direction
            }
        }
    }

    (capture_squares, non_capture_squares)
}

pub fn queen_attacking_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> (Vec<i32>, Vec<i32>) {
    let mut capture_squres: Vec<i32> = vec![];
    let mut non_capture_squares: Vec<i32> = vec![];

    let (b_capture_squares, n_non_capture_squares) =
        bishop_possible_squares(board_pieces, piece, position);
    let (r_capture_squares, r_non_capture_squares) =
        rook_possible_squares(board_pieces, piece, position);

    capture_squres.extend(b_capture_squares);
    capture_squres.extend(r_capture_squares);
    non_capture_squares.extend(n_non_capture_squares);
    non_capture_squares.extend(r_non_capture_squares);

    (capture_squres, non_capture_squares)
}

pub fn knight_possible_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> (Vec<i32>, Vec<i32>) {
    let mut capture_squares: Vec<i32> = vec![];
    let mut non_capture_squares: Vec<i32> = vec![];

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
                non_capture_squares.push(new_position);
            }
            if target_piece != ChessPiece::None {
                // If the piece is an opponent's piece, it's a valid capture square
                if target_piece.color() != piece.color() {
                    capture_squares.push(new_position);
                }
            }
        }
    }

    (capture_squares, non_capture_squares)
}

pub fn pawn_possible_squares(
    board: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
    fen: &str,
) -> Vec<i32> {
    let mut squares: Vec<i32> = vec![];
    let forward_squares = pawn_forward_move(board, piece, position, fen);
    let en_passant_sqr = enpassant_moves(position, piece, fen);
    let capture_squares = pawn_capture_squares(board, piece, position);
    squares.extend(capture_squares);
    squares.extend(en_passant_sqr);
    squares.extend(forward_squares);
    squares
}

pub fn pawn_forward_move(
    board: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
    fen: &str,
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
            && board[target_square as usize] == ChessPiece::None
        {
            squares.push(target_square_2);
        }
    }
    squares
}

pub fn pawn_capture_squares(
    board: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
) -> Vec<i32> {
    let mut squares: Vec<i32> = Vec::new();
    let color = piece.color();
    let direction = if color == Color::White { -1 } else { 1 };

    let left_offset = direction * 8 - 1;
    let right_offset = direction * 8 + 1;

    // Calculate potential capture squares
    for &offset in &[left_offset, right_offset] {
        let target_square = position + offset;
        if target_square >= 0 && target_square < 64 {
            let target_piece = board[target_square as usize];
            if target_piece != ChessPiece::None && target_piece.color() != color {
                squares.push(target_square);
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

pub fn king_adjuscent_squares(position: i32) -> Vec<i32> {
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
    let mut squares: Vec<i32> = vec![];
    let normal_squares = king_normal_squares(board_pieces, piece, position, castle_rules);
    let castle_squares = king_castling(board_pieces, piece, position, castle_rules);

    squares.extend(normal_squares);
    squares.extend(castle_squares);

    squares
}

pub fn king_normal_squares(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
    castle_rules: &str, // "KQkq"
) -> Vec<i32> {
    let mut squares: Vec<i32> = vec![];
    let color = piece.color();

    let opp_king = if color == Color::White {
        ChessPiece::BKing
    } else {
        ChessPiece::WKing
    };

    let opp_king_pos = board_pieces
        .iter()
        .position(|&piece| piece == opp_king)
        .unwrap_or_default();

    // opp king attacking squares
    let opp_king_attacking_squares = king_adjuscent_squares(opp_king_pos as i32);

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

            // opp king should not be able to attack the king's new position
            if opp_king_attacking_squares.contains(&new_position) {
                continue;
            }

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

    squares
}

pub fn king_castling(
    board_pieces: &[ChessPiece; 64],
    piece: ChessPiece,
    position: i32,
    castle_rules: &str, // "KQkq"
) -> Vec<i32> {
    let mut squares: Vec<i32> = vec![];
    let color = piece.color();
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
                        // remove king from original position
                        updated_board[position as usize] = ChessPiece::None;
                        let is_check = in_check(&mut updated_board, color);
                        if !is_check {
                            squares.push(6);
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
                        updated_board[position as usize] = ChessPiece::None;
                        let is_check = in_check(&mut updated_board, color);
                        if !is_check {
                            squares.push(2);
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
                        updated_board[position as usize] = ChessPiece::None;
                        let is_check = in_check(&mut updated_board, color);
                        if !is_check {
                            squares.push(62);
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
                        updated_board[position as usize] = ChessPiece::None;
                        let is_check = in_check(&mut updated_board, color);
                        if !is_check {
                            squares.push(58);
                        }
                    }
                }
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
    pub promote: bool,
    pub promote_to: Option<String>,
    pub move_type: MoveType,
    pub captured_piece: ChessPiece,
}

impl Game {
    pub fn possible_moves(&self, color: Color) -> Vec<PossibleMoves> {
        let board = fen_to_board(&self.fen);
        // return from and to squares
        let color_pieces = get_pieces_by_color(&board, color);
        let poss_moves: Vec<PossibleMoves> = color_pieces
            .into_iter()
            .flat_map(|(piece, position)| {
                let mut poss_moves: Vec<PossibleMoves> = vec![];
                let castle_rules = self.fen.split(" ").collect::<Vec<&str>>()[2];
                let pawn_normal_moves = pawn_forward_move(&board, piece, position, &self.fen);
                let pawn_capture_moves = pawn_capture_squares(&board, piece, position);
                let pawn_enpassant_moves = enpassant_moves(position, piece, &self.fen);

                let mut pawn_moves: Vec<i32> = vec![];
                pawn_moves.extend(pawn_normal_moves);
                pawn_moves.extend(pawn_capture_moves);
                pawn_moves.extend(pawn_enpassant_moves);

                poss_moves.extend(pawn_moves.into_iter().map(|to| PossibleMoves {
                    from: position,
                    to,
                    piece,
                    promote: false,
                    promote_to: None,
                    move_type: MoveType::Normal,
                    captured_piece: board[to as usize].into(),
                }));

                if piece == ChessPiece::WKnight || piece == ChessPiece::BKnight {
                    let (knight_capture_moves, knight_non_capture_moves) =
                        knight_possible_squares(&board, piece, position);
                    poss_moves.extend(
                        knight_capture_moves
                            .into_iter()
                            .map(|to| PossibleMoves {
                                from: position,
                                to,
                                piece,
                                promote: false,
                                promote_to: None,
                                move_type: MoveType::Normal,
                                captured_piece: board[to as usize].into(),
                            })
                            .collect::<Vec<PossibleMoves>>()
                            .into_iter(),
                    );

                    poss_moves.extend(
                        knight_non_capture_moves
                            .into_iter()
                            .map(|to| PossibleMoves {
                                from: position,
                                to,
                                piece,
                                promote: false,
                                promote_to: None,
                                move_type: MoveType::Normal,
                                captured_piece: ChessPiece::None,
                            })
                            .collect::<Vec<PossibleMoves>>()
                            .into_iter(),
                    );
                }

                if piece == ChessPiece::WBishop || piece == ChessPiece::BBishop {
                    let (bishop_capture_moves, bishop_non_capture_moves) =
                        bishop_possible_squares(&board, piece, position);

                    poss_moves.extend(
                        bishop_capture_moves
                            .into_iter()
                            .map(|to| PossibleMoves {
                                from: position,
                                to,
                                piece,
                                promote: false,
                                promote_to: None,
                                move_type: MoveType::Normal,
                                captured_piece: board[to as usize].into(),
                            })
                            .collect::<Vec<PossibleMoves>>()
                            .into_iter(),
                    );

                    poss_moves.extend(
                        bishop_non_capture_moves
                            .into_iter()
                            .map(|to| PossibleMoves {
                                from: position,
                                to,
                                piece,
                                promote: false,
                                promote_to: None,
                                move_type: MoveType::Normal,
                                captured_piece: ChessPiece::None,
                            })
                            .collect::<Vec<PossibleMoves>>()
                            .into_iter(),
                    );
                }

                if piece == ChessPiece::WRook || piece == ChessPiece::BRook {
                    let (rook_capture_moves, rook_non_capture_moves) =
                        rook_possible_squares(&board, piece, position);

                    poss_moves.extend(
                        rook_capture_moves
                            .into_iter()
                            .map(|to| PossibleMoves {
                                from: position,
                                to,
                                piece,
                                promote: false,
                                promote_to: None,
                                move_type: MoveType::Normal,
                                captured_piece: board[to as usize].into(),
                            })
                            .collect::<Vec<PossibleMoves>>()
                            .into_iter(),
                    );

                    poss_moves.extend(
                        rook_non_capture_moves
                            .into_iter()
                            .map(|to| PossibleMoves {
                                from: position,
                                to,
                                piece,
                                promote: false,
                                promote_to: None,
                                move_type: MoveType::Normal,
                                captured_piece: ChessPiece::None,
                            })
                            .collect::<Vec<PossibleMoves>>()
                            .into_iter(),
                    );
                }

                if piece == ChessPiece::WQueen || piece == ChessPiece::BQueen {
                    let (queen_capture_moves, queen_non_capture_moves) =
                        queen_attacking_squares(&board, piece, position);

                    poss_moves.extend(
                        queen_capture_moves
                            .into_iter()
                            .map(|to| PossibleMoves {
                                from: position,
                                to,
                                piece,
                                promote: false,
                                promote_to: None,
                                move_type: MoveType::Normal,
                                captured_piece: board[to as usize].into(),
                            })
                            .collect::<Vec<PossibleMoves>>()
                            .into_iter(),
                    );

                    poss_moves.extend(
                        queen_non_capture_moves
                            .into_iter()
                            .map(|to| PossibleMoves {
                                from: position,
                                to,
                                piece,
                                promote: false,
                                promote_to: None,
                                move_type: MoveType::Normal,
                                captured_piece: ChessPiece::None,
                            })
                            .collect::<Vec<PossibleMoves>>()
                            .into_iter(),
                    );
                }

                if piece == ChessPiece::WKing || piece == ChessPiece::BKing {
                    poss_moves.extend(
                        king_normal_squares(&board, piece, position, castle_rules)
                            .into_iter()
                            .map(|to| PossibleMoves {
                                from: position,
                                to,
                                piece,
                                promote: false,
                                promote_to: None,
                                move_type: MoveType::Normal,
                                captured_piece: board[to as usize].into(),
                            })
                            .collect::<Vec<PossibleMoves>>()
                            .into_iter(),
                    );

                    poss_moves.extend(
                        king_castling(&board, piece, position, castle_rules)
                            .into_iter()
                            .map(|to| PossibleMoves {
                                from: position,
                                to,
                                piece,
                                promote: false,
                                promote_to: None,
                                move_type: MoveType::Castle,
                                captured_piece: ChessPiece::None,
                            })
                            .collect::<Vec<PossibleMoves>>()
                            .into_iter(),
                    );
                }

                poss_moves
            })
            .collect::<Vec<PossibleMoves>>();

        poss_moves
    }
}

fn count_possible_moves(depth: i32, game: &mut Game) -> i32 {
    if depth == 0 {
        return 1;
    }

    let color_part = game.fen.split(" ").collect::<Vec<&str>>()[1];
    let color = if color_part == "w" {
        Color::White
    } else {
        Color::Black
    };
    let castling_part = game.fen.split(" ").collect::<Vec<&str>>()[2];
    let moves: Vec<PossibleMoves> = game.possible_moves(color);
    let mut num_posions = 0;
    for mv in moves {
        let move_ = Move {
            from: mv.from,
            to: mv.to,
            piece: mv.piece,
            promote: "".into(),
            move_type: mv.move_type,
            captured_piece: mv.captured_piece,
            castling_rights: castling_part.into(),
        };
        let game = &mut game.clone();

        game.make_move(&move_);
        num_posions += count_possible_moves(depth - 1, game);

        game.unmake_move(&move_);
    }
    num_posions
}

#[cfg(test)]
#[test]
fn test_possible_moves() {
    for i in 1..=3 {
        let start_time = std::time::Instant::now();
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut game = Game::new(fen.into(), None);
        let num_pos = count_possible_moves(i, &mut game);
        let end_time = std::time::Instant::now();
        println!(
            "depth: {} position: {} time: {:?}",
            i,
            num_pos,
            end_time - start_time
        );
    }
}
