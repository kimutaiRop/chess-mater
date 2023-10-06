use crate::{actions::play::Game, interface::chessboard::piece::Color as PieceColor};
use godot::{
    engine::{Node2D, Node2DVirtual, VBoxContainer},
    prelude::*,
};

use super::{
    board::{Board, GameStateVariant, PlayResult},
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
        let fen = "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a4 0 1".to_string();

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
        if self.engine_color == PieceColor::White {
            // self.engine_play();
        }
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
        let board_mut = board.bind();
    }

    #[func]
    fn on_choose_piece(&mut self, piece: GodotString, from: i32, to: i32) {
        if self.game_over {
            return;
        }
        let board = self.base.get_node_as::<VBoxContainer>("Board");
        let board = board
            .get_child(0)
            .unwrap()
            .get_child(0)
            .unwrap()
            .cast::<Board>();
        // let board_mut = board.bind();
        // let play_variant = board_mut.trigger_movement(piece, from, to);
        // let pay = play_variant.try_to::<PlayResult>();
        // let play = pay.unwrap();
        let mut prom_overlay = self.base.get_node_as::<PromotionOverlay>("ModalOverlay");
        prom_overlay.hide();

        if self.game.turn == self.engine_color && !self.game_over {
            // self.engine_play();
        }
    }

    #[func]
    fn on_trigger_move(&mut self, from: i32, to: i32) {
        if self.game_over {
            return;
        }
        let board = self.base.get_node_as::<VBoxContainer>("Board");
        let board = board
            .get_child(0)
            .unwrap()
            .get_child(0)
            .unwrap()
            .cast::<Board>();
        // let board_mut = board.bind();

        // let play_variant =
        //     board_mut.trigger_movement(GodotString::from(""), from, to);
        // let pay = play_variant.try_to::<PlayResult>();
        // let play = pay.unwrap();
        // if play.state == GameStateVariant::Draw || play.state == GameStateVariant::Checkmate {
        //     self.game_over = true;
        //     // let mut prom_overlay = self.base.get_node_as::<PromotionOverlay>("ModalOverlay");
        //     // prom_overlay.show();
        // }

        if self.game.turn == self.engine_color && !self.game_over {
            // self.engine_play();
        }
    }
}
