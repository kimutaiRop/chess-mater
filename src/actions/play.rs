use crate::interface::chessboard::piece::{board_to_fen, fen_to_board, ChessPiece, Color};

use super::{
    capture::in_check,
    path::{
        bishop_possible_squares, king_possible_squares, knight_possible_squares,
        rook_possible_squares,
    },
};

#[derive(Debug)]
pub struct Move {
    pub from: i32,
    pub to: i32,
    pub piece: ChessPiece,
    pub moved: bool,
    pub fen: String,
}

// Function to check if en passant can be done
fn can_en_passant(move_: &Move) -> bool {
    let board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());
    let enpassant_part = move_.fen.split(" ").collect::<Vec<&str>>()[3];

    if enpassant_part == "-" {
        return false;
    }

    // no enemy piece in enpassant square
    let enpassant_sqr = (enpassant_part.chars().nth(0).unwrap() as i32 - 97) + // a-h
        (8 * (7-(enpassant_part.chars().nth(1).unwrap() as i32 - 49))); // 1-8

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
fn do_en_passant(move_: &Move) -> (String, bool) {
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());
    let enpassant_part = move_.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
    let rules_part = move_.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
    let enpassant_sqr = (enpassant_part.chars().nth(0).unwrap() as i32 - 97) + // a-h
        (8 * (7-(enpassant_part.chars().nth(1).unwrap() as i32 - 49))); // 1-8

    let piece = board_pieces[move_.from as usize];

    let enp_move_sqr = if piece == ChessPiece::BPawn {
        enpassant_sqr + 8
    } else {
        enpassant_sqr - 8
    };

    if piece == ChessPiece::BPawn {
        if enp_move_sqr != move_.to {
            return (move_.fen.clone(), false);
        }
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        board_pieces[enpassant_sqr as usize] = ChessPiece::None;
        let fen = board_to_fen(&board_pieces);
        let fen = fen.replace(enpassant_part, "-");
        let check = in_check(&board_pieces, piece.color());
        if check {
            return (move_.fen.clone(), false);
        }

        return (fen.clone(), true);
    } else if piece == ChessPiece::WPawn {
        if enp_move_sqr != move_.to {
            return (move_.fen.clone(), false);
        }
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        board_pieces[enpassant_sqr as usize] = ChessPiece::None;

        let fen = board_to_fen(&board_pieces);
        let fen = format!("{} {}", fen, rules_part);
        let fen = fen.replace(enpassant_part, "-");

        let check = in_check(&board_pieces, piece.color());
        if check {
            return (move_.fen.clone(), false);
        }
        return (fen.clone(), true);
    }
    return (move_.fen.clone(), false);
}

fn pawn_move(move_: &Move) -> (String, bool) {
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen);
    let mut rules_part = move_.fen.split(" ").collect::<Vec<&str>>()[1..].to_vec();

    let piece = board_pieces[move_.from as usize];
    let enpassant_part = move_.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
    let mut enpassant_sqr: Option<i32> = None;
    if enpassant_part != "-" {
        enpassant_sqr = Some(
            (enpassant_part.chars().nth(0).unwrap() as i32 - 97) + // a-h
        (8 * (7-(enpassant_part.chars().nth(1).unwrap() as i32 - 49))),
        );
    }
    let posible_moves =
        super::path::pawn_possible_squares(&board_pieces, piece, move_.from, enpassant_sqr);

    if !posible_moves.contains(&move_.to) {
        return (move_.fen.clone(), false);
    }
    if can_en_passant(move_) && board_pieces[move_.to as usize] == ChessPiece::None {
        return do_en_passant(move_);
    }

    if posible_moves.contains(&move_.to) {
        let mut enpassant_string = String::from("-");
        let starting_rank = match piece.color() {
            Color::White => 6,
            Color::Black => 1,
        };
        // from_times_moved is 0 if pawn has not moved yet and must have moved 2 squares at start
        if move_.from / 8 == starting_rank && (move_.to / 8 - move_.from / 8).abs() == 2 {
            let row_diff = (move_.to / 8) as i32 - (move_.from / 8) as i32;
            if row_diff.abs() == 2 {
                let file = (move_.to % 8) as i32 + 97;
                let rank = 8 - (move_.to / 8) as i32;
                enpassant_string = format!("{}{}", file as u8 as char, rank);
            }
        }
        rules_part[2] = &enpassant_string;
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        let mut fen = board_to_fen(&board_pieces);
        // replace enpassant part with emp_string
        fen = format!("{} {}", fen, rules_part.join(" "));
        let check = in_check(&board_pieces, piece.color());
        if check {
            return (move_.fen.clone(), false);
        }
        return (fen.clone(), true);
    }

    return (move_.fen.clone(), false);
}

fn bishop_move(move_: &Move) -> (String, bool) {
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());
    let rules_part = move_.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
    let enpassant_part = move_.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
    let piece = board_pieces[move_.from as usize];
    let possible_moves = bishop_possible_squares(&board_pieces, piece, move_.from);
    if possible_moves.contains(&move_.to) {
        board_pieces[move_.to as usize] = piece;
        let check = in_check(&board_pieces, piece.color());
        if check {
            return (move_.fen.clone(), false);
        }
        board_pieces[move_.from as usize] = ChessPiece::None;
        let mut fen = board_to_fen(&board_pieces);
        fen = format!("{} {}", fen, rules_part);
        fen = fen.replace(enpassant_part, "-");

        return (fen.clone(), true);
    }
    return (move_.fen.clone(), false);
}

fn knight_move(move_: &Move) -> (String, bool) {
    // println!("knight {:}", &move_.fen);
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());
    let rules_part = move_.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
    let enpassant_part = move_.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
                                                                         // check if knight is in index from
    let piece = board_pieces[move_.from as usize];

    let possible_moves = knight_possible_squares(&board_pieces, piece, move_.from);
    if possible_moves.contains(&move_.to) {
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        let fen = board_to_fen(&board_pieces);
        let fen = format!("{} {}", fen, rules_part);
        // println!("fen: {:?}", fen);
        // replace enpassant part with '-'
        let fen = fen.replace(enpassant_part, "-");
        let check = in_check(&board_pieces, piece.color());
        if check {
            return (move_.fen.clone(), false);
        }
        return (fen.clone(), true);
    }

    return (move_.fen.clone(), false);
}

fn rook_move(move_: &Move) -> (String, bool) {
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());
    // check if rook is in index from
    let rules_part = move_.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
    let enpassant_part = move_.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
    let piece = board_pieces[move_.from as usize];
    let castling_rights = move_.fen.split(" ").collect::<Vec<&str>>()[2];
    let possible_moves = rook_possible_squares(&board_pieces, piece, move_.from);
    if possible_moves.contains(&move_.to) {
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        let fen = board_to_fen(&board_pieces);
        let fen = format!("{} {}", fen, rules_part);
        // replace enpassant part with '-'
        let fen = fen.replace(enpassant_part, "-");
        let check = in_check(&board_pieces, piece.color());
        if check {
            return (move_.fen.clone(), false);
        }

        // check if castling rights are affected
        let mut rights = String::from(castling_rights);
        // check the side of the rook
        if move_.from % 8 == 0 {
            // left rook
            if piece.color() == Color::White {
                rights = rights.replace("Q", "");
            } else {
                rights = rights.replace("q", "");
            }
        } else if move_.from % 8 == 7 {
            // right rook
            if piece.color() == Color::White {
                rights = rights.replace("K", "");
            } else {
                rights = rights.replace("k", "");
            }
        }
        // if rights is empty, replace with '-'
        if rights == "" {
            rights = String::from("-");
        }
        let fen = fen.replace(castling_rights, &rights);

        return (fen.clone(), true);
    }
    return (move_.fen.clone(), false);
}

fn queen_move(move_: &Move) -> (String, bool) {
    let as_bishop = bishop_move(move_);
    // println!("as_bishop: {:?}", as_bishop);
    if as_bishop.1 {
        return as_bishop;
    }
    let as_rook = rook_move(move_);
    // println!("as_rook: {:?}", as_rook);
    if as_rook.1 {
        return as_rook;
    }
    return (move_.fen.clone(), false);
}

fn castle_move(move_: &Move) -> (String, bool) {
    // println!("castle");
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

    // println!("rook_position: {:?}", rook_from);

    // Check if rook is in the correct position
    let rook = board_pieces[rook_from as usize];

    let original_rights = move_.fen.split(" ").collect::<Vec<&str>>()[2];

    // Determine the appropriate castling rights
    let mut can_castle = match piece.color() {
        Color::White => original_rights.contains("K") || original_rights.contains("Q"),
        Color::Black => original_rights.contains("k") || original_rights.contains("q"),
    };

    // has right to castle to the side
    if is_kingside_castle {
        // check for both black and white
        can_castle = if piece.color() == Color::White {
            can_castle && original_rights.contains("K")
        } else {
            can_castle && original_rights.contains("k")
        };
    } else {
        can_castle = if piece.color() == Color::White {
            can_castle && original_rights.contains("Q")
        } else {
            can_castle && original_rights.contains("q")
        };
    }

    // Check if castling is allowed
    if !can_castle {
        return (move_.fen.clone(), false);
    }

    // Check if all squares between the king and rook are empty
    for i in (rook_from + 1..move_.from).step_by(if is_kingside_castle { 1 } else { usize::MAX }) {
        if board_pieces[i as usize] != ChessPiece::None {
            // println!("squares between king and rook are not empty");
            return (move_.fen.clone(), false);
        }
    }

    // cannot castle out of check
    let is_cheked = in_check(&board_pieces, piece.color());
    if is_cheked {
        return (move_.fen.clone(), false);
    }

    // Update the board after castling
    board_pieces[move_.from as usize] = ChessPiece::None;
    board_pieces[rook_from as usize] = ChessPiece::None;
    board_pieces[move_.to as usize] = piece;
    board_pieces[rook_to as usize] = rook;
    let is_cheked = in_check(&board_pieces, piece.color());
    if is_cheked {
        return (move_.fen.clone(), false);
    }
    let mut fen = board_to_fen(&board_pieces);
    let mut rights = String::from(original_rights);
    if piece.color() == Color::White {
        rights = rights.replace("K", "");
        rights = rights.replace("Q", "");
    } else {
        rights = rights.replace("k", "");
        rights = rights.replace("q", "");
    }
    // if rights is empty, replace with '-'
    if rights == "" {
        rights = String::from("-");
    }
    fen = format!(
        "{} {}",
        fen,
        move_.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ")
    );
    fen = fen.replace(original_rights, &rights);

    (fen.clone(), true)
}

fn king_move(move_: &Move) -> (String, bool) {
    let mut board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());
    let rules_part = move_.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
    let enpassant_part = move_.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
    let castling_rights = move_.fen.split(" ").collect::<Vec<&str>>()[2];
    // check if king is in index from
    let piece = board_pieces[move_.from as usize];

    let possible_sqrs = king_possible_squares(&board_pieces, piece, move_.from, castling_rights);
    if !possible_sqrs.contains(&move_.to) {
        return (move_.fen.clone(), false);
    }
    let col_diff = (move_.to % 8) as i32 - (move_.from % 8) as i32;
    if col_diff.abs() > 1 {
        return castle_move(move_);
    }
    println!("possible_sqrs: {:?}", possible_sqrs);
    if possible_sqrs.contains(&move_.to) {
        // target square does not contain a piece of the same color
        let move_sqr = board_pieces[move_.to as usize];
        if move_sqr != ChessPiece::None {
            if board_pieces[move_.to as usize].color() == piece.color() {
                return (move_.fen.clone(), false);
            }
        }
        board_pieces[move_.to as usize] = piece;
        board_pieces[move_.from as usize] = ChessPiece::None;
        let original_rights = move_.fen.split(" ").collect::<Vec<&str>>()[2];
        let is_check = in_check(&board_pieces, piece.color());
        if is_check {
            return (move_.fen.clone(), false);
        }
        let mut rights = String::from(original_rights);

        if piece.color() == Color::White {
            rights = rights.replace("K", "");
            rights = rights.replace("Q", "");
        } else {
            rights = rights.replace("k", "");
            rights = rights.replace("q", "");
        }
        let fen = board_to_fen(&board_pieces);
        let fen = format!("{} {}", fen, rules_part);
        let fen = fen.replace(enpassant_part, "-");
        let fen = fen.replace(original_rights, &rights);
        return (fen.clone(), true);
    }
    return (move_.fen.clone(), false);
}

pub fn make_move(move_: &Move) -> (String, bool) {
    let board_pieces: [ChessPiece; 64] = fen_to_board(&move_.fen.clone());
    let turn = move_.fen.split(" ").collect::<Vec<&str>>()[1];
    let color = match turn {
        "w" => Color::White,
        "b" => Color::Black,
        _ => Color::White,
    };
    if color != move_.piece.color() {
        return (move_.fen.clone(), false);
    }
    if move_.from == move_.to {
        return (move_.fen.clone(), false);
    }
    let piece = board_pieces[move_.from as usize];
    // choose correct move function
    let move_fn = match piece {
        ChessPiece::BPawn | ChessPiece::WPawn => pawn_move,
        ChessPiece::BBishop | ChessPiece::WBishop => bishop_move,
        ChessPiece::BKnight | ChessPiece::WKnight => knight_move,
        ChessPiece::BRook | ChessPiece::WRook => rook_move,
        ChessPiece::BQueen | ChessPiece::WQueen => queen_move,
        ChessPiece::BKing | ChessPiece::WKing => king_move,
        ChessPiece::None => return (move_.fen.clone(), false),
    };

    let (mut fen, moved) = move_fn(move_);
    if !moved {
        return (fen, false);
    }
    let fen_part = fen.split(" ").collect::<Vec<&str>>()[0];
    let mut rules_part = fen.split(" ").collect::<Vec<&str>>()[1..].to_vec();

    // update move color
    let turn = match turn {
        "w" => "b",
        "b" => "w",
        _ => "w",
    };

    let mut move_count = rules_part[4].parse::<i32>().unwrap();
    if color == Color::Black {
        move_count += 1;
    }
    let move_count = move_count.to_string();

    rules_part[0] = turn;
    rules_part[4] = &move_count;
    let rules_part = rules_part.join(" ");
    fen = format!("{} {}", fen_part, rules_part);

    (fen, moved)
}
