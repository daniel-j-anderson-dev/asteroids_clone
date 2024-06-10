pub mod asteroid;
pub mod bullet;
pub mod kinematic;
pub mod player;

pub use crate::{asteroid::*, bullet::*, kinematic::*, player::*};
use macroquad::prelude::*;

pub fn screen_dimensions() -> Vec2 {
    return vec2(screen_width(), screen_height());
}
pub fn screen_origin() -> Vec2 {
    return screen_dimensions() / 2.0;
}
