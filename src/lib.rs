pub mod assets;
pub mod asteroid;
pub mod bullet;
pub mod kinematic;
pub mod player;

pub use crate::{assets::*, asteroid::*, bullet::*, kinematic::*, player::*};
use macroquad::prelude::*;

const FRAC_SQRT3_2: f32 = 0.86602540378443864676372317075294;

/// This function checks every asteroid-bullet combination and asteroid-player combinations for collisions.
/// ## Returns
/// - A list of asteroid-bullet collision indexes
/// - A bool that represents if there was a asteroid-player collision
pub fn calculate_collisions(asteroids: &[Asteroid], bullets: &[Bullet], player: &Player) -> (Vec<(usize, usize)>, bool) {
    let mut collision_indexes = Vec::new();
    let mut player_collision = false;

    for i in 0..asteroids.len() {
        for j in 0..bullets.len() {
            // if the bullet is inside the asteroid
            if is_point_in_hexagon(
                bullets[j].position(),
                asteroids[i].position(),
                asteroids[i].orientation(),
                asteroids[i].size(),
            ) {
                // store the indexes to be processed later
                collision_indexes.push((i, j));
            }
        }
        // TODO: check for collision with player and asteroid and print on all collisions
    }

    return (collision_indexes, player_collision);
}

pub fn is_point_in_hexagon(
    point: Vec2,
    hexagon_center: Vec2,
    hexagon_orientation: f32,
    hexagon_size: f32,
) -> bool {
    // Translate point to the hexagon's origin
    let translated = vec2(point.x - hexagon_center.x, point.y - hexagon_center.y);

    // Rotate the point by the negative of the hexagon’s angle
    let cos = hexagon_orientation.cos();
    let sin = hexagon_orientation.sin();
    let rotated = vec2(
        translated.x * cos + translated.y * sin,
        -translated.x * sin + translated.y * cos,
    );

    let within_vertical_bounds = rotated.x.abs() <= hexagon_size;
    let within_horizontal_bounds = rotated.y.abs() <= FRAC_SQRT3_2 * hexagon_size;
    let within_diagonal_bounds = (rotated.y - FRAC_SQRT3_2 * rotated.y).abs()
        <= FRAC_SQRT3_2 * hexagon_size
        && (rotated.x + FRAC_SQRT3_2 * rotated.y).abs() <= FRAC_SQRT3_2 * hexagon_size;

    return within_vertical_bounds && within_horizontal_bounds && within_diagonal_bounds;
}

/// returns a rectangular Vec2 with the given `norm` and `angle`. `angle` is relative to the positive x axis
pub fn polar_vec2(norm: f32, angle: f32) -> Vec2 {
    return vec2(angle.cos(), angle.sin()) * norm;
}

pub trait RotationMatrix {
    fn rotation_matrix(&self) -> Mat2;
}
impl RotationMatrix for f32 {
    fn rotation_matrix(&self) -> Mat2 {
        let sin = self.sin();
        let cos = self.cos();
        return mat2(
            vec2(-sin, cos), //
            vec2(cos, sin),
        );
    }
}

pub trait Draw {
    fn draw(&self);
}

pub fn initialize_rng() {
    let current_time = macroquad::miniquad::date::now() as u64;
    rand::srand(current_time);
}

pub fn screen_dimensions() -> Vec2 {
    return vec2(screen_width(), screen_height());
}

pub fn screen_origin() -> Vec2 {
    return screen_dimensions() / 2.0;
}
