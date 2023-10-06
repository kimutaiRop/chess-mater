use crate::{
    actions::{
        path::PossibleMoves,
        play::{Game, MoveType},
    },
    interface::chessboard::piece::{fen_to_board, ChessPiece, Color as PieceColor},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ChessData {
    depth: i32,
    nodes: i32,
    fen: String,
}

use super::{
    capture::get_pieces_by_color,
    heatmap::{piece_heat_map, GameType},
    play::Move,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Engine {
    pub color: PieceColor,
}

impl Engine {
    pub fn generate_best_move(game: &mut Game, color: PieceColor) -> Move {
        let (valuation, best_move) =
            alpha_beta_search(game, 3, std::f64::NEG_INFINITY, std::f64::INFINITY, color);
        println!("best move: {:?}", best_move);
        let castling_part = game.fen.split(" ").collect::<Vec<&str>>()[2];
        if best_move.is_none() {
            return Move {
                from: 0,
                to: 0,
                piece: ChessPiece::None,
                promote: "".into(),
                move_type: MoveType::Normal,
                captured_piece: ChessPiece::None,
                castling_rights: castling_part.to_string(),
            };
        }

        let best_move = best_move.unwrap();
        let best_move = Move {
            from: best_move.from,
            to: best_move.to,
            piece: best_move.piece,
            promote: "".into(),
            move_type: MoveType::Normal,
            captured_piece: ChessPiece::None,
            castling_rights: castling_part.to_string(),
        };
        best_move
    }
}

pub fn evaluate_position(game: &Game, color: PieceColor) -> f64 {
    let board: [ChessPiece; 64] = fen_to_board(&game.fen);

    let my_pieces: Vec<(ChessPiece, i32)> = get_pieces_by_color(&board, color);
    let opponent_color = if color == PieceColor::White {
        PieceColor::Black
    } else {
        PieceColor::White
    };

    let opponent_pieces: Vec<(ChessPiece, i32)> = get_pieces_by_color(&board, opponent_color);

    let game_stage = if my_pieces.len() + opponent_pieces.len() < 10 {
        GameType::End
    } else if my_pieces.len() + opponent_pieces.len() < 20 {
        GameType::Middle
    } else {
        GameType::Opening
    };
    let mut valuation = 0;
    for (piece, pos) in &my_pieces {
        let heat_map: [[i32; 8]; 8] = piece_heat_map(*piece, color, &game_stage);

        let row = pos / 8;
        let col = pos % 8;
        valuation += heat_map[row as usize][col as usize];
    }

    for (piece, pos) in &opponent_pieces {
        let heat_map: [[i32; 8]; 8] = piece_heat_map(*piece, opponent_color, &game_stage);

        let row = pos / 8;
        let col = pos % 8;
        if *piece != ChessPiece::None {
            valuation -= heat_map[row as usize][col as usize];
        }
    }
    let opp_possible_moves = game.possible_moves(opponent_color);

    for mv in opp_possible_moves {
        let piece = board[mv.to as usize];
        let capture_penalty = match piece {
            ChessPiece::WKing | ChessPiece::BKing => 10000,
            ChessPiece::WQueen | ChessPiece::BQueen => 200,
            ChessPiece::WRook | ChessPiece::BRook => 180,
            ChessPiece::WBishop | ChessPiece::BBishop => 150,
            ChessPiece::WKnight | ChessPiece::BKnight => 150,
            _ => 80,
        };
        if mv.captured_piece != ChessPiece::None {
            let piece = board[mv.to as usize];
            if piece != ChessPiece::None {
                valuation -= piece.piece_value() * capture_penalty;
            }
        }
    }

    let board_table: [[ChessPiece; 8]; 8] = [[ChessPiece::None; 8]; 8];

    let mut rook_bonus = 0;
    if game_stage == GameType::Middle {
        for (piece, pos) in &my_pieces {
            if *piece == ChessPiece::WRook || *piece == ChessPiece::BRook {
                let col = pos % 8;

                let mut open_file = true;
                for i in 0..8 {
                    if board_table[i][col as usize] != ChessPiece::None {
                        open_file = false;
                        break;
                    }
                }

                // Check if the square is occupied by an opponent piece
                let square_occupied_by_opponent = opponent_pieces
                    .iter()
                    .any(|(_, opponent_pos)| *opponent_pos == *pos);

                if open_file {
                    rook_bonus += 20;
                }
                if square_occupied_by_opponent {
                    rook_bonus += 10;
                }
            }
        }
    }

    // Add rook bonuses to the overall valuation
    valuation += rook_bonus;

    let my_possible_moves = game.possible_moves(color);
    for mv in my_possible_moves {
        let piece = board[mv.to as usize];
        let capture_penalty = match piece {
            ChessPiece::WKing | ChessPiece::BKing => 10000,
            ChessPiece::WQueen | ChessPiece::BQueen => 200,
            ChessPiece::WRook | ChessPiece::BRook => 180,
            ChessPiece::WBishop | ChessPiece::BBishop => 150,
            ChessPiece::WKnight | ChessPiece::BKnight => 150,
            _ => 80,
        };
        if mv.captured_piece != ChessPiece::None {
            let piece = board[mv.to as usize];
            if piece != ChessPiece::None {
                valuation += piece.piece_value() * capture_penalty;
            }
        }
    }
    println!("valuation: {}", valuation);
    valuation as f64
}

fn evaluate_and_search(
    game: &mut Game,
    depth: i32,
    mut alpha: f64,
    mut beta: f64,
    color: PieceColor,
    mv: PossibleMoves,
) -> (f64, Option<PossibleMoves>) {
    let mut best_value = if color == PieceColor::White {
        std::f64::NEG_INFINITY
    } else {
        std::f64::INFINITY
    };

    let mut best_move = None; // Keep track of the best move.
    let castling_part = game.fen.split(" ").collect::<Vec<&str>>()[2];
    if !mv.promote {
        let move_ = Move {
            from: mv.from,
            to: mv.to,
            piece: mv.piece,
            promote: "".into(),
            move_type: MoveType::Normal,
            captured_piece: mv.captured_piece,
            castling_rights: castling_part.to_string(),
        };
        let move_res = game.make_move(&move_);

        if !move_res {
            // If the move puts the opponent in check, we want to search deeper
            let (value, new_best_move) = alpha_beta_search(
                game,
                depth,
                alpha.clone(),
                beta.clone(),
                if color == PieceColor::White {
                    PieceColor::Black
                } else {
                    PieceColor::White
                },
            );
            let value = value as f64;
            if color == PieceColor::White && value > best_value {
                best_value = value;
                best_move = new_best_move;
                alpha = alpha.max(best_value);
            } else if color == PieceColor::Black && value < best_value {
                best_value = value;
                best_move = new_best_move;
                beta = beta.min(best_value);
            }
        }

        let (value, _) = alpha_beta_search(
            game,
            depth - 1,
            alpha.clone(),
            beta.clone(),
            if color == PieceColor::White {
                PieceColor::Black
            } else {
                PieceColor::White
            },
        );
        let value = value as f64;

        if color == PieceColor::White && value > best_value {
            best_value = value;
            best_move = Some(mv);
            alpha = alpha.max(best_value);
        } else if color == PieceColor::Black && value < best_value {
            best_value = value;
            best_move = Some(mv);
            beta = beta.min(best_value);
        }

        if beta <= alpha {
            return (best_value, best_move); // Beta cutoff
        }
    } else {
        // Handle promotion by considering all four promotion pieces (queen, rook, knight, bishop)
        let promotion_pieces = ["q", "r", "n", "b"];
        let castling_part = game.fen.split(" ").collect::<Vec<&str>>()[2];
        for promote_piece in &promotion_pieces {
            let mv = mv.clone();
            let promoted_move = Move {
                from: mv.from,
                to: mv.to,
                piece: mv.piece,
                promote: promote_piece.to_string(),
                move_type: MoveType::Promotion,
                captured_piece: mv.captured_piece,
                castling_rights: castling_part.to_string(),
            };

            let game = &mut game.clone();

            let (value, new_best_move) = alpha_beta_search(
                game,
                depth - 1,
                alpha.clone(),
                beta.clone(),
                if color == PieceColor::White {
                    PieceColor::Black
                } else {
                    PieceColor::White
                },
            );
            let value = value as f64;
            if color == PieceColor::White && value > best_value {
                best_value = value;
                best_move = new_best_move;
                alpha = alpha.max(best_value);
            } else if color == PieceColor::Black && value < best_value {
                best_value = value;
                best_move = new_best_move;
                beta = beta.min(best_value);
            }

            if beta <= alpha {
                break; // Beta cutoff
            }
        }
    }

    (best_value, best_move)
}

pub fn alpha_beta_search(
    game: &mut Game,
    depth: i32,
    mut alpha: f64,
    mut beta: f64,
    color: PieceColor,
) -> (f64, Option<PossibleMoves>) {
    if depth == 0 {
        return (evaluate_position(game, color), None);
    }

    let mut best_value = if color == PieceColor::White {
        -f64::INFINITY
    } else {
        f64::INFINITY
    };

    let mut best_move = None; // Keep track of the best move.

    let moves = game.possible_moves(color);

    for mv in moves {
        let (value, new_best_move) = evaluate_and_search(game, depth, alpha, beta, color, mv);
        let value = value as f64;
        if color == PieceColor::White && value > best_value {
            best_value = value;
            best_move = new_best_move;
            alpha = alpha.max(best_value);
        } else if color == PieceColor::Black && value < best_value {
            best_value = value;
            best_move = new_best_move;
            beta = beta.min(best_value);
        }

        if beta <= alpha {
            break; // Beta cutoff
        }
    }
    (best_value, best_move)
}
