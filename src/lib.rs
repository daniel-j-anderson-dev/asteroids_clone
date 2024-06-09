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
}
