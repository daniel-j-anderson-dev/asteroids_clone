use crate::{screen_origin, kinematic::Kinematic, RotationMatrix};
use std::f32::consts::TAU;
use macroquad::prelude::*;

pub struct Player {
    pub kinematic: Kinematic,
    pub lives: usize,
    /// An angle in radians from the positive y-axis
    pub orientation: f32,
}
impl Player {
    pub const SIZE: f32 = 20.0;
    /// An angle in radians
    pub const ROTATION_DELTA: f32 = 0.1;
    pub const THRUSTER_ACCELERATION: f32 = 0.2;

    const MAX_SPEED: f32 = Self::SIZE * 2.0;
    const MAX_SPEED_SQUARED: f32 = Self::MAX_SPEED * Self::MAX_SPEED;

    const FRONT_OFFSET: Vec2 = vec2(0.0, Self::SIZE);
    const LEFT_OFFSET: Vec2 = vec2(-Self::SIZE / 2.5, Self::SIZE / -4.0);
    const RIGHT_OFFSET: Vec2 = vec2(Self::SIZE / 2.5, Self::SIZE / -4.0);
}
impl Player {
    pub fn default() -> Player {
        return Player {
            kinematic: Kinematic {
                position: screen_origin(),
                velocity: Vec2::ZERO,
                acceleration: Vec2::ZERO,
            },
            lives: 3,
            orientation: 0.0,
        };
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
    pub fn orientation(&self) -> f32 {
        return self.orientation;
    }
    pub fn vertices(&self) -> (Vec2, Vec2, Vec2) {
        let rotation = self.orientation.rotation_matrix();

        let front = (rotation * Self::FRONT_OFFSET) + self.kinematic.position;
        let left = (rotation * Self::LEFT_OFFSET) + self.kinematic.position;
        let right = (rotation * Self::RIGHT_OFFSET) + self.kinematic.position;

        return (front, left, right);
    }
}
impl Player {
    pub fn keep_on_screen(&mut self) {
        // take a peek forward in time!
        let next_position = self.kinematic.position + self.kinematic.velocity;

        // next frame player will travel off the left side
        if next_position.x < 0.0 {
            // so lets teleport them to the right side
            self.kinematic.position.x = screen_width();
        }
        if next_position.x > screen_width() {
            self.kinematic.position.x = 0.0;
        }
        if next_position.y < 0.0 {
            self.kinematic.position.y = screen_height();
        }
        if next_position.y > screen_height() {
            self.kinematic.position.y = 0.0;
        }
    }
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
            let thrust = forward * Self::THRUSTER_ACCELERATION;

            self.kinematic.acceleration += (thrust - self.kinematic.acceleration) * 0.1;
        }

        if self.kinematic.velocity.length_squared() > Self::MAX_SPEED_SQUARED {
            self.kinematic.velocity = self.kinematic.velocity.normalize() * Self::MAX_SPEED;
        }

        self.kinematic.step_friction();
        self.keep_on_screen();
        self.kinematic.step_motion();
    }
    pub fn draw(&self) {
        let (v1, v2, v3) = self.vertices();
        draw_triangle(v1, v2, v3, WHITE);
        draw_circle(
            self.kinematic.position.x,
            self.kinematic.position.y,
            2.5,
            RED,
        );
    }
}
