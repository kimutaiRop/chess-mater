use godot::{
    engine::{Button, ButtonVirtual, VBoxContainer, VBoxContainerVirtual},
    prelude::*,
};

#[derive(Debug, GodotClass)]
#[class(base=VBoxContainer)]
pub struct PromoteVbox {
    index: Option<i32>,
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

#[godot_api]
impl VBoxContainerVirtual for PromoteVbox {
    fn init(node: Base<VBoxContainer>) -> Self {
        Self { node, index: None }
    }
    fn ready(&mut self) {
        self.add_buttons();
    }
}

#[godot_api]
impl PromoteVbox {
    #[signal]
    fn choose_piece() {}

    fn add_buttons(&mut self) {
        for i in "kqrb".chars() {
            println!("Loading: {}", i);
            let base = self.node.clone().cast::<VBoxContainer>();
            let scene = load::<PackedScene>(format!("res://promote/{}.tscn", i));
            let node_button = scene.instantiate().unwrap();
            let mut button = node_button.clone().cast::<Button>();
            button.set_size(Vector2::new(200.0, 25.0));
            // let callable = Callable::from_fn("promote", |args: &[&Variant]| {
            //     let mut this = args[0].try_to_object::<PromoteVbox>().unwrap();
            //     this.promote();
            //     Variant::new()
            // });
            button.connect(
                "pressed".into(),
                Callable::from_object_method(base, "promote"),
            );
            // button.hide();
            self.node.add_child(node_button);
        }
    }
    #[func]
    pub fn promote(&mut self) {
        self.node
            .emit_signal("choose_piece".into(), &[self.index.unwrap().to_variant()]);
    }

    #[func]
    pub fn open(&mut self, index: i32) {
        for i in 0..self.node.get_child_count() {
            let child = self.node.get_child(i).unwrap();
            let mut button = child.cast::<Button>();
            button.show();
        }
        self.add_buttons();
        self.index = Some(index);
    }
}
