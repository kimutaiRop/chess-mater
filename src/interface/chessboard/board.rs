use godot::engine::{
    CenterContainer, CenterContainerVirtual, CollisionShape2D, CollisionShape2DVirtual, ColorRect,
    ColorRectVirtual,
};
use godot::engine::{GridContainer, GridContainerVirtual};
use godot::prelude::*;

use crate::actions::play::{GameState, Move, MoveType};
use crate::interface::chessboard::piece::{piece_to_fen, string_to_piece};
use crate::interface::chessboard::piece::{ChessPiece, Color as PieceColor, Piece};

use crate::interface::chessboard::promote::PromotionOverlay;

use super::main_scene::MainGame;
use super::piece::fen_to_board;
use super::promote::{PromoteMove, PromoteVbox};

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

#[derive(Debug, PartialEq, Clone, godot::prelude::ToVariant, FromVariant)]
pub enum GameStateVariant {
    Checkmate,
    Stalemate,
    Normal,
    Draw,
}

#[derive(Debug, PartialEq, Clone, godot::prelude::ToVariant, FromVariant)]
pub struct PieceMove {
    pub from: i32,
    pub to: i32,
    pub piece: ChessPiece,
    pub promote: GodotString,
    pub move_type: MoveType,
    pub captured_piece: ChessPiece,
    pub castling_rights: GodotString,
}

impl PieceMove {
    pub fn from_move(move_: &Move) -> Self {
        Self {
            from: move_.from,
            to: move_.to,
            piece: move_.piece,
            promote: GodotString::from(move_.promote.as_str()),
            move_type: move_.move_type.clone(),
            captured_piece: move_.captured_piece,
            castling_rights: GodotString::from(move_.castling_rights.as_str()),
        }
    }
}

impl GameStateVariant {
    pub fn from_game_state(state: GameState) -> Self {
        match state {
            GameState::Checkmate => GameStateVariant::Checkmate,
            GameState::Stalemate => GameStateVariant::Stalemate,
            GameState::Normal => GameStateVariant::Normal,
            GameState::Draw => GameStateVariant::Draw,
        }
    }
}

#[derive(Debug, godot::prelude::ToVariant, FromVariant)]
pub struct PlayResult {
    pub moved: bool,
    pub check: bool,
    pub state: GameStateVariant,
}

#[godot_api]
impl ColorRectVirtual for Square {
    fn init(node: Base<ColorRect>) -> Self {
        Self { node: node }
    }
    fn process(&mut self, _delta: f64) {
        // add the squares to the board
    }
    fn can_drop_data(&self, _at_position: Vector2, _data: Variant) -> bool {
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

        let square_parent_node = square_parent.clone().try_cast::<PlaceCenter>();
        if square_parent_node.is_none() {
            return;
        }
        let square_parent_node = square_parent_node.unwrap();

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
            let old_square_parent_node = old_square_parent_node.unwrap();
            let from = self.get_square_index(&old_square_parent_node);
            let to = self.get_square_index(&square_parent_node);

            Square::move_piece(from, to, &board_node);
        }
    }
}

impl Square {
    fn move_piece(from: i32, to: i32, board_node: &Gd<Node>) -> bool {
        let from_node = board_node.get_child(from).unwrap();
        let from_node = from_node.try_cast::<PlaceCenter>();
        if from_node.is_none() {
            return false;
        }
        let from_node = from_node.unwrap();
        let from_centre_node_drag = from_node.get_child(1);
        if from_centre_node_drag.is_none() {
            return false;
        }

        let piece = from_centre_node_drag.clone().unwrap().get_child(0);
        if piece.is_none() {
            return false;
        }
        let mut board = board_node.clone().try_cast::<Board>().unwrap();
        let piece = piece.unwrap().try_cast::<Piece>().unwrap();
        let piece = piece.bind();

        if (piece.piece == ChessPiece::BPawn && to >= 56 && to <= 63)
            || (piece.piece == ChessPiece::WPawn && to >= 0 && to <= 7)
        {
            let parent = board_node.get_parent().unwrap();
            let pp = parent.get_parent().unwrap();
            let pp = pp.get_parent().unwrap();

            let mut modal_overlay = pp.get_node_as::<PromotionOverlay>("ModalOverlay");

            let mut mo: Gd<PromotionOverlay> = modal_overlay.clone();

            let mut modal_overlay = modal_overlay.bind_mut();

            modal_overlay.move_ = Some(PromoteMove { from: from, to: to });
            mo.show();
        } else {
            board.emit_signal("trigger_move".into(), &[from.to_variant(), to.to_variant()]);
        }
        return true;
    }
    pub fn move_drag_element(board: &Gd<Node>, from: i32, to: i32) -> bool {
        let board = board.clone().try_cast::<Board>();
        if board.is_none() {
            return false;
        }
        let board = &mut board.unwrap();
        let from_node = board.get_child(from);
        if from_node.is_none() {
            return false;
        }
        let from_node = from_node.unwrap();
        let to_node = board.get_child(to).unwrap();
        let from_node = from_node.try_cast::<PlaceCenter>();
        if from_node.is_none() {
            return false;
        }
        let mut from_node = from_node.unwrap();
        let from_centre_node_drag = from_node.get_child(1);
        if from_centre_node_drag.is_none() {
            return false;
        }
        let from_centre_node_drag = from_centre_node_drag.unwrap();
        let from_centre_node_drag = from_centre_node_drag.try_cast::<PlaceCenterDrag>();

        // from is ready

        let to_node = to_node.try_cast::<PlaceCenter>();
        if to_node.is_none() {
            return false;
        }
        let mut to_node = to_node.unwrap();

        if from_centre_node_drag.is_none() {
            return false;
        }
        let from_centre_node_drag = from_centre_node_drag.unwrap();

        // remove from node
        from_node.remove_child(from_centre_node_drag.clone().upcast::<Node>());
        let to_centre_node_drag = to_node.get_child(1);
        if to_centre_node_drag.is_some() {
            to_node.remove_child(to_centre_node_drag.unwrap());
        }
        to_node.add_child(from_centre_node_drag.clone().upcast::<Node>());
        return true;
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
    pub promote: Option<Gd<PromoteVbox>>,
    #[base]
    node: Base<GridContainer>,
    pub orientation: PieceColor,
}

#[godot_api]
impl GridContainerVirtual for Board {
    fn init(node: Base<GridContainer>) -> Self {
        let mut node: Base<GridContainer> = node;
        node.set_columns(8);
        node.add_theme_constant_override(StringName::from("hseparation"), 0);
        node.add_theme_constant_override(StringName::from("vseparation"), 0);

        let mut board = Self {
            node,
            promote: None,
            orientation: PieceColor::White,
        };
        board.create_grid();
        board
    }

    fn ready(&mut self) {
        let main_game = self
            .node
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap();
        let mut main_game = main_game.try_cast::<MainGame>().unwrap();
        let node = self.node.clone().upcast::<Node>();
        main_game.connect(
            "update_board".into(),
            Callable::from_object_method(node, "trigger_movement"),
        );
    }
}

#[godot_api]
impl Board {
    #[signal]
    fn update_fen() {}

    #[signal]
    fn trigger_move() {}

    #[func]
    pub fn trigger_movement(&self, piece_move: Variant) {
        let piece_move = piece_move.try_to::<PieceMove>();
        let piece_move = piece_move.unwrap();
        let board_node = self.node.clone().upcast::<Node>();
        let from_node = board_node.get_child(piece_move.from).unwrap();
        let from_node = from_node.clone().try_cast::<PlaceCenter>();

        let mut from_node = from_node.unwrap();
        let from_centre_node_drag = from_node.get_child(1);

        let move_diff = piece_move.from - piece_move.to;
        if piece_move.move_type == MoveType::EnPassant
            && (piece_move.piece == ChessPiece::BPawn || piece_move.piece == ChessPiece::WPawn)
        {
            let enp_pawn_pos = if piece_move.piece == ChessPiece::BPawn {
                piece_move.to - 8
            } else {
                piece_move.to + 8
            };
            let empassant_pawn = board_node.get_child(enp_pawn_pos).unwrap();
            let empassant_pawn = empassant_pawn.try_cast::<PlaceCenter>();

            let mut empassant_pawn = empassant_pawn.unwrap();
            let drag_empassant_pawn = empassant_pawn.get_child(1);
            empassant_pawn.remove_child(drag_empassant_pawn.unwrap());
            println!("enpassant pawn removed");
        }

        from_node.remove_child(from_centre_node_drag.clone().unwrap());

        let to_node = board_node.get_child(piece_move.to).unwrap();
        let to_node = to_node.try_cast::<PlaceCenter>();

        let mut to_node = to_node.unwrap();
        let to_centre_node_drag = to_node.get_child(1);
        if to_centre_node_drag.is_some() {
            to_node.remove_child(to_centre_node_drag.unwrap());
        }
        if piece_move.move_type == MoveType::Castle
            && piece_move.piece == ChessPiece::BKing
            && move_diff.abs() == 2
        {
            Square::move_drag_element(
                &board_node,
                if move_diff > 0 { 0 } else { 7 },
                if move_diff > 0 { 3 } else { 5 },
            );
        } else if piece_move.move_type == MoveType::Castle
            && piece_move.piece == ChessPiece::WKing
            && move_diff.abs() == 2
        {
            Square::move_drag_element(
                &board_node,
                if move_diff > 0 { 56 } else { 63 },
                if move_diff > 0 { 59 } else { 61 },
            );
        }

        if piece_move.promote.to_string().as_str() != "" {
            let pawn = piece_move.piece;
            let piece = match piece_move.promote.to_string().as_str() {
                "q" => {
                    if pawn.color() == crate::interface::chessboard::piece::Color::White {
                        ChessPiece::WQueen
                    } else {
                        ChessPiece::BQueen
                    }
                }
                "r" => {
                    if pawn.color() == crate::interface::chessboard::piece::Color::White {
                        ChessPiece::WRook
                    } else {
                        ChessPiece::BRook
                    }
                }
                "b" => {
                    if pawn.color() == crate::interface::chessboard::piece::Color::White {
                        ChessPiece::WBishop
                    } else {
                        ChessPiece::BBishop
                    }
                }
                "n" => {
                    if pawn.color() == crate::interface::chessboard::piece::Color::White {
                        ChessPiece::WKnight
                    } else {
                        ChessPiece::BKnight
                    }
                }
                _ => ChessPiece::None,
            };
            let piece = Board::create_piece(piece, self.orientation);
            let piece = piece.try_to::<Gd<PlaceCenterDrag>>();

            to_node.add_child(piece.unwrap().upcast::<Node>());
        } else {
            to_node.add_child(from_centre_node_drag.clone().unwrap());
        }
    }

    fn create_grid(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                // create node from disc
                let node = load::<PackedScene>("res://square.tscn");
                let node = node.instantiate().unwrap();
                let mut node: Gd<Square> = node.cast::<Square>();
                node.add_theme_constant_override(StringName::from("separation"), 0);
                node.set_scale(Vector2::new(0.125, 0.125));
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
    pub fn add_pieces(&self, fen: GodotString, player_color: PieceColor) {
        let array_fen = fen_to_board(&fen.to_string());
        for i in 0..8 {
            for j in 0..8 {
                let piece: ChessPiece = array_fen[i * 8 + j];
                let mut squre_centre_node = self.node.get_child((i * 8 + j) as i32).unwrap();
                let centre = Board::create_piece(piece, player_color);
                let centre = centre.try_to::<Gd<PlaceCenterDrag>>();
                if centre.is_err() {
                    continue;
                }
                let centre = centre.unwrap();
                squre_centre_node.add_child(centre.clone().upcast::<Node>());
            }
        }
    }

    pub fn create_piece(piece: ChessPiece, player_color: PieceColor) -> Variant {
        let centre = load::<PackedScene>("res://place_center_drag.tscn");
        let centre = centre.instantiate().unwrap();
        let mut centre = centre.cast::<PlaceCenterDrag>();
        if piece != ChessPiece::None {
            let path = piece_to_fen(&piece);
            if path.len() > 0 {
                let piece_node = load::<PackedScene>(format!("res://{}.tscn", path));
                let piece_node = piece_node.instantiate().unwrap();
                let mut piece_node = piece_node.cast::<Piece>();
                let mut piece_mut = piece_node.clone();
                if player_color == PieceColor::Black {
                    piece_node.set_rotation_degrees(180.0);
                }
                piece_node.set_centered(true);
                piece_node.set_scale(Vector2::new(0.55, 0.55));

                let mut piece_mut = piece_mut.bind_mut();
                piece_mut.piece = string_to_piece(&path);
                centre.add_child(piece_node.upcast::<Node>());
            }
        }
        centre.to_variant()
    }
}
