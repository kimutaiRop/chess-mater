use crate::{
    actions::{
        path::enpassant_moves,
        play::{Game, Move, MoveType},
    },
    interface::chessboard::piece::{ChessPiece, Color as PieceColor, Piece},
};
use godot::{
    engine::{Node2D, Node2DVirtual, VBoxContainer},
    prelude::*,
};

use super::{
    board::{Board, PieceMove},
    piece::fen_to_board,
    promote::PromotionOverlay,
};

#[derive(Debug, GodotClass)]
#[class(base=Node2D)]
pub struct MainGame {
    promotion_overlay: Gd<PackedScene>,
    #[base]
    base: Base<Node2D>,
    game: Game,
    game_over: bool,
    engine_color: PieceColor,
}

#[godot_api]
impl Node2DVirtual for MainGame {
    fn init(base: Base<Node2D>) -> Self {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq a4 0 1".to_string();

        MainGame {
            base,
            promotion_overlay: PackedScene::new(),
            game: Game::new(&fen, None),
            game_over: false,
            engine_color: PieceColor::Black,
        }
    }

    fn ready(&mut self) {
        self.promotion_overlay = load("res://promote/modal_overlay.tscn");
        let mut prom_overlay = self.base.get_node_as::<PromotionOverlay>("ModalOverlay");
        let board_hbox = self.base.get_node_as::<VBoxContainer>("Board");
        let mut board = board_hbox
            .get_child(0)
            .unwrap()
            .get_child(0)
            .unwrap()
            .cast::<Board>();

        prom_overlay.hide();
        let node = self.base.clone().cast::<MainGame>();
        prom_overlay.connect(
            "choose_piece".into(),
            Callable::from_object_method(node.clone(), "on_choose_piece"),
        );

        board.connect(
            "trigger_move".into(),
            Callable::from_object_method(node.clone(), "on_trigger_move"),
        );
        // get mut ref to board
        let board_mut = board.bind();

        board_mut.add_pieces(GodotString::from(self.game.fen.clone()));
        // if self.engine_color == PieceColor::White {
        //     // self.engine_play();
        // }
    }
}

#[godot_api]
impl MainGame {
    #[func]
    fn engine_play(&mut self) {
        println!("engine play");
        let board = self.base.get_node_as::<VBoxContainer>("Board");
        let board = board
            .get_child(0)
            .unwrap()
            .get_child(0)
            .unwrap()
            .cast::<Board>();
    }

    #[func]
    fn on_choose_piece(&mut self, piece: GodotString, from: i32, to: i32) {
        //pro,ote pawns
        if self.game_over {
            return;
        }
        let board_placement = fen_to_board(&self.game.fen);
        let rules_part = self.game.fen.split(" ").collect::<Vec<&str>>();
        let board = self.base.get_node_as::<VBoxContainer>("Board");
        let board = board
            .get_child(0)
            .unwrap()
            .get_child(0)
            .unwrap()
            .cast::<Board>();
        let board_mut = board.bind();
        let move_ = Move {
            from: from,
            to: to,
            promote: String::from(&piece),
            piece: board_placement[from as usize],
            captured_piece: board_placement[to as usize],
            move_type: MoveType::Promotion,
            castling_rights: rules_part[2].to_string(),
        };
        let moved = self.game.make_move(&move_);
        if moved {
            let p_move = PieceMove::from_move(&move_);
            let mut prom_overlay = self.base.get_node_as::<PromotionOverlay>("ModalOverlay");
            prom_overlay.hide();

            board_mut.trigger_movement(Variant::from(p_move));
            if self.game.turn == self.engine_color && !self.game_over {
                // self.engine_play();
            }
        }
    }

    #[func]
    fn on_trigger_move(&mut self, from: i32, to: i32) {
        if self.game_over {
            return;
        }
        let board_placement = fen_to_board(&self.game.fen);
        let rules_part = self.game.fen.split(" ").collect::<Vec<&str>>();
        let board = self.base.get_node_as::<VBoxContainer>("Board");
        let board = board
            .get_child(0)
            .unwrap()
            .get_child(0)
            .unwrap()
            .cast::<Board>();
        let board_mut = board.bind();
        let piece = board_placement[from as usize];
        let enp_squares = if piece == ChessPiece::WPawn || piece == ChessPiece::BPawn {
            enpassant_moves(from, piece, &self.game.fen)
        } else {
            vec![]
        };

        let is_enp = enp_squares.contains(&to);

        let is_casle =
            (piece == ChessPiece::WPawn || piece == ChessPiece::BPawn) && (from - to).abs() == 2;

        let move_tye = if is_enp {
            MoveType::EnPassant
        } else if is_casle {
            MoveType::Castle
        } else {
            MoveType::Normal
        };

        let move_ = Move {
            from: from,
            to: to,
            promote: String::from(""),
            piece: board_placement[from as usize],
            captured_piece: board_placement[to as usize],
            move_type: move_tye,
            castling_rights: rules_part[2].to_string(),
        };
        let moved = self.game.make_move(&move_);
        if moved {
            let p_move = PieceMove::from_move(&move_);

            board_mut.trigger_movement(Variant::from(p_move));
            if self.game.turn == self.engine_color && !self.game_over {
                // self.engine_play();
            }
        }
    }
}
