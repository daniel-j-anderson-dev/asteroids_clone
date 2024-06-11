use crate::{kinematic::Kinematic, screen_origin, RotationMatrix};
use macroquad::prelude::*;
use std::f32::consts::TAU;

pub struct Player {
    pub kinematic: Kinematic,
    pub lives: usize,
    /// An angle in radians clockwise from the positive x-axis
    pub orientation: f32,
}
impl Player {
    pub const SIZE: f32 = 20.0;
    /// An angle in radians
    pub const ROTATION_DELTA: f32 = 0.1;
    pub const THRUST_ACCELERATION: f32 = 0.2;

    pub const MAX_SPEED: f32 = Self::SIZE * 2.0;

    pub const VERTICES: [Vec2; 3] = [
        vec2(0.0, Self::SIZE),
        vec2(-Self::SIZE / 2.5, Self::SIZE / -4.0),
        vec2(Self::SIZE / 2.5, Self::SIZE / -4.0),
    ];
}
impl Player {
    pub fn default() -> Player {
        return Player {
            kinematic: Kinematic::new(screen_origin(), Vec2::ZERO, Vec2::ZERO),
            lives: 3,
            orientation: 0.0,
        };
    }
    pub fn position(&self) -> Vec2 {
        return self.kinematic.position();
    }
    pub fn velocity(&self) -> Vec2 {
        return self.kinematic.velocity();
    }
    pub fn acceleration(&self) -> Vec2 {
        return self.kinematic.acceleration();
    }
    pub fn orientation(&self) -> f32 {
        return self.orientation;
    }
    pub fn vertices(&self) -> [Vec2; 3] {
        let rotation = self.orientation.rotation_matrix();
        let position = self.kinematic.position();

        let vertices = Self::VERTICES.map(|vertex| (rotation * vertex) + position);

        return vertices;
    }
}
impl Player {
    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.orientation = (self.orientation - Self::ROTATION_DELTA) % TAU;
        }
        if is_key_down(KeyCode::Right) {
            self.orientation = (self.orientation + Self::ROTATION_DELTA) % TAU;
        }
        if is_key_down(KeyCode::Up) {
            let rotation = self.orientation.rotation_matrix();
            let forward = rotation * Vec2::Y;
            let thrust = forward * Self::THRUST_ACCELERATION;
            self.kinematic.lerp_acceleration(thrust)
        }

        self.kinematic.cap_speed(Self::MAX_SPEED);
        self.kinematic.step_friction();
        self.kinematic.keep_on_screen();
        self.kinematic.step_motion();
    }
    pub fn draw(&self) {
        let [v1, v2, v3] = self.vertices();
        draw_triangle(v1, v2, v3, WHITE);
        draw_circle(
            self.kinematic.position().x,
            self.kinematic.position().y,
            2.5,
            RED,
        );
    }
}
