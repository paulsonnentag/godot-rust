use godot::classes::INode;
use godot::global::godot_print;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct AutomergeRepo {
    base: Base<Node>,
}

#[godot_api]
impl INode for AutomergeRepo {
    fn init(base: Base<Node>) -> Self {
        godot_print!("create repo");
        return Self { base };
    }
}

#[godot_api]
impl AutomergeRepo {
    #[func]
    fn destroy() {
        godot_print!("destroy repo");
    }
}
