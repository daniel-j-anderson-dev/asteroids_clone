use crate::{
    duck_texture, kinematic::Kinematic, polar_vec2, screen_origin, Draw, KinematicGetters,
    RotationMatrix,
};
use macroquad::prelude::*;
use std::f32::consts::{FRAC_PI_2, TAU};

pub struct Player {
    kinematic: Kinematic,
    lives: usize,
    orientation: f32,
    texture: Texture2D,
}
impl Player {
    pub const SIZE: f32 = 20.0;
    /// An angle in radians
    pub const ROTATION_DELTA: f32 = 0.1;
    pub const THRUST_ACCELERATION: f32 = 0.2;

    pub const MAX_SPEED: f32 = Self::SIZE * 2.0;

    /// Normalized front vertex before rotation and translation
    pub const FORWARD: Vec2 = Vec2::Y;

    pub const VERTICES: [Vec2; 3] = [
        vec2(0.0, Self::SIZE),
        vec2(-Self::SIZE / 2.5, Self::SIZE / -4.0),
        vec2(Self::SIZE / 2.5, Self::SIZE / -4.0),
    ];
}
impl Player {
    /// Create a stationary player at the screen's origin
    pub fn default() -> Player {
        return Player {
            kinematic: Kinematic::new(screen_origin(), Vec2::ZERO, Vec2::ZERO),
            lives: 3,
            orientation: 0.0,
            texture: duck_texture(),
        };
    }
    /// Getter for the player's orientation angle
    pub fn orientation(&self) -> f32 {
        return self.orientation;
    }
    /// Returns vertices of player by calculating default vertices rotated by `self.orientation` then translated by `self.position`
    pub fn vertices(&self) -> [Vec2; 3] {
        let rotation = self.orientation.rotation_matrix();
        let position = self.kinematic.position();

        let vertices = Self::VERTICES.map(|vertex| (rotation * vertex) + position);

        return vertices;
    }
}
impl KinematicGetters for Player {
    fn kinematic(&self) -> &Kinematic {
        return &self.kinematic;
    }
}
impl Player {
    /// - rotate
    ///   - Left (counter-clockwise)
    ///   - Right (clockwise)
    ///   - -2pi <= 'self.orientation' <= 2pi
    /// - accelerate player forward
    ///   - Up
    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.orientation -= Self::ROTATION_DELTA;
        }
        if is_key_down(KeyCode::Right) {
            self.orientation += Self::ROTATION_DELTA;
        }
        self.orientation %= TAU;

        if is_key_down(KeyCode::Up) {
            let thrust = polar_vec2(Self::THRUST_ACCELERATION, self.orientation);
            self.kinematic.lerp_acceleration(thrust)
        }
    }
    /// Move one time step further in the player simulation
    pub fn step(&mut self) {
        self.handle_input();
        self.kinematic.cap_speed(Self::MAX_SPEED);
        self.kinematic.keep_on_screen();
        self.kinematic.step_motion();
        self.kinematic.step_friction();
    }
}
impl Draw for Player {
    fn draw(&self) {
        let pos = self.position();

        let [v1, v2, v3] = self.vertices();
        draw_triangle(v1, v2, v3, WHITE);
        draw_circle(pos.x, pos.y, 2.5, RED);

        draw_texture_ex(
            &self.texture,
            pos.x - (Self::SIZE / 2.0),
            pos.y - (Self::SIZE / 2.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::splat(Self::SIZE)),
                source: None,
                rotation: self.orientation - FRAC_PI_2,
                flip_x: false,
                flip_y: true,
                pivot: None,
            },
        );
    }
}
