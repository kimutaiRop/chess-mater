use godot::prelude::FromVariant;

use crate::interface::chessboard::piece::{board_to_fen, fen_to_board, ChessPiece, Color};

use super::{
    capture::in_check,
    path::{
        bishop_possible_squares, enpassant_moves, is_insufficient_material, king_possible_squares,
        knight_possible_squares, rook_possible_squares,
    },
    player::Engine,
};

#[derive(Debug, Clone, PartialEq, godot::prelude::ToVariant, FromVariant)]
pub enum MoveType {
    Normal,
    Castle,
    EnPassant,
    Promotion,
}

#[derive(Debug)]
pub struct Move {
    pub from: i32,
    pub to: i32,
    pub piece: ChessPiece,
    pub promote: String,
    pub move_type: MoveType,
    pub captured_piece: ChessPiece,
    pub castling_rights: String,
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    Checkmate,
    Stalemate,
    Normal,
    Draw,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub fen: String,
    pub move_notations: Vec<String>,
    pub game_over: bool,
    pub winner: Option<Color>,
    pub engine: Option<Engine>,
    pub turn: Color,
}

impl Game {
    pub fn new(fen: &str, engine: Option<Engine>) -> Self {
        let turn = fen.split(" ").collect::<Vec<&str>>()[1];
        let turn = match turn {
            "w" => Color::White,
            "b" => Color::Black,
            _ => Color::White,
        };
        Game {
            fen: fen.to_string(),
            move_notations: vec![],
            game_over: false,
            winner: None,
            engine,
            turn: turn,
        }
    }

    // Function to perform en passant
    fn do_en_passant(&self, move_: &Move) -> (String, bool) {
        let mut board_pieces: [ChessPiece; 64] = fen_to_board(&self.fen.clone());
        let enpassant_part = self.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
        let rules_part = self.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
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
                return (self.fen.clone(), false);
            }
            board_pieces[move_.to as usize] = piece;
            board_pieces[move_.from as usize] = ChessPiece::None;
            board_pieces[enpassant_sqr as usize] = ChessPiece::None;
            let fen = board_to_fen(&board_pieces);
            let rules_part = rules_part.replace(enpassant_part, "-");
            let fen = format!("{} {}", fen, rules_part);
            let check = in_check(&board_pieces, piece.color());
            if check {
                return (self.fen.clone(), false);
            }

            return (fen.clone(), true);
        } else if piece == ChessPiece::WPawn {
            if enp_move_sqr != move_.to {
                return (self.fen.clone(), false);
            }
            board_pieces[move_.to as usize] = piece;
            board_pieces[move_.from as usize] = ChessPiece::None;
            board_pieces[enpassant_sqr as usize] = ChessPiece::None;

            let fen = board_to_fen(&board_pieces);
            let rules_part = rules_part.replace(enpassant_part, "-");
            let fen = format!("{} {}", fen, rules_part);

            let check = in_check(&board_pieces, piece.color());
            if check {
                return (self.fen.clone(), false);
            }
            return (fen.clone(), true);
        }
        return (self.fen.clone(), false);
    }

    fn pawn_move(&self, move_: &Move) -> (String, bool) {
        let mut board_pieces: [ChessPiece; 64] = fen_to_board(&self.fen.clone());
        let mut rules_part = self.fen.split(" ").collect::<Vec<&str>>()[1..].to_vec();

        let piece = board_pieces[move_.from as usize];

        let posible_moves =
            super::path::pawn_possible_squares(&board_pieces, piece, move_.from, &self.fen.clone());
        if !posible_moves.contains(&move_.to) {
            return (self.fen.clone(), false);
        }
        let enp_squares = enpassant_moves(move_.from, piece, &self.fen.clone());
        if enp_squares.contains(&move_.to) {
            return self.do_en_passant(move_);
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
            // if promotion no ""
            if move_.promote != "" {
                // get correct piece and color
                let promote = match move_.promote.as_str() {
                    "q" => {
                        if piece.color() == Color::White {
                            ChessPiece::WQueen
                        } else {
                            ChessPiece::BQueen
                        }
                    }
                    "r" => {
                        if piece.color() == Color::White {
                            ChessPiece::WRook
                        } else {
                            ChessPiece::BRook
                        }
                    }
                    "b" => {
                        if piece.color() == Color::White {
                            ChessPiece::WBishop
                        } else {
                            ChessPiece::BBishop
                        }
                    }
                    "n" => {
                        if piece.color() == Color::White {
                            ChessPiece::WKnight
                        } else {
                            ChessPiece::BKnight
                        }
                    }
                    _ => ChessPiece::None,
                };
                // if piece is not retuern false
                if promote == ChessPiece::None {
                    return (self.fen.clone(), false);
                }
                board_pieces[move_.to as usize] = promote;
            }
            let mut fen = board_to_fen(&board_pieces);
            fen = format!("{} {}", fen, rules_part.join(" "));
            let check = in_check(&board_pieces, piece.color());
            if check {
                return (self.fen.clone(), false);
            }
            return (fen.clone(), true);
        }

        return (self.fen.clone(), false);
    }

    fn bishop_move(&self, move_: &Move) -> (String, bool) {
        let mut board_pieces: [ChessPiece; 64] = fen_to_board(&self.fen.clone());
        let mut rules_part = self.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
        let enpassant_part = self.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
        let piece = board_pieces[move_.from as usize];
        let b_possible_moves: (Vec<i32>, Vec<i32>) =
            bishop_possible_squares(&board_pieces, piece, move_.from);
        let mut possible_moves = b_possible_moves.0;
        possible_moves.extend(b_possible_moves.1);
        if possible_moves.contains(&move_.to) {
            board_pieces[move_.to as usize] = piece;
            let check = in_check(&board_pieces, piece.color());
            if check {
                return (self.fen.clone(), false);
            }
            board_pieces[move_.from as usize] = ChessPiece::None;
            let mut fen = board_to_fen(&board_pieces);
            rules_part = rules_part.replace(enpassant_part, "-");
            fen = format!("{} {}", fen, rules_part);

            return (fen.clone(), true);
        }
        return (self.fen.clone(), false);
    }

    fn knight_move(&self, move_: &Move) -> (String, bool) {
        let mut board_pieces: [ChessPiece; 64] = fen_to_board(&self.fen.clone());
        let rules_part = self.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
        let enpassant_part = self.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
                                                                            // check if knight is in index from
        let piece = board_pieces[move_.from as usize];

        let n_possible_moves = knight_possible_squares(&board_pieces, piece, move_.from);
        let mut possible_moves = n_possible_moves.0;
        possible_moves.extend(n_possible_moves.1);
        if possible_moves.contains(&move_.to) {
            board_pieces[move_.to as usize] = piece;
            board_pieces[move_.from as usize] = ChessPiece::None;
            let fen = board_to_fen(&board_pieces);

            let rules_part = rules_part.replace(enpassant_part, "-");
            let fen = format!("{} {}", fen, rules_part);
            let check = in_check(&board_pieces, piece.color());
            if check {
                return (self.fen.clone(), false);
            }
            return (fen.clone(), true);
        }

        return (self.fen.clone(), false);
    }

    fn rook_move(&self, move_: &Move) -> (String, bool) {
        let mut board_pieces: [ChessPiece; 64] = fen_to_board(&self.fen.clone());
        // check if rook is in index from
        let rules_part = self.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
        let enpassant_part = self.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
        let piece = board_pieces[move_.from as usize];
        let castling_rights = self.fen.split(" ").collect::<Vec<&str>>()[2];
        let r_possible_moves = rook_possible_squares(&board_pieces, piece, move_.from);
        let mut possible_moves = r_possible_moves.0;
        possible_moves.extend(r_possible_moves.1);
        if possible_moves.contains(&move_.to) {
            board_pieces[move_.to as usize] = piece;
            board_pieces[move_.from as usize] = ChessPiece::None;
            // replace enpassant part with '-'
            let mut rules_part = rules_part.replace(enpassant_part, "-");
            let fen = board_to_fen(&board_pieces);

            let check = in_check(&board_pieces, piece.color());
            if check {
                return (self.fen.clone(), false);
            }

            // check if castling rights are affected
            if piece == ChessPiece::BRook || piece == ChessPiece::WRook {
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
                rules_part = rules_part.replace(castling_rights, &rights);
            }
            let fen = format!("{} {}", fen, rules_part);

            return (fen.clone(), true);
        }
        return (self.fen.clone(), false);
    }

    fn queen_move(&self, move_: &Move) -> (String, bool) {
        // check miving like a bishop or rook

        let like_rook = move_.from % 8 == move_.to % 8 || move_.from / 8 == move_.to / 8;
        if like_rook {
            let as_rook = self.rook_move(move_);
            if as_rook.1 {
                return as_rook;
            }
        }
        let as_bishop = self.bishop_move(move_);
        if as_bishop.1 {
            return as_bishop;
        }

        return (self.fen.clone(), false);
    }

    fn castle_move(&self, move_: &Move) -> (String, bool) {
        let mut board_pieces: [ChessPiece; 64] = fen_to_board(&self.fen.clone());

        // Determine the direction of castling
        let is_kingside_castle = move_.to > move_.from;

        // Get the position of the rook to castle with
        let (rook_from, rook_to) = if is_kingside_castle {
            (move_.to + 1, move_.to - 1) // Adjust for kingside castling
        } else {
            (move_.to - 2, move_.to + 1) // Adjust for queenside castling
        };

        // Check if rook is in the correct position
        let rook = board_pieces[rook_from as usize];
        let rules_part = self.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
        let enpassant_part = self.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
        let piece = board_pieces[move_.from as usize];
        let original_rights = self.fen.split(" ").collect::<Vec<&str>>()[2];

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
            return (self.fen.clone(), false);
        }

        // Check if all squares between the king and rook are empty
        for i in
            (rook_from + 1..move_.from).step_by(if is_kingside_castle { 1 } else { usize::MAX })
        {
            if board_pieces[i as usize] != ChessPiece::None {
                return (self.fen.clone(), false);
            }
        }

        // cannot castle out of check
        let is_cheked = in_check(&board_pieces, piece.color());
        if is_cheked {
            return (self.fen.clone(), false);
        }

        // Update the board after castling
        board_pieces[move_.from as usize] = ChessPiece::None;
        board_pieces[rook_from as usize] = ChessPiece::None;
        board_pieces[move_.to as usize] = piece;
        board_pieces[rook_to as usize] = rook;
        let is_cheked = in_check(&board_pieces, piece.color());
        if is_cheked {
            return (self.fen.clone(), false);
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
        if rights == "" {
            rights = String::from("-");
        }

        let rules_part = rules_part.replace(enpassant_part, "-");
        let rules_part = rules_part.replace(original_rights, &rights);
        fen = format!("{} {}", fen, rules_part);

        (fen.clone(), true)
    }

    fn king_move(&self, move_: &Move) -> (String, bool) {
        let mut board_pieces: [ChessPiece; 64] = fen_to_board(&self.fen.clone());
        let rules_part = self.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
        let enpassant_part = self.fen.split(" ").collect::<Vec<&str>>()[3]; // e6 for example
        let castling_rights = self.fen.split(" ").collect::<Vec<&str>>()[2];
        // check if king is in index from
        let piece = board_pieces[move_.from as usize];

        let possible_sqrs =
            king_possible_squares(&board_pieces, piece, move_.from, castling_rights);
        if !possible_sqrs.contains(&move_.to) {
            return (self.fen.clone(), false);
        }
        let col_diff = (move_.to % 8) as i32 - (move_.from % 8) as i32;
        if col_diff.abs() > 1 {
            return self.castle_move(move_);
        }
        if possible_sqrs.contains(&move_.to) {
            // target square does not contain a piece of the same color
            let move_sqr = board_pieces[move_.to as usize];
            if move_sqr != ChessPiece::None {
                if board_pieces[move_.to as usize].color() == piece.color() {
                    return (self.fen.clone(), false);
                }
            }
            board_pieces[move_.to as usize] = piece;
            board_pieces[move_.from as usize] = ChessPiece::None;
            let original_rights = self.fen.split(" ").collect::<Vec<&str>>()[2];
            let is_check = in_check(&board_pieces, piece.color());
            if is_check {
                return (self.fen.clone(), false);
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
            let rules_part = rules_part.replace(enpassant_part, "-");
            if rights == "" {
                rights = String::from("-");
            }
            let rules_part = rules_part.replace(original_rights, &rights);

            let fen = format!("{} {}", fen, rules_part);
            return (fen.clone(), true);
        }
        return (self.fen.clone(), false);
    }

    pub fn make_move(&mut self, move_: &Move) -> (bool, bool) {
        let board_pieces: [ChessPiece; 64] = fen_to_board(&self.fen.clone());
        let turn = self.fen.split(" ").collect::<Vec<&str>>()[1];
        let color = match turn {
            "w" => Color::White,
            "b" => Color::Black,
            _ => Color::White,
        };
        if color != move_.piece.color() {
            return (false, false);
        }
        if move_.from == move_.to {
            return (false, false);
        }

        let piece = board_pieces[move_.from as usize];
        // choose correct move function
        let (fen, moved) = match piece {
            ChessPiece::BPawn | ChessPiece::WPawn => self.pawn_move(move_),
            ChessPiece::BBishop | ChessPiece::WBishop => self.bishop_move(move_),
            ChessPiece::BKnight | ChessPiece::WKnight => self.knight_move(move_),
            ChessPiece::BRook | ChessPiece::WRook => self.rook_move(move_),
            ChessPiece::BQueen | ChessPiece::WQueen => self.queen_move(move_),
            ChessPiece::BKing | ChessPiece::WKing => self.king_move(move_),
            ChessPiece::None => {
                return (false, false);
            }
        };

        if !moved {
            return (false, false);
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
        let fen = format!("{} {}", fen_part, rules_part);

        // see if oponemt is checked
        let opp_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        let board = fen_to_board(&fen);

        // get black pieces remaining
        let black_rem_pieces = board
            .iter()
            .filter(|&x| *x != ChessPiece::None && x.color() == Color::Black)
            .collect::<Vec<&ChessPiece>>();

        // get white pieces remaining
        let white_rem_pieces: Vec<&ChessPiece> = board
            .iter()
            .filter(|&x| *x != ChessPiece::None && x.color() == Color::White)
            .collect::<Vec<&ChessPiece>>();

        if black_rem_pieces.len() == 1 && white_rem_pieces.len() == 1 {
            return (false, false);
        }
        if black_rem_pieces.len() < 3 && white_rem_pieces.len() < 3 {
            let is_draw = is_insufficient_material(black_rem_pieces, white_rem_pieces);
            if is_draw {
                return (false, false);
            }
        }

        let is_check = in_check(&fen_to_board(&fen), opp_color);

        let mut opp_king_pos = 0;
        for (i, piece) in board_pieces.iter().enumerate() {
            if *piece == ChessPiece::WKing {
                opp_king_pos = i;
                break;
            }
            if *piece == ChessPiece::BKing {
                opp_king_pos = i;
                break;
            }
        }
        let opp_king_pos = opp_king_pos as i32;
        let king = board[opp_king_pos as usize];
        let king_pos_moved = king_possible_squares(
            &fen_to_board(&fen),
            king,
            opp_king_pos,
            rules_part.split(" ").collect::<Vec<&str>>()[2],
        );

        let state = if king_pos_moved.len() == 0 && is_check {
            GameState::Checkmate
        } else if king_pos_moved.len() == 0 && !is_check {
            GameState::Stalemate
        } else {
            GameState::Normal
        };
        // replace color part with '-'
        let mut rules_part = rules_part.split(" ").collect::<Vec<&str>>().to_vec();
        if state == GameState::Checkmate {
            rules_part[0] = "-";
        }
        // index 0 is color replace with '-'
        let rules_part = rules_part.join(" ");
        // let fen_part = fen_part.replace("w", "-");
        let fen = format!("{} {}", fen_part, rules_part);
        self.fen = fen;
        self.move_notations.push(name_moves(&self.fen, move_));
        if state == GameState::Checkmate || state == GameState::Stalemate {
            self.game_over = true;
            self.winner = if state == GameState::Checkmate {
                Some(color)
            } else {
                None
            };
        }
        return (true, is_check);
    }
    pub fn unmake_move(&mut self, move_: &Move) -> bool {
        let is_en_passant = move_.move_type == MoveType::EnPassant;
        let is_castle = move_.move_type == MoveType::Castle;
        // println!(
        //     "reversing move to: {} from: {} fen: {}",
        //     move_.to, move_.from, self.fen
        // );
        let mut board_pieces: [ChessPiece; 64] = fen_to_board(&self.fen.clone());
        let rules_part = self.fen.split(" ").collect::<Vec<&str>>()[1..].join(" ");
        let fen_part = self.fen.split(" ").collect::<Vec<&str>>()[0];

        let moving_piece = board_pieces[move_.to as usize];

        let color = moving_piece.color();
        let opp_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        if is_castle {
            let is_kingside_castle = move_.to > move_.from;
            let (rook_from, rook_to) = if is_kingside_castle {
                (move_.to + 1, move_.to - 1) // Adjust for kingside castling
            } else {
                (move_.to - 2, move_.to + 1) // Adjust for queenside castling
            };
            board_pieces[move_.from as usize] = moving_piece;
            board_pieces[move_.to as usize] = ChessPiece::None;
            board_pieces[rook_from as usize] = board_pieces[rook_to as usize];
            board_pieces[rook_to as usize] = ChessPiece::None;
        }
        // if enpassant occured return pawn to original position
        if is_en_passant {
            let capture_pawn_sqr = if moving_piece == ChessPiece::BPawn {
                move_.to + 8
            } else {
                move_.to - 8
            };
            board_pieces[move_.from as usize] = moving_piece;
            board_pieces[move_.to as usize] = ChessPiece::None;
            board_pieces[capture_pawn_sqr as usize] = if moving_piece == ChessPiece::BPawn {
                ChessPiece::WPawn
            } else {
                ChessPiece::BPawn
            };
        }

        // if promotion occured return pawn to original position
        if move_.move_type == MoveType::Promotion {
            board_pieces[move_.from as usize] = moving_piece;
            board_pieces[move_.to as usize] = move_.captured_piece;
        }

        // if normal move return piece to original position
        if move_.move_type == MoveType::Normal {
            board_pieces[move_.from as usize] = moving_piece;
            board_pieces[move_.to as usize] = move_.captured_piece;
        }

        //update fen (if enpassant, castle, promotion)
        let rules_part = rules_part.split(" ").collect::<Vec<&str>>();
        let mut rules_part = rules_part.to_vec();
        let mut enpassant_part = String::from("-");
        if is_en_passant {
            // reconstruct enpassant part
            enpassant_part = if moving_piece == ChessPiece::BPawn {
                let file = (move_.to % 8) as i32 + 97;
                let rank = 8 - (move_.to / 8) as i32;
                format!("{}{}", file as u8 as char, rank)
            } else {
                let file = (move_.to % 8) as i32 + 97;
                let rank = 8 - (move_.to / 8) as i32;
                format!("{}{}", file as u8 as char, rank)
            };
        }

        rules_part[2] = &enpassant_part;

        rules_part[0] = match color {
            Color::White => "w",
            Color::Black => "b",
        };

        // update castling rights
        rules_part[1] = move_.castling_rights.as_str();
        // reverse move count
        let mut fifty_move_count = rules_part[4].parse::<i32>().unwrap();
        let mut move_count = rules_part[4].parse::<i32>().unwrap();
        // if pawn move or capture move or promotion move update 50 move count
        if moving_piece == ChessPiece::BPawn
            || moving_piece == ChessPiece::WPawn
            || move_.captured_piece != ChessPiece::None
            || move_.move_type == MoveType::Promotion
        {
            fifty_move_count -= 1;
        }

        if color == Color::Black {
            fifty_move_count -= 1;
        }

        let fifty_move_count = fifty_move_count.to_string();
        rules_part[3] = &fifty_move_count;
        move_count -= 1;

        let move_count = move_count.to_string();
        rules_part[4] = &move_count;

        let rules_part = rules_part.join(" ");
        let fen = format!("{} {}", fen_part, rules_part);

        // update game state
        self.fen = fen;
        self.move_notations.pop();
        self.game_over = false;
        self.winner = None;
        self.turn = opp_color;
        true
    }
}

pub fn name_moves(fen: &str, move_: &Move) -> String {
    let board = fen_to_board(&fen);
    let from_sqr;
    let from_file;
    from_file = format!("{}", ((move_.from % 8) as i32 + 97) as u8 as char);
    let from_rank = 8 - (move_.from / 8) as i32;

    let to_file = (move_.to % 8) as i32 + 97;
    let to_rank = 8 - (move_.to / 8) as i32;

    if board[move_.to as usize] != ChessPiece::None {
        from_sqr = format!("{}{}", from_file, from_rank);
    } else {
        from_sqr = format!("{}{}", from_file, from_rank);
    }

    let move_note = format!("{}{}{}", from_sqr, to_file as u8 as char, to_rank,);

    move_note
}
