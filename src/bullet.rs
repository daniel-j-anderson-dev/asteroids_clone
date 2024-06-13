//! make a bullet struct that
//! - is a small circle
//! - has a constant velocity
//! - Starts from the front of the player
//! - travels in the direction of the player
//! - disappears after n frames

use macroquad::prelude::*;
use crate::{Draw, Player, KinematicGetters};
use crate::Kinematic;
pub struct Bullet {
    kinematic: Kinematic,
    size: f32,
}
impl Bullet {
    fn default() -> Self {
        let player = Player::default();
        return Bullet {
            kinematic: Kinematic {
                position: vec2(0.0, 0.0),
                velocity: vec2(0.0, 0.0),
                acceleration: vec2(0.0, 0.0),
            },
            size: 5.0,
        }
    }
}
impl KinematicGetters for Bullet {
    fn kinematic(&self) -> &Kinematic {
        return &self.kinematic;
    }
}
impl Draw for Bullet {
    fn draw(&self) {
        draw_circle(x, y, 1.5, WHITE)
    }
}