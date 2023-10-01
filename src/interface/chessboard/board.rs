use godot::engine::{
    CenterContainer, CenterContainerVirtual, CollisionShape2D, CollisionShape2DVirtual, ColorRect,
    ColorRectVirtual,
};
use godot::engine::{GridContainer, GridContainerVirtual};
use godot::prelude::*;

use crate::actions::play::{make_move, Move};
use crate::interface::chessboard::piece::string_to_piece;

use super::piece::{fen_to_board, ChessPiece, Piece};

#[derive(GodotClass)]
#[class(base=ColorRect)]
pub struct Square {
    #[base]
    node: Base<ColorRect>,
}
#[derive(GodotClass)]
#[class(base=CenterContainer)]
pub struct PlaceCenter {
    #[base]
    _node: Base<CenterContainer>,
}
#[derive(GodotClass)]
#[class(base=CenterContainer)]
pub struct PlaceCenterDrag {
    #[base]
    _node: Base<CenterContainer>,
}

#[godot_api]
impl ColorRectVirtual for Square {
    fn init(node: Base<ColorRect>) -> Self {
        Self { node: node }
    }
    fn process(&mut self, _delta: f64) {
        // add the squares to the board
    }
    fn can_drop_data(&self, at_position: Vector2, _data: Variant) -> bool {
        return true;
    }
    fn get_drag_data(&mut self, _at_position: Vector2) -> Variant {
        let parent = self.node.get_parent();
        if parent.is_none() {
            return Variant::nil();
        }
        let centre_node = parent.unwrap().try_cast::<PlaceCenter>();
        let centre_node_drag = centre_node.unwrap().get_child(1);
        if centre_node_drag.is_none() {
            return Variant::nil();
        }
        let centre_node_drag = centre_node_drag.unwrap();
        let centre_node_drag = centre_node_drag.try_cast::<PlaceCenterDrag>();
        if centre_node_drag.is_some() {
            let centre_node_drag = centre_node_drag.unwrap();
            // make sure piece is not none
            let piece_node = centre_node_drag.get_child(0);
            if piece_node.is_none() {
                return Variant::nil();
            }
            let piece_node = piece_node.unwrap();
            let piece = piece_node.try_cast::<Piece>();
            if piece.is_none() {
                return Variant::nil();
            }
            return centre_node_drag.to_variant();
        }
        return Variant::nil();
    }
    fn drop_data(&mut self, _at_position: Vector2, data: Variant) {
        if data.is_nil() {
            return;
        }
        // replace index 1 child with data if parent 1 exists or add child to parent 0
        let square_parent = self.node.get_parent();
        if square_parent.is_none() {
            return;
        }
        let square_parent = square_parent.unwrap();
        let board = square_parent.get_parent();
        if board.is_none() {
            return;
        }
        let board_node: Gd<Node> = board.clone().unwrap();
        let board = board.clone().unwrap();

        let board = board.try_cast::<Board>();
        if board.is_none() {
            return;
        }
        let board = &mut board.unwrap();
        let board = &mut board.bind_mut();

        let square_parent_node = square_parent.clone().try_cast::<PlaceCenter>();
        if square_parent_node.is_none() {
            return;
        }
        let mut square_parent_node = square_parent_node.unwrap();
        let centre_node_drag = square_parent_node.get_child(1);

        let new_centre_drag_node = data.try_to::<Gd<PlaceCenterDrag>>();
        if new_centre_drag_node.is_ok() {
            let new_centre_drag_node = new_centre_drag_node.unwrap();
            let old_square_parent_node = new_centre_drag_node.get_parent();
            if old_square_parent_node.is_none() {
                return;
            }
            let old_square_parent_node = old_square_parent_node.unwrap();
            let old_square_parent_node = old_square_parent_node.try_cast::<PlaceCenter>();
            if old_square_parent_node.is_none() {
                return;
            }
            let mut old_square_parent_node = old_square_parent_node.unwrap();
            let piece_node = new_centre_drag_node.get_child(0).unwrap();
            let mut piece = piece_node.try_cast::<Piece>().unwrap();
            let mut piece = piece.bind_mut();
            let piece_move = Move {
                from: self.get_square_index(&old_square_parent_node),
                to: self.get_square_index(&square_parent_node),
                piece: piece.piece,
                moved: piece.moved,
                fen: board.fen.clone(),
                from_times_moved: piece.times_moved,
                can_enpassant: board.can_enpassant.clone(),
                can_castle: board.can_castle.clone(),
            };
            let allowed = make_move(&piece_move);
            if !allowed.2 {
                return;
            }
            board.fen = allowed.0.to_string();
            piece.moved = true;
            piece.times_moved += 1;
            // if any thing is in can_enpassant then remove it
            board.can_enpassant = vec![];
            if allowed.1.is_some() {
                board.can_enpassant.push(allowed.1.unwrap());
            }

            // if from or to is 4 then remove castling for
            if piece_move.from == 4 || piece_move.to == 4 {
                // check id 1 or 2 is in can_castle remove it
                if board.can_castle.contains(&1) {
                    let index = board.can_castle.iter().position(|&x| x == 1).unwrap();
                    board.can_castle.remove(index);
                }
                if board.can_castle.contains(&2) {
                    let index = board.can_castle.iter().position(|&x| x == 2).unwrap();
                    board.can_castle.remove(index);
                }
            }

            // if from or to is 60 then remove castling for white
            if piece_move.from == 60 || piece_move.to == 60 {
                // check id 3 or 4 is in can_castle remove it
                if board.can_castle.contains(&3) {
                    let index = board.can_castle.iter().position(|&x| x == 3).unwrap();
                    board.can_castle.remove(index);
                }
                if board.can_castle.contains(&4) {
                    let index = board.can_castle.iter().position(|&x| x == 4).unwrap();
                    board.can_castle.remove(index);
                }
            }

            // if from or to is 0 then remove queen side castling for black
            if piece_move.from == 0 || piece_move.to == 0 {
                if board.can_castle.contains(&1) {
                    let index = board.can_castle.iter().position(|&x| x == 1).unwrap();
                    board.can_castle.remove(index);
                }
            } else if piece_move.from == 7 || piece_move.to == 7 {
                if board.can_castle.contains(&2) {
                    let index = board.can_castle.iter().position(|&x| x == 2).unwrap();
                    board.can_castle.remove(index);
                }
            } else if piece_move.from == 56 || piece_move.to == 56 {
                if board.can_castle.contains(&3) {
                    let index = board.can_castle.iter().position(|&x| x == 3).unwrap();
                    board.can_castle.remove(index);
                }
            } else if piece_move.from == 63 || piece_move.to == 63 {
                if board.can_castle.contains(&4) {
                    let index = board.can_castle.iter().position(|&x| x == 4).unwrap();
                    board.can_castle.remove(index);
                }
            }

            // if king moves for casting call move drag element
            let move_diff = piece_move.from - piece_move.to;

            if piece_move.piece == ChessPiece::BKing && move_diff.abs() == 2 {
                self.move_drag_element(
                    board_node,
                    if move_diff > 0 { 0 } else { 7 },
                    if move_diff > 0 { 3 } else { 5 },
                );
            } else if piece_move.piece == ChessPiece::WKing && move_diff.abs() == 2 {
                self.move_drag_element(
                    board_node,
                    if move_diff > 0 { 56 } else { 63 },
                    if move_diff > 0 { 59 } else { 61 },
                );
            }

            if centre_node_drag.is_some() {
                let mut centre_node_drag = centre_node_drag.unwrap();
                let centre_node_drag_child = centre_node_drag.get_child(0);
                if centre_node_drag_child.is_some() {
                    let centre_node_drag_child = centre_node_drag_child.unwrap();
                    centre_node_drag.remove_child(centre_node_drag_child);
                }
                square_parent_node.remove_child(centre_node_drag.clone());
                old_square_parent_node.add_child(centre_node_drag.clone());
            }
            old_square_parent_node.remove_child(new_centre_drag_node.clone().upcast::<Node>());
            square_parent_node.add_child(new_centre_drag_node.upcast::<Node>());
        }
    }
}

impl Square {
    pub fn move_drag_element(&self, board: Gd<Node>, from: i32, to: i32) {
        println!("move_drag_element {:} {:}", from, to);
        let board = board.try_cast::<Board>();
        if board.is_none() {
            return;
        }
        let board = &mut board.unwrap();
        let from_node = board.get_child(from);
        if from_node.is_none() {
            println!("from_node is none");
            return;
        }
        let from_node = from_node.unwrap();
        println!("from_node {:}", from_node);
        let to_node = board.get_child(to).unwrap();
        let from_node = from_node.try_cast::<PlaceCenter>();
        if from_node.is_none() {
            return;
        }
        let mut from_node = from_node.unwrap();
        let from_centre_node_drag = from_node.get_child(1);
        println!("from_node {:}", from_node);

        if from_centre_node_drag.is_none() {
            return;
        }
        let from_centre_node_drag = from_centre_node_drag.unwrap();
        let from_centre_node_drag = from_centre_node_drag.try_cast::<PlaceCenterDrag>();

        // from is ready

        let to_node = to_node.try_cast::<PlaceCenter>();
        if to_node.is_none() {
            return;
        }
        let mut to_node = to_node.unwrap();
        let to_centre_node_drag = to_node.get_child(1);
        if to_centre_node_drag.is_none() {
            return;
        }
        let to_centre_node_drag = to_centre_node_drag.unwrap();
        let to_centre_node_drag = to_centre_node_drag.try_cast::<PlaceCenterDrag>();

        if from_centre_node_drag.is_none() {
            return;
        }
        let from_centre_node_drag = from_centre_node_drag.unwrap();
        if to_centre_node_drag.is_none() {
            return;
        }
        let to_centre_node_drag = to_centre_node_drag.unwrap();

        // remove from node
        from_node.remove_child(from_centre_node_drag.clone().upcast::<Node>());
        // remove to node
        to_node.remove_child(to_centre_node_drag.clone().upcast::<Node>());

        // add to node
        from_node.add_child(to_centre_node_drag.clone().upcast::<Node>());
        // add from node
        to_node.add_child(from_centre_node_drag.clone().upcast::<Node>());
    }
    pub fn get_square_index(&self, node: &Gd<PlaceCenter>) -> i32 {
        let parent = node.get_parent();
        if parent.is_none() {
            return -1;
        }
        let parent = parent.unwrap();
        let parent = parent.try_cast::<Board>();
        if parent.is_none() {
            return -1;
        }
        let parent = parent.unwrap();

        let children: Array<Gd<Node>> = parent.get_children();
        let mut c = 0;
        let child_len = children.len() as i32;
        for i in 0..child_len {
            let child = parent.get_child(i).unwrap();
            let child_node = &child.cast::<PlaceCenter>();
            if child_node == node {
                c = i;
                continue;
            }
        }
        godot_print!("c:{}", c);
        return c;
    }
}

#[derive(GodotClass)]
#[class(base=CollisionShape2D)]
pub struct DragCollider {
    #[base]
    pub node: Base<CollisionShape2D>,
}

#[godot_api]
impl CollisionShape2DVirtual for DragCollider {
    fn init(node: Base<CollisionShape2D>) -> Self {
        Self { node }
    }
}

#[godot_api]
impl CenterContainerVirtual for PlaceCenterDrag {
    fn init(node: Base<CenterContainer>) -> Self {
        Self { _node: node }
    }
}

impl PlaceCenterDrag {
    fn _on_mouse_entered(&mut self, size: Vector2) {
        self._node.set_size(size);
    }
}

#[godot_api]
impl CenterContainerVirtual for PlaceCenter {
    fn init(node: Base<CenterContainer>) -> Self {
        Self { _node: node }
    }
}

#[derive(GodotClass, Debug)]
#[class(base=GridContainer)]
pub struct Board {
    #[base]
    pub fen: String,
    node: Base<GridContainer>,
    can_enpassant: Vec<i32>,
    can_castle: Vec<i32>,
}

#[godot_api]
impl GridContainerVirtual for Board {
    fn init(node: Base<GridContainer>) -> Self {
        // create mutabale node
        let mut node: Base<GridContainer> = node;
        node.set_columns(8);
        node.add_theme_constant_override(StringName::from("hseparation"), 0);
        node.add_theme_constant_override(StringName::from("vseparation"), 0);
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string();
        let mut board = Self {
            node,
            fen: fen.clone(),
            can_enpassant: vec![],
            can_castle: vec![1, 2, 3, 4],
        };
        board.create_grid();
        board.add_pieces();
        board
    }
}

impl Board {
    fn create_grid(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                // create node from disc
                let node = load::<PackedScene>("res://square.tscn");
                let node = node.instantiate().unwrap();
                let mut node: Gd<Square> = node.cast::<Square>();
                node.add_theme_constant_override(StringName::from("separation"), 0);

                let centre = load::<PackedScene>("res://place_center.tscn");
                let centre = centre.instantiate().unwrap();
                let mut centre: Gd<PlaceCenter> = centre.cast::<PlaceCenter>();
                if (i + j) % 2 == 0 {
                    node.set_color(Color::from_rgb(238.0, 238.0, 228.0));
                } else {
                    node.set_color(Color::from_rgb(0.0, 129.0, 176.0));
                }
                centre.add_child(node.upcast::<Node>());
                self.node.add_child(centre.upcast::<Node>());
            }
        }
    }
    fn add_pieces(&mut self) {
        let array_fen = fen_to_board(&self.fen);

        for i in 0..8 {
            for j in 0..8 {
                let piece = array_fen[i * 8 + j];
                let mut squre_centre_node = self.node.get_child((i * 8 + j) as i32).unwrap();
                godot_print!("squre_node {:?}", squre_centre_node);
                let centre = load::<PackedScene>("res://place_center_drag.tscn");
                let centre = centre.instantiate().unwrap();
                let mut centre = centre.cast::<PlaceCenterDrag>();
                if piece != ChessPiece::None {
                    let path = match piece {
                        ChessPiece::BPawn => ("res://p.tscn", "p"),
                        ChessPiece::WPawn => ("res://P.tscn", "P"),
                        ChessPiece::BKnight => ("res://n.tscn", "n"),
                        ChessPiece::WKnight => ("res://N.tscn", "N"),
                        ChessPiece::BBishop => ("res://b.tscn", "b"),
                        ChessPiece::WBishop => ("res://B.tscn", "B"),
                        ChessPiece::BRook => ("res://r.tscn", "r"),
                        ChessPiece::WRook => ("res://R.tscn", "R"),
                        ChessPiece::BQueen => ("res://q.tscn", "q"),
                        ChessPiece::WQueen => ("res://Q.tscn", "Q"),
                        ChessPiece::BKing => ("res://k.tscn", "k"),
                        ChessPiece::WKing => ("res://K.tscn", "K"),
                        ChessPiece::None => ("", ""),
                    };

                    if path.1.len() > 0 {
                        let piece_node = load::<PackedScene>(path.0);
                        let piece_node = piece_node.instantiate().unwrap();
                        let mut piece_node = piece_node.cast::<Piece>();
                        let mut piece_mut = piece_node.clone();
                        piece_node.set_centered(true);
                        piece_node.set_scale(Vector2::new(0.55, 0.55));
                        let mut piece_mut = piece_mut.bind_mut();
                        piece_mut.piece = string_to_piece(&path.1.to_string());
                        piece_mut.moved = false;
                        centre.add_child(piece_node.upcast::<Node>());
                    }
                }
                squre_centre_node.add_child(centre.clone().upcast::<Node>());
            }
        }
    }
}