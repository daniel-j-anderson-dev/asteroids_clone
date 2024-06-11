//! Implement an asteroid struct that is drawn as a spinning hexagon with constant velocity
//! random position constructor!
//! - figure out where vertexes are relative to the origin
//! - rotate those vertexes according to the rotation matrix
//! - add those rotated vertexes to the position of the hexagon
//! - draw a line between each vertex

use crate::{Kinematic, Player, RotationMatrix};
use macroquad::{prelude::*, rand::gen_range};
use std::f32::consts::TAU;

pub struct Asteroid {
    kinematic: Kinematic,
    orientation: f32,
}
impl Asteroid {
    const SIZE: f32 = 20.0;

    const TRIANGLE_SIZE: f32 = Self::SIZE / 2.0;
    const HALF_TRIANGLE_SIZE: f32 = Self::TRIANGLE_SIZE / 2.0;

    const VERTEX1: Vec2 = vec2(-Self::HALF_TRIANGLE_SIZE, Self::TRIANGLE_SIZE);
    const VERTEX2: Vec2 = vec2(Self::HALF_TRIANGLE_SIZE, Self::TRIANGLE_SIZE);
    const VERTEX3: Vec2 = vec2(Self::TRIANGLE_SIZE, 0.0);
    const VERTEX4: Vec2 = vec2(Self::HALF_TRIANGLE_SIZE, -Self::TRIANGLE_SIZE);
    const VERTEX5: Vec2 = vec2(-Self::HALF_TRIANGLE_SIZE, -Self::TRIANGLE_SIZE);
    const VERTEX6: Vec2 = vec2(-Self::HALF_TRIANGLE_SIZE, 0.0);

    pub const MIN_SPEED: f32 = 1.0;
    pub const MAX_SPEED: f32 = Player::MAX_SPEED;
}
impl Asteroid {
    pub fn random() -> Self {
        let position = vec2(
            gen_range(0.0, screen_width()),
            gen_range(0.0, screen_height()),
        );

        let speed = gen_range(Self::MIN_SPEED, Self::MAX_SPEED);
        let forward = gen_range(0.0, TAU).rotation_matrix() * Vec2::Y;
        let velocity = speed * forward;

        let orientation = gen_range(0.0, TAU);

        return Self {
            kinematic: Kinematic::new(position, velocity, Vec2::ZERO),
            orientation,
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
}
impl Asteroid {
    pub fn vertices(&self) -> [Vec2; 6] {
        let rotation = self.orientation.rotation_matrix();
        let position = self.kinematic.position();

        let vertices = [
            (rotation * Self::VERTEX1) + position,
            (rotation * Self::VERTEX2) + position,
            (rotation * Self::VERTEX3) + position,
            (rotation * Self::VERTEX4) + position,
            (rotation * Self::VERTEX5) + position,
            (rotation * Self::VERTEX6) + position,
        ];

        return vertices;
    }
    /// # Example
    /// <img src="https://i.imgur.com/sI2p3qU.png">
    pub fn draw(&self) {
        let [v1, v2, v3, v4, v5, v6] = self.vertices();

        draw_triangle(v1, v2, v3, WHITE);
        draw_triangle(v1, v6, v5, WHITE);
        draw_triangle(v3, v4, v5, WHITE);
        draw_triangle(v1, v3, v5, WHITE);
    }
}
