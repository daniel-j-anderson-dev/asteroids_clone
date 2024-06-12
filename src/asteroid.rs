//! Implement an asteroid struct that is drawn as a spinning hexagon with constant velocity
//! random position constructor!
//! - figure out where vertexes are relative to the origin
//! - rotate those vertexes according to the rotation matrix
//! - add those rotated vertexes to the position of the hexagon
//! - draw a line between each vertex

use crate::{Draw, Kinematic, KinematicGetters, Player, RotationMatrix, FRAC_SQRT3_2};
use macroquad::{prelude::*, rand::gen_range};
use std::f32::consts::TAU;

pub struct Asteroid {
    kinematic: Kinematic,
    size: f32,
    orientation: f32,
    rotation_speed: f32,
}
impl Asteroid {
    pub const UNIT_VERTICES: [Vec2; 6] = [
        vec2(1.0, 0.0),
        vec2(0.5, FRAC_SQRT3_2),
        vec2(-0.5, FRAC_SQRT3_2),
        vec2(-1.0, 0.0),
        vec2(-0.5, -FRAC_SQRT3_2),
        vec2(0.5, -FRAC_SQRT3_2),
    ];

    pub const MIN_SPEED: f32 = 1.0;
    pub const MAX_SPEED: f32 = Player::MAX_SPEED / 8.0;

    pub const MIN_ROTATION_SPEED: f32 = Player::ROTATION_DELTA / 4.0;
    pub const MAX_ROTATION_SPEED: f32 = Player::ROTATION_DELTA / 2.0;

    pub const MIN_SIZE: f32 = Player::SIZE / 2.0;
    pub const MAX_SIZE: f32 = Player::SIZE;
}
impl Asteroid {
    pub fn random() -> Self {
        let size = gen_range(Self::MIN_SIZE, Self::MAX_SIZE);

        let position = vec2(
            gen_range(0.0, screen_width()),
            gen_range(0.0, screen_height()),
        );

        let speed = gen_range(Self::MIN_SPEED, Self::MAX_SPEED);
        let forward = gen_range(0.0, TAU).rotation_matrix() * Player::FORWARD;
        let velocity = speed * forward;

        let orientation = gen_range(0.0, TAU);
        let rotation_speed = gen_range(Self::MIN_ROTATION_SPEED, Self::MAX_ROTATION_SPEED);

        return Self {
            kinematic: Kinematic::new(position, velocity, Vec2::ZERO),
            size,
            orientation,
            rotation_speed,
        };
    }
    pub fn orientation(&self) -> f32 {
        return self.orientation;
    }
    pub fn vertices(&self) -> [Vec2; 6] {
        let rotation = self.orientation.rotation_matrix();
        let position = self.kinematic.position();
        let scale = self.size;

        let vertices = Self::UNIT_VERTICES.map(|vertex| (rotation * (vertex * scale)) + position);

        return vertices;
    }
}
impl KinematicGetters for Asteroid {
    fn kinematic(&self) -> &Kinematic {
        return &self.kinematic;
    }
}
impl Asteroid {
    pub fn update(&mut self) {
        self.rotate();
        self.kinematic.keep_on_screen();
        self.kinematic.step_motion();
    }
    pub fn rotate(&mut self) {
        self.orientation += self.rotation_speed;
    }
}
impl Draw for Asteroid {
    /// # Example
    /// <img src="https://i.imgur.com/sI2p3qU.png">
    fn draw(&self) {
        let [v1, v2, v3, v4, v5, v6] = self.vertices();

        draw_triangle(v1, v2, v3, WHITE);
        draw_triangle(v1, v6, v5, WHITE);
        draw_triangle(v3, v4, v5, WHITE);
        draw_triangle(v1, v3, v5, WHITE);
    }
}
