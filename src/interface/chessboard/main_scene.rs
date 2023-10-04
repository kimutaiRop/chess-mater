use godot::{
    engine::{Node2D, Node2DVirtual, VBoxContainer},
    prelude::*,
};

use super::{
    board::{Board, PlayResult},
    promote::PromotionOverlay,
};

#[derive(Debug, GodotClass)]
#[class(base=Node2D)]
pub struct MainGame {
    promotion_overlay: Gd<PackedScene>,
    #[base]
    base: Base<Node2D>,
    pub fen: GodotString,
}

#[godot_api]
impl Node2DVirtual for MainGame {
    fn init(base: Base<Node2D>) -> Self {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string();

        MainGame {
            base,
            promotion_overlay: PackedScene::new(),
            fen: GodotString::from(fen),
        }
    }

    fn ready(&mut self) {
        println!("MainGame::ready()");
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
        board_mut.add_pieces(self.fen.clone());
    }
}

#[godot_api]
impl MainGame {
    #[func]
    fn on_choose_piece(&mut self, piece: GodotString, from: i32, to: i32) {
        let board = self.base.get_node_as::<VBoxContainer>("Board");
        let board = board
            .get_child(0)
            .unwrap()
            .get_child(0)
            .unwrap()
            .cast::<Board>();
        let boad_mut = board.bind();
        let play_variant = boad_mut.trigger_movement(self.fen.clone(), piece, from, to);
        let pay = play_variant.try_to::<PlayResult>();
        let play = pay.unwrap();
        self.fen = GodotString::from(play.fen);
        let mut prom_overlay = self.base.get_node_as::<PromotionOverlay>("ModalOverlay");
        prom_overlay.hide();
    }

    #[func]
    fn on_trigger_move(&mut self, from: i32, to: i32) {
        let board = self.base.get_node_as::<VBoxContainer>("Board");
        let board = board
            .get_child(0)
            .unwrap()
            .get_child(0)
            .unwrap()
            .cast::<Board>();
        let boad = board.bind();

        let play_variant = boad.trigger_movement(self.fen.clone(), GodotString::from(""), from, to);
        let pay = play_variant.try_to::<PlayResult>();
        let play = pay.unwrap();
        self.fen = GodotString::from(play.fen);
    }
}
