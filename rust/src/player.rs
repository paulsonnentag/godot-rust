use godot::classes::Sprite2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    angular_speed: f64,

    base: Base<Sprite2D>,
}

use godot::classes::ISprite2D;

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            angular_speed: std::f64::consts::PI * 2.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }
}