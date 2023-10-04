use godot::{
    engine::{
        Button, ButtonVirtual, MarginContainer, MarginContainerVirtual, VBoxContainer,
        VBoxContainerVirtual,
    },
    prelude::*,
};

#[derive(Debug)]
pub struct PromoteMove {
    pub from: i32,
    pub to: i32,
}

#[derive(Debug, GodotClass)]
#[class(base=VBoxContainer)]
pub struct PromoteVbox {
    modal_overlay: Option<Gd<Node>>,
    node: Base<VBoxContainer>,
}

#[derive(Debug, GodotClass)]
#[class(base=Button)]
pub struct PromoteButton {
    _node: Base<Button>,
}

#[godot_api]
impl ButtonVirtual for PromoteButton {
    fn init(_node: Base<Button>) -> Self {
        Self { _node }
    }
}

#[derive(Debug, GodotClass)]
#[class(base=MarginContainer)]
pub struct PromotionOverlay {
    pub move_: Option<PromoteMove>,
    #[base]
    node: Base<MarginContainer>,
}

#[godot_api]
impl MarginContainerVirtual for PromotionOverlay {
    fn init(node: Base<MarginContainer>) -> Self {
        Self { node, move_: None }
    }
}

#[godot_api]
impl PromotionOverlay {
    #[signal]
    fn choose_piece() {}

    // these functions as godot does not suporrt bining data yet
    #[func]
    pub fn _on_promote_button_pressed_q(&mut self) {
        self.node.emit_signal(
            "choose_piece".into(),
            &[
                "q".to_variant(),
                self.move_.as_mut().unwrap().from.to_variant(),
                self.move_.as_mut().unwrap().to.to_variant(),
            ],
        );
    }
    #[func]
    pub fn _on_promote_button_pressed_r(&mut self) {
        self.node.emit_signal(
            "choose_piece".into(),
            &[
                "r".to_variant(),
                self.move_.as_mut().unwrap().from.to_variant(),
                self.move_.as_mut().unwrap().to.to_variant(),
            ],
        );
    }
    #[func]
    pub fn _on_promote_button_pressed_k(&mut self) {
        self.node.emit_signal(
            "choose_piece".into(),
            &[
                "n".to_variant(),
                self.move_.as_mut().unwrap().from.to_variant(),
                self.move_.as_mut().unwrap().to.to_variant(),
            ],
        );
    }
    #[func]
    pub fn _on_promote_button_pressed_b(&mut self) {
        self.node.emit_signal(
            "choose_piece".into(),
            &[
                "b".to_variant(),
                self.move_.as_mut().unwrap().from.to_variant(),
                self.move_.as_mut().unwrap().to.to_variant(),
            ],
        );
    }
}

#[godot_api]
impl VBoxContainerVirtual for PromoteVbox {
    fn init(node: Base<VBoxContainer>) -> Self {
        Self {
            node,
            modal_overlay: None,
        }
    }
    fn ready(&mut self) {
        self.add_buttons();
    }
}

#[godot_api]
impl PromoteVbox {
    #[func]
    fn add_buttons(&mut self) {
        for i in "qrbk".chars() {
            let base = self.node.get_parent().unwrap().get_parent().unwrap();
            let base = base.cast::<PromotionOverlay>();
            let scene = load::<PackedScene>(format!("res://promote/{}.tscn", i));
            let node_button = scene.instantiate().unwrap();
            let mut button = node_button.clone().cast::<Button>();
            button.set_size(Vector2::new(200.0, 25.0));

            button.connect(
                "pressed".into(),
                Callable::from_object_method(base, format!("_on_promote_button_pressed_{}", i)),
            );
            self.node.add_child(node_button);
        }
    }
}
