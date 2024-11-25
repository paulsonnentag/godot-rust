use godot::classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, PhysicsBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct Automerge {
    name: String,
}

#[godot_api]
impl Automerge {
    #[func]
    fn test() -> GString {
        "test 123".into()
    }
}
