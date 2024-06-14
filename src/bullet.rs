//! make a bullet struct that
//! - is a small circle
//! - has a constant velocity
//! - Starts from the front of the player (a vertex)
//! - travels in the direction of the player (an angle)
//! - disappears after n frames

use crate::{polar_vec2, Kinematic};
use crate::{Draw, KinematicGetters, Player};
use macroquad::prelude::*;
pub struct Bullet {
    kinematic: Kinematic,
}
impl Bullet {
    const SIZE: f32 = Player::SIZE / 10.0;
    const SPEED: f32 = Player::MAX_SPEED / 4.0;

    pub fn new(player_front: Vec2, angle: f32) -> Self {
        let velocity = polar_vec2(Self::SPEED, angle);
        
        return Bullet {
            kinematic: Kinematic::new(player_front, velocity, Vec2::ZERO),
        }; 
    }
    pub fn step(&mut self) {
        self.kinematic.step_motion();
        self.kinematic.keep_on_screen();
    }
}
impl KinematicGetters for Bullet {
    fn kinematic(&self) -> &Kinematic {
        return &self.kinematic;
    }
}
impl Draw for Bullet {
    fn draw(&self) {
        let position = self.position();
        draw_circle(position.x, position.y, Self::SIZE, WHITE);
    }
}
impl From<&Player> for Bullet {
    fn from(player: &Player) -> Self {
        return Bullet::new(player.vertices()[0], player.orientation());
    }
}
