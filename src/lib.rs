pub mod assets;
pub mod asteroid;
pub mod bullet;
pub mod kinematic;
pub mod player;

pub use crate::{assets::*, asteroid::*, bullet::*, kinematic::*, player::*};
use macroquad::prelude::*;

pub const FRAC_SQRT3_2: f32 = 0.86602540378443864676372317075294;
pub const SCREEN_WIDTH: usize = 800;
pub const SCREEN_HEIGHT: usize = 800;

pub fn is_point_in_polygon(point: Vec2, polygon_vertices: &[Vec2]) -> bool {
    let mut is_inside_polygon = false;

    let vertex_count = polygon_vertices.len();

    let mut current_vertex_index = 0;
    let mut next_vertex_index = vertex_count - 1;
    while current_vertex_index < vertex_count {
        let current_vertex = polygon_vertices[current_vertex_index];
        let next_vertex = polygon_vertices[next_vertex_index];

        let is_vertically_between_current_and_next =
            (current_vertex.y > point.y) != (next_vertex.y > point.y);

        if is_vertically_between_current_and_next {
            let difference = next_vertex - current_vertex;
            let x_intersection =
                ((difference.x * (point.y - current_vertex.y)) / difference.y) + current_vertex.x;
            let is_left_of_intersection = point.x < x_intersection;

            if is_left_of_intersection {
                is_inside_polygon = !is_inside_polygon;
            }
        }

        next_vertex_index = current_vertex_index;
        current_vertex_index += 1;
    }

    is_inside_polygon
}

/// returns a rectangular Vec2 with the given `norm` and `angle`. `angle` is relative to the positive x axis
pub fn polar_vec2(norm: f32, angle: f32) -> Vec2 {
    vec2(angle.cos(), angle.sin()) * norm
}

pub trait RotationMatrix {
    fn rotation_matrix(&self) -> Mat2;
}
impl RotationMatrix for f32 {
    fn rotation_matrix(&self) -> Mat2 {
        let sin = self.sin();
        let cos = self.cos();
        mat2(
            vec2(-sin, cos), //
            vec2(cos, sin),
        )
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
    vec2(screen_width(), screen_height())
}

pub fn screen_origin() -> Vec2 {
    screen_dimensions() / 2.0
}
