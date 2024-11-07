use crate::{
    duck_texture, kinematic::Kinematic, polar_vec2, screen_origin, Draw, KinematicGetters,
    KinematicMutators, RotationMatrix,
};
use macroquad::prelude::*;
use std::f32::consts::{FRAC_PI_2, TAU};

pub struct Player {
    kinematic: Kinematic,
    has_collided: bool,
    lives: usize,
    orientation: f32,
}
impl Player {
    pub const SIZE: f32 = 20.0;
    /// An angle in radians
    pub const ROTATION_DELTA: f32 = 0.1;
    pub const THRUST: f32 = Self::SIZE / 100.0;

    pub const MAX_SPEED: f32 = Self::SIZE * 2.0;

    pub const VERTICES: [Vec2; 3] = [
        vec2(0.0, Self::SIZE),
        vec2(-Self::SIZE / 2.5, Self::SIZE / -4.0),
        vec2(Self::SIZE / 2.5, Self::SIZE / -4.0),
    ];
}
impl Player {
    /// Create a stationary player at the screen's origin
    pub fn default() -> Player {
        Player {
            kinematic: Kinematic::new(screen_origin(), Vec2::ZERO, Vec2::ZERO),
            lives: 3,
            orientation: 0.0,
            has_collided: false,
        }
    }
    pub fn has_collided(&self) -> bool {
        self.has_collided
    }
    /// Getter for the player's orientation angle
    pub fn orientation(&self) -> f32 {
        self.orientation
    }
    /// Returns vertices of player by calculating default vertices rotated by `self.orientation` then translated by `self.position`
    pub fn vertices(&self) -> [Vec2; 3] {
        let rotation = self.orientation.rotation_matrix();
        let position = self.kinematic.position();

        let vertices = Self::VERTICES.map(|vertex| (rotation * vertex) + position);

        vertices
    }
    pub fn front_vertex(&self) -> Vec2 {
        let rotation = self.orientation.rotation_matrix();
        let position = self.kinematic.position();

        let front_vertex = rotation * Self::VERTICES[0] + position;

        front_vertex
    }
}
impl KinematicGetters for Player {
    fn kinematic(&self) -> &Kinematic {
        &self.kinematic
    }
}
impl KinematicMutators for Player {
    fn kinematic_mut(&mut self) -> &mut Kinematic {
        &mut self.kinematic
    }
}
impl Player {
    pub fn destroy(&mut self) {
        self.has_collided = true;
    }
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
            let thrust = polar_vec2(Self::THRUST, self.orientation);
            self.apply_acceleration(thrust)
        }
    }
    /// Move one time step further in the player simulation
    pub fn step(&mut self) {
        self.cap_speed(Self::MAX_SPEED);
        self.keep_on_screen();
        self.step_motion();
        self.step_friction();
    }
}
impl Draw for Player {
    fn draw(&self) {
        let position = self.position();
        let [v1, v2, v3] = self.vertices();
        draw_triangle(v1, v2, v3, WHITE);

        const TEXTURE_OFFSET: f32 = Player::SIZE / 2.0;
        let texture_position = position - TEXTURE_OFFSET;
        draw_texture_ex(
            duck_texture(),
            texture_position.x,
            texture_position.y,
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
        draw_circle(position.x, position.y, 2.5, RED);
    }
}
