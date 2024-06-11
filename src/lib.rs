pub mod asteroid;
pub mod bullet;
pub mod kinematic;
pub mod player;

pub use crate::{asteroid::*, bullet::*, kinematic::*, player::*};
use macroquad::prelude::*;

pub trait RotationMatrix {
    fn angle(&self) -> f32;
    fn rotation_matrix(&self) -> Mat2 {
        let angle = self.angle();
        let sin = angle.sin();
        let cos = angle.cos();
        return mat2(vec2(-sin, cos), vec2(cos, sin));
    }
}
impl RotationMatrix for f32 {
    fn angle(&self) -> f32 {
        return *self;
    }
}

pub fn screen_dimensions() -> Vec2 {
    return vec2(screen_width(), screen_height());
}
pub fn screen_origin() -> Vec2 {
    return screen_dimensions() / 2.0;
}
