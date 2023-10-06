use godot::engine::{Sprite2D, Sprite2DVirtual};
use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Sprite2D)]
pub struct Piece {
    pub piece: ChessPiece,
    #[base]
    pub node: Base<Sprite2D>,
}

#[godot_api]
impl Sprite2DVirtual for Piece {
    fn init(node: Base<Sprite2D>) -> Self {
        Self {
            node,
            piece: ChessPiece::None,
        }
    }
    fn process(&mut self, _delta: f64) {}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ChessPiece {
    BPawn,
    WPawn,
    BKnight,
    WKnight,
    BBishop,
    WBishop,
    BRook,
    WRook,
    BQueen,
    WQueen,
    BKing,
    WKing,
    None,
}

pub fn fen_to_board(fen: &str) -> [ChessPiece; 64] {
    let board_fen = fen.split(' ').collect::<Vec<&str>>()[0];
    let mut board: [ChessPiece; 64] = [ChessPiece::None; 64];
    let mut sqr: usize = 0;

    for c in board_fen.chars() {
        match c {
            '1'..='8' => {
                // fill n squares with empty
                let n = c.to_digit(10).unwrap() as usize;
                for _ in 0..n {
                    board[sqr] = ChessPiece::None;
                    sqr += 1;
                }
            }
            // if c is / then go to next sqr
            '/' => {
                // sqr += 1;
            }
            // if c is a piece then add it to the board
            'p' => {
                board[sqr] = ChessPiece::BPawn;
                sqr += 1;
            }
            'P' => {
                board[sqr] = ChessPiece::WPawn;
                sqr += 1;
            }
            'n' => {
                board[sqr] = ChessPiece::BKnight;
                sqr += 1;
            }
            'N' => {
                board[sqr] = ChessPiece::WKnight;
                sqr += 1;
            }
            'b' => {
                board[sqr] = ChessPiece::BBishop;
                sqr += 1;
            }
            'B' => {
                board[sqr] = ChessPiece::WBishop;
                sqr += 1;
            }
            'r' => {
                board[sqr] = ChessPiece::BRook;
                sqr += 1;
            }
            'R' => {
                board[sqr] = ChessPiece::WRook;
                sqr += 1;
            }
            'q' => {
                board[sqr] = ChessPiece::BQueen;
                sqr += 1;
            }
            'Q' => {
                board[sqr] = ChessPiece::WQueen;
                sqr += 1;
            }
            'k' => {
                board[sqr] = ChessPiece::BKing;
                sqr += 1;
            }
            'K' => {
                board[sqr] = ChessPiece::WKing;
                sqr += 1;
            }
            _ => {}
        }
    }

    board
}

pub fn piece_to_fen(piece: &ChessPiece) -> String {
    match piece {
        ChessPiece::BPawn => "p".to_string(),
        ChessPiece::WPawn => "P".to_string(),
        ChessPiece::BKnight => "n".to_string(),
        ChessPiece::WKnight => "N".to_string(),
        ChessPiece::BBishop => "b".to_string(),
        ChessPiece::WBishop => "B".to_string(),
        ChessPiece::BRook => "r".to_string(),
        ChessPiece::WRook => "R".to_string(),
        ChessPiece::BQueen => "q".to_string(),
        ChessPiece::WQueen => "Q".to_string(),
        ChessPiece::BKing => "k".to_string(),
        ChessPiece::WKing => "K".to_string(),
        ChessPiece::None => "".to_string(),
    }
}

pub fn string_to_piece(piece: &str) -> ChessPiece {
    match piece {
        "p" => ChessPiece::BPawn,
        "P" => ChessPiece::WPawn,
        "n" => ChessPiece::BKnight,
        "N" => ChessPiece::WKnight,
        "b" => ChessPiece::BBishop,
        "B" => ChessPiece::WBishop,
        "r" => ChessPiece::BRook,
        "R" => ChessPiece::WRook,
        "q" => ChessPiece::BQueen,
        "Q" => ChessPiece::WQueen,
        "k" => ChessPiece::BKing,
        "K" => ChessPiece::WKing,
        _ => ChessPiece::None,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

// toggle color
impl Color {
    pub fn toggle(&mut self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl ChessPiece {
    pub fn color(&self) -> Color {
        // uppercase is white and lowercase is black (return)
        let piece_fen = piece_to_fen(self);
        if piece_fen == piece_fen.to_uppercase() {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn piece_value(&self) -> i32 {
        match self {
            ChessPiece::BPawn => 1,
            ChessPiece::WPawn => 1,
            ChessPiece::BKnight => 3,
            ChessPiece::WKnight => 3,
            ChessPiece::BBishop => 3,
            ChessPiece::WBishop => 3,
            ChessPiece::BRook => 5,
            ChessPiece::WRook => 5,
            ChessPiece::BQueen => 9,
            ChessPiece::WQueen => 9,
            ChessPiece::BKing => 100,
            ChessPiece::WKing => 100,
            ChessPiece::None => 0,
        }
    }
}

pub fn board_to_fen(board: &[ChessPiece; 64]) -> String {
    let mut fen = String::new();
    let mut empty = 0;

    for (i, piece) in board.iter().enumerate() {
        if *piece == ChessPiece::None {
            empty += 1;
        } else {
            if empty > 0 {
                fen.push_str(&empty.to_string());
                empty = 0;
            }
            fen.push_str(&piece_to_fen(piece));
        }
        if (i + 1) % 8 == 0 {
            if empty > 0 {
                fen.push_str(&empty.to_string());
                empty = 0;
            }
            if i != 63 {
                fen.push('/');
            }
        }
    }

    fen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_to_board() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string();
        let board = fen_to_board(&fen);
        // Add your test assertions here
        // For example, to check that the first square is a rook, you can do:
        assert_eq!(board[0], ChessPiece::BRook);

        // Add more assertions as needed to validate the board state
    }
}
