use macroquad::prelude::*;

pub struct Kinematic {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}
impl Kinematic {
    pub fn step(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
    pub fn apply_friction(&mut self) {
        // apply friction (using linear interpolation aka lerp)
        self.acceleration = self.acceleration.lerp(Vec2::ZERO, 0.02);
        self.velocity = self.velocity.lerp(Vec2::ZERO, 0.02);
    }
}