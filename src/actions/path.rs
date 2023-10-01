use crate::interface::chessboard::piece::{ChessPiece, Color};

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

    // Combine the directions of rook and bishop to cover all directions
    let directions: [(i32, i32); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    // use both rook and bishop functions
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
) -> Vec<i32> {
    // black ,oves from 1st to 8th rank
    let mut squares: Vec<i32> = vec![];
    let color = piece.color();
    let mut direction = 1;
    if color == Color::White {
        direction = -1;
    }

    let x = position / 8;
    let y = position % 8;

    if board[((x + direction) * 8 + y) as usize] == ChessPiece::None {
        squares.push((x + direction) * 8 + y);
    }

    if (color == Color::White && x == 6) || (color == Color::Black && x == 1) {
        // Check if the square two squares in front of the pawn is empty
        if board[((x + 2 * direction) * 8 + y) as usize] == ChessPiece::None {
            squares.push((x + 2 * direction) * 8 + y);
        }
    }

    if y > 0 {
        let target_piece = board[((x + direction) * 8 + y - 1) as usize];
        if target_piece != ChessPiece::None && target_piece.color() != color {
            squares.push((x + direction) * 8 + y - 1);
        }
    }

    if y < 7 {
        let target_piece = board[((x + direction) * 8 + y + 1) as usize];
        if target_piece != ChessPiece::None && target_piece.color() != color {
            squares.push((x + direction) * 8 + y + 1);
        }
    }

    squares
}
