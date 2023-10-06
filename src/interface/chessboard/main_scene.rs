use crate::{
    actions::{
        path::enpassant_moves,
        play::{Game, Move, MoveType},
    },
    interface::chessboard::piece::{ChessPiece, Color as PieceColor},
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
    move_sound: Option<Gd<AudioStreamPlayer>>,
    capture_sound: Option<Gd<AudioStreamPlayer>>,
    castle_sound: Option<Gd<AudioStreamPlayer>>,
    check_sound: Option<Gd<AudioStreamPlayer>>,
    #[base]
    base: Base<Node2D>,
    game: Game,
    game_over: bool,
    engine_color: PieceColor,
}

#[derive(Debug, GodotClass)]
#[class(base=AudioStreamPlayer)]
pub struct MoveSound {
    #[base]
    base: Base<AudioStreamPlayer>,
}

#[godot_api]
impl AudioStreamPlayerVirtual for MoveSound {
    fn init(base: Base<AudioStreamPlayer>) -> Self {
        MoveSound { base }
    }
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
            move_sound: None,
            capture_sound: None,
            castle_sound: None,
            check_sound: None,
        }
    }

    fn ready(&mut self) {
        self.promotion_overlay = load("res://promote/modal_overlay.tscn");
        self.move_sound = Some(self.base.get_node_as::<AudioStreamPlayer>("MoveSound"));
        self.capture_sound = Some(self.base.get_node_as::<AudioStreamPlayer>("CaptureSound"));
        self.castle_sound = Some(self.base.get_node_as::<AudioStreamPlayer>("CastleSound"));
        self.check_sound = Some(self.base.get_node_as::<AudioStreamPlayer>("CheckSound"));
        let mut prom_overlay = self.base.get_node_as::<PromotionOverlay>("ModalOverlay");
        let mut board_hbox = self.base.get_node_as::<VBoxContainer>("Board");
        if self.engine_color != PieceColor::Black {
            board_hbox.set_rotation_degrees(180.0);
            board_hbox.set_position(Vector2::new(614.0, 614.0));
        }
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
        let board_mut = &mut board.bind_mut();

        board_mut.add_pieces(
            GodotString::from(self.game.fen.clone()),
            self.engine_color.toggle(),
        );
        board_mut.orientation = self.engine_color.toggle();
        // if self.engine_color == PieceColor::White {
        //     // self.engine_play();
        // }
    }
}

#[godot_api]
impl MainGame {
    #[signal]
    fn update_board();

    fn engine_play(&mut self) {
        println!("engine play");

        let engine = self.game.engine.clone();
        if engine.is_none() {
            return;
        }
        let mut engine = engine.unwrap();
        let best_move = engine.generate_best_move(&mut self.game, self.engine_color);

        let mut node = self.base.clone().cast::<MainGame>();
        node.emit_signal(
            "update_board".into(),
            &[Variant::from(PieceMove::from_move(&best_move))],
        );
    }

    fn play_sound(&mut self, move_: &Move, check: bool) {
        if move_.captured_piece == ChessPiece::None {
            if move_.move_type == MoveType::Castle {
                if let Some(sound) = &mut self.castle_sound {
                    sound.play();
                    if check {
                        if let Some(sound) = &mut self.check_sound {
                            sound.play();
                        }
                    }
                }
            }
            if let Some(sound) = &mut self.move_sound {
                // IF ENPASSANT
                if move_.move_type == MoveType::EnPassant {
                    if let Some(sound) = &mut self.capture_sound {
                        if check {
                            if let Some(sound) = &mut self.check_sound {
                                sound.play();
                            }
                        } else {
                            sound.play();
                        }
                    }
                } else {
                    if check {
                        if let Some(sound) = &mut self.check_sound {
                            sound.play();
                        }
                    } else {
                        sound.play();
                    }
                }
            }
        } else {
            if let Some(sound) = &mut self.capture_sound {
                if check {
                    if let Some(sound) = &mut self.check_sound {
                        sound.play();
                    }
                } else {
                    sound.play();
                }
            }
        }
    }

    #[func]
    fn on_choose_piece(&mut self, piece: GodotString, from: i32, to: i32) {
        //pro,ote pawns
        if self.game_over {
            return;
        }
        let board_placement = fen_to_board(&self.game.fen);
        let rules_part = self.game.fen.split(" ").collect::<Vec<&str>>();
        let move_ = Move {
            from: from,
            to: to,
            promote: String::from(&piece),
            piece: board_placement[from as usize],
            captured_piece: board_placement[to as usize],
            move_type: MoveType::Promotion,
            castling_rights: rules_part[2].to_string(),
        };
        let mut prom_overlay = self.base.get_node_as::<PromotionOverlay>("ModalOverlay");
        prom_overlay.hide();

        let (moved, check) = self.game.make_move(&move_);
        if moved {
            self.play_sound(&move_, check);
            self.game.turn = self.game.turn.toggle();
            let p_move = PieceMove::from_move(&move_);
            let mut node = self.base.clone().cast::<MainGame>();
            node.emit_signal("update_board".into(), &[Variant::from(p_move)]);
            if self.game.turn == self.engine_color && !self.game_over {
                // sleep(Duration::from_millis(10000));
                println!("engine play");
                // self.engine_play(); //TODO: engine play is not working (threading issue)
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
        let piece = board_placement[from as usize];
        let enp_squares = if piece == ChessPiece::WPawn || piece == ChessPiece::BPawn {
            enpassant_moves(from, piece, &self.game.fen)
        } else {
            vec![]
        };

        let is_enp = enp_squares.contains(&to);

        let is_casle =
            (piece == ChessPiece::BKing || piece == ChessPiece::WKing) && (from - to).abs() == 2;

        let move_tye = if is_enp {
            MoveType::EnPassant
        } else if is_casle {
            MoveType::Castle
        } else {
            MoveType::Normal
        };

        println!("move type {:?}", move_tye);

        let move_ = Move {
            from: from,
            to: to,
            promote: String::from(""),
            piece: board_placement[from as usize],
            captured_piece: board_placement[to as usize],
            move_type: move_tye,
            castling_rights: rules_part[2].to_string(),
        };
        let (moved, check) = self.game.make_move(&move_);
        if moved {
            self.play_sound(&move_, check);

            self.game.turn = self.game.turn.toggle();
            let p_move = PieceMove::from_move(&move_);
            let mut node = self.base.clone().cast::<MainGame>();
            node.emit_signal("update_board".into(), &[Variant::from(p_move)]);
            if self.game.turn == self.engine_color && !self.game_over {
                println!("engine play");
                // self.engine_play();  //TODO: engine play is not working (threading issue)
            }
        }
    }
}
