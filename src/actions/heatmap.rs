use crate::interface::chessboard::piece::{ChessPiece, Color as PieceColor};

#[derive(PartialEq)]
pub enum GameType {
    Opening,
    Middle,
    End,
}

pub fn piece_heat_map(piece: ChessPiece, color: PieceColor, game_type: &GameType) -> [[i32; 8]; 8] {
    let pawn_opening: [[i32; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [55, 60, 60, 20, 20, 60, 40, 55],
        [20, 20, 20, 70, 70, 20, 20, 60],
        [0, 0, 0, 80, 80, 0, 0, 0],
        [-5, -5, -5, 50, 50, 30, -5, -5],
        [-10, -10, -10, 10, 10, -20, 1 - 0, -10],
        [5, 5, 10, 20, 20, 10, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let bishop_opening: [[i32; 8]; 8] = [
        [70, 70, 70, 70, 70, 70, 70, 70],
        [70, 70, 70, 70, 70, 70, 70, 70],
        [70, 70, 90, 90, 90, 90, 70, 70],
        [70, 90, 90, 100, 100, 90, 90, 70],
        [70, 90, 100, 110, 110, 100, 90, 70],
        [70, 90, 90, 100, 100, 90, 90, 70],
        [70, 70, 70, 70, 70, 70, 70, 70],
        [70, 70, 70, 70, 70, 70, 70, 70],
    ];

    let queen_opening: [[i32; 8]; 8] = [
        [10, 10, 10, 10, 10, 10, 10, 10],
        [10, 20, 20, 20, 20, 20, 20, 10],
        [10, 20, 30, 30, 30, 30, 20, 10],
        [10, 20, 30, 40, 40, 30, 20, 10],
        [10, 20, 30, 40, 40, 30, 20, 10],
        [10, 20, 30, 30, 30, 30, 20, 10],
        [10, 20, 20, 20, 20, 20, 20, 10],
        [10, 10, 10, 10, 10, 10, 10, 10],
    ];

    let rook_opening: [[i32; 8]; 8] = [
        [0, 0, 0, 10, 10, 0, 0, 0],
        [0, 0, 0, 10, 10, 0, 0, 0],
        [0, 0, 0, 10, 10, 0, 0, 0],
        [0, 0, 0, 10, 10, 0, 0, 0],
        [0, 0, 0, 10, 10, 0, 0, 0],
        [0, 0, 0, 10, 10, 0, 0, 0],
        [20, 20, 20, 20, 20, 20, 20, 20],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let knight_opening: [[i32; 8]; 8] = [
        [-50, -40, -30, -30, -30, -30, -40, -50],
        [-40, -20, 10, 10, 10, 10, -20, -40],
        [-30, 20, 25, 20, 20, 25, 20, -30],
        [-30, 0, 5, 10, 10, 5, 0, -30],
        [-30, 0, 5, 5, 5, 5, 0, -30],
        [-30, 5, -10, -10, -10, 10, 5, -30],
        [-40, -20, -10, -15, -15, -10, -20, -40],
        [-50, -40, -30, -30, -30, -30, -40, -50],
    ];

    let king_opening: [[i32; 8]; 8] = [
        [20, 30, 30, 20, 20, 30, 30, 20],
        [10, 20, 20, 20, 20, 20, 20, 10],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [-10, -10, -10, -10, -10, -10, -10, -10],
        [-20, -20, -20, -20, -20, -20, -20, -20],
        [-30, -30, -30, -30, -30, -30, -30, -30],
        [-40, -40, -40, -40, -40, -40, -40, -40],
        [-50, -50, -50, -50, -50, -50, -50, -50],
    ];

    let pawn_middle: [[i32; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [60, 60, 60, 40, 40, 60, 60, 60],
        [40, 40, 50, 80, 80, 50, 40, 40],
        [20, 20, 80, 80, 80, 30, 20, 20],
        [10, 10, 20, 40, 40, 20, 10, 10],
        [5, 5, 10, 20, 20, 10, 5, 5],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, -10, -10, 0, 0, -10, -10, 0],
    ];

    let bishop_middle: [[i32; 8]; 8] = [
        [70, 70, 70, 70, 70, 70, 70, 70],
        [70, 70, 70, 70, 70, 70, 70, 70],
        [70, 70, 90, 90, 90, 90, 70, 70],
        [70, 90, 90, 100, 100, 90, 90, 70],
        [70, 90, 100, 110, 110, 100, 90, 70],
        [70, 90, 90, 100, 100, 90, 90, 70],
        [70, 70, 70, 70, 70, 70, 70, 70],
        [70, 70, 70, 70, 70, 70, 70, 70],
    ];

    let queen_middle: [[i32; 8]; 8] = [
        [10, 10, 10, 10, 10, 10, 10, 10],
        [10, 20, 20, 20, 20, 20, 20, 10],
        [10, 20, 30, 30, 30, 30, 20, 10],
        [10, 20, 30, 40, 40, 30, 20, 10],
        [10, 20, 30, 40, 40, 30, 20, 10],
        [10, 20, 30, 30, 30, 30, 20, 10],
        [10, 20, 20, 20, 20, 20, 20, 10],
        [10, 10, 10, 10, 10, 10, 10, 10],
    ];

    let rook_middle: [[i32; 8]; 8] = [
        [0, 0, 0, 10, 10, 0, 0, 0],
        [0, 0, 0, 10, 10, 0, 0, 0],
        [0, 0, 0, 10, 10, 0, 0, 0],
        [0, 0, 0, 10, 10, 0, 0, 0],
        [0, 0, 0, 10, 10, 0, 0, 0],
        [0, 0, 0, 10, 10, 0, 0, 0],
        [20, 20, 20, 20, 20, 20, 20, 20],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let knight_middle: [[i32; 8]; 8] = [
        [-50, -40, -30, -30, -30, -30, -40, -50],
        [-40, -20, 0, 0, 0, 0, -20, -40],
        [-30, 0, 10, 15, 15, 10, 0, -30],
        [-30, 5, 15, 20, 20, 15, 5, -30],
        [-30, 0, 15, 20, 20, 15, 0, -30],
        [-30, 5, 10, 15, 15, 10, 5, -30],
        [-40, -20, 0, 5, 5, 0, -20, -40],
        [-50, -40, -30, -30, -30, -30, -40, -50],
    ];

    let king_middle: [[i32; 8]; 8] = [
        [50, 50, 40, 20, 20, 40, 50, 50],
        [30, 30, 0, 0, 0, 0, 30, 30],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let pawn_endgame: [[i32; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [5, 5, 5, 0, 0, 5, 5, 5],
        [10, 10, 10, 0, 0, 10, 10, 10],
        [20, 20, 20, 30, 30, 20, 20, 20],
        [30, 40, 50, 50, 50, 50, 40, 30],
        [40, 50, 60, 60, 60, 60, 20, 40],
        [50, 50, 70, 70, 70, 70, 60, 50],
        [100, 100, 100, 100, 100, 100, 100, 100],
    ];

    let bishop_endgame: [[i32; 8]; 8] = [
        [70, 70, 70, 70, 70, 70, 70, 70],
        [70, 70, 70, 70, 70, 70, 70, 70],
        [70, 70, 90, 90, 90, 90, 70, 70],
        [70, 90, 90, 100, 100, 90, 90, 70],
        [70, 90, 100, 110, 110, 100, 90, 70],
        [70, 90, 90, 100, 100, 90, 90, 70],
        [70, 70, 70, 70, 70, 70, 70, 70],
        [70, 70, 70, 70, 70, 70, 70, 70],
    ];

    let queen_endgame: [[i32; 8]; 8] = [
        [10, 10, 10, 10, 10, 10, 10, 10],
        [10, 20, 20, 20, 20, 20, 20, 10],
        [10, 20, 30, 30, 30, 30, 20, 10],
        [10, 20, 30, 40, 40, 30, 20, 10],
        [10, 20, 30, 40, 40, 30, 20, 10],
        [10, 20, 30, 30, 30, 30, 20, 10],
        [10, 20, 20, 20, 20, 20, 20, 10],
        [10, 10, 10, 10, 10, 10, 10, 10],
    ];

    let rook_endgame: [[i32; 8]; 8] = [
        [-5, 5, -10, 20, 20, 10, 0, -5],
        [5, 10, 15, 20, 20, 15, 10, 5],
        [0, 5, 15, 20, 20, 15, 0, 0],
        [0, 0, 15, 20, 20, 15, 0, 0],
        [0, 0, 15, 20, 20, 15, 0, 0],
        [0, 0, 15, 20, 20, 15, 0, 0],
        [70, 70, 70, 70, 70, 70, 70, 70],
        [0, 02, 20, 20, 20, 20, 20, 0],
    ];

    let knight_endgame: [[i32; 8]; 8] = [
        [-50, -40, -30, -30, -30, -30, -40, -50],
        [-40, -20, 0, 0, 0, 0, -20, -40],
        [-30, 0, 10, 15, 15, 10, 0, -30],
        [-30, 5, 15, 20, 20, 15, 5, -30],
        [-30, 0, 15, 20, 20, 15, 0, -30],
        [-30, 5, 10, 15, 15, 10, 5, -30],
        [-40, -20, 0, 5, 5, 0, -20, -40],
        [-50, -40, -30, -30, -30, -30, -40, -50],
    ];

    let king_endgame: [[i32; 8]; 8] = [
        [-50, -30, -30, -30, -30, -30, -30, -50],
        [-30, -30, 0, 0, 0, 0, -30, -30],
        [-30, -10, 20, 30, 30, 20, -10, -30],
        [-30, -10, 30, 40, 40, 30, -10, -30],
        [-30, -10, 30, 40, 40, 30, -10, -30],
        [-30, -10, 20, 30, 30, 20, 10, -30],
        [-30, 10, 10, 0, 0, -10, 10, 10],
        [-50, -40, -30, -20, -20, -30, -40, -50],
    ];

    // this shows heatap for back for white they are the same for black but flipped

    let correct_healt = match color {
        PieceColor::White => match piece {
            ChessPiece::WPawn => {
                let mut reversed = [[0; 8]; 8];
                for i in 0..8 {
                    for j in 0..8 {
                        if game_type == &GameType::Opening {
                            reversed[i][j] = pawn_opening[7 - i][j];
                        } else if game_type == &GameType::Middle {
                            reversed[i][j] = pawn_middle[7 - i][j];
                        } else {
                            reversed[i][j] = pawn_endgame[7 - i][j];
                        }
                    }
                }
                reversed
            }
            ChessPiece::WBishop => {
                let mut reversed = [[0; 8]; 8];
                for i in 0..8 {
                    for j in 0..8 {
                        if game_type == &GameType::Opening {
                            reversed[i][j] = bishop_opening[7 - i][j];
                        } else if game_type == &GameType::Middle {
                            reversed[i][j] = bishop_middle[7 - i][j];
                        } else {
                            reversed[i][j] = bishop_endgame[7 - i][j];
                        }
                    }
                }
                reversed
            }
            ChessPiece::WQueen => {
                let mut reversed = [[0; 8]; 8];
                for i in 0..8 {
                    for j in 0..8 {
                        if game_type == &GameType::Opening {
                            reversed[i][j] = queen_opening[7 - i][j];
                        } else if game_type == &GameType::Middle {
                            reversed[i][j] = queen_middle[7 - i][j];
                        } else {
                            reversed[i][j] = queen_endgame[7 - i][j];
                        }
                    }
                }
                reversed
            }
            ChessPiece::WRook => {
                let mut reversed = [[0; 8]; 8];
                for i in 0..8 {
                    for j in 0..8 {
                        if game_type == &GameType::Opening {
                            reversed[i][j] = rook_opening[7 - i][j];
                        } else if game_type == &GameType::Middle {
                            reversed[i][j] = rook_middle[7 - i][j];
                        } else {
                            reversed[i][j] = rook_endgame[7 - i][j];
                        }
                    }
                }
                reversed
            }
            ChessPiece::WKing => {
                let mut reversed = [[0; 8]; 8];
                for i in 0..8 {
                    for j in 0..8 {
                        if game_type == &GameType::Opening {
                            reversed[i][j] = king_opening[7 - i][j];
                        } else if game_type == &GameType::Middle {
                            reversed[i][j] = king_middle[7 - i][j];
                        } else {
                            reversed[i][j] = knight_endgame[7 - i][j];
                        }
                    }
                }
                reversed
            }
            ChessPiece::WKnight => {
                let mut reversed = [[0; 8]; 8];
                for i in 0..8 {
                    for j in 0..8 {
                        if game_type == &GameType::Opening {
                            reversed[i][j] = knight_opening[7 - i][j];
                        } else if game_type == &GameType::Middle {
                            reversed[i][j] = knight_middle[7 - i][j];
                        } else {
                            reversed[i][j] = king_endgame[7 - i][j];
                        }
                    }
                }
                reversed
            }
            _ => [[0; 8]; 8],
        },
        PieceColor::Black => match piece {
            ChessPiece::BPawn => {
                if game_type == &GameType::Opening {
                    pawn_opening
                } else if game_type == &GameType::Middle {
                    pawn_middle
                } else {
                    pawn_endgame
                }
            }
            ChessPiece::BBishop => {
                if game_type == &GameType::Opening {
                    bishop_opening
                } else if game_type == &GameType::Middle {
                    bishop_middle
                } else {
                    bishop_endgame
                }
            }
            ChessPiece::BQueen => {
                if game_type == &GameType::Opening {
                    queen_opening
                } else if game_type == &GameType::Middle {
                    queen_middle
                } else {
                    queen_endgame
                }
            }
            ChessPiece::BRook => {
                if game_type == &GameType::Opening {
                    rook_opening
                } else if game_type == &GameType::Middle {
                    rook_middle
                } else {
                    rook_endgame
                }
            }
            ChessPiece::BKing => {
                if game_type == &GameType::Opening {
                    king_opening
                } else if game_type == &GameType::Middle {
                    king_middle
                } else {
                    king_endgame
                }
            }
            ChessPiece::BKnight => {
                if game_type == &GameType::Opening {
                    knight_opening
                } else if game_type == &GameType::Middle {
                    knight_middle
                } else {
                    knight_endgame
                }
            }
            _ => [[0; 8]; 8],
        },
    };

    correct_healt
}
