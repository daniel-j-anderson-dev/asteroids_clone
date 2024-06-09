use std::f32::consts::PI;

use macroquad::prelude::*;

pub struct Player {
    pub kinematic: Kinematic,
    pub lives: usize,
    pub orientation: f32,
}
impl Player {
    const SIZE: f32 = 12.0;
    pub fn default() -> Player {
        return Player {
            kinematic: Kinematic {
                position: vec2(screen_width(), screen_height()) / 2.0,
                velocity: vec2(0.0, 0.0),
                acceleration: vec2(0.0, 0.0),
            },
            lives: 3,
            orientation: 0.0,
        };
    }
    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.orientation -= 0.1;
        }
        if is_key_down(KeyCode::Right) {
            self.orientation += 0.1;
        }
    }
    pub fn position(&self) -> Vec2 {
        return self.kinematic.position;
    }
    pub fn velocity(&self) -> Vec2 {
        return self.kinematic.velocity;
    }
    pub fn acceleration(&self) -> Vec2 {
        return self.kinematic.acceleration;
    }
   
}

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
