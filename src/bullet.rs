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
    frames_left: usize,
    has_collided: bool,
}
impl Bullet {
    pub const SIZE: f32 = Player::SIZE / 10.0;
    pub const SPEED: f32 = Player::MAX_SPEED / 4.0;

    pub const FRAMES_ALIVE: usize = 60;

    pub fn many_new() -> Vec<Self> {
        return Vec::new();
    }
    pub fn new(player_front: Vec2, angle: f32) -> Self {
        let velocity = polar_vec2(Self::SPEED, angle);

        return Bullet {
            kinematic: Kinematic::new(player_front, velocity, Vec2::ZERO),
            frames_left: Self::FRAMES_ALIVE,
            has_collided: false,
        };
    }
    pub fn is_too_old(&self) -> bool {
        return self.frames_left == 0;
    }
    pub fn has_collided(&self) -> bool {
        return self.has_collided;
    }
    pub fn destroy(&mut self) {
        self.has_collided = true;
    }
    pub fn step(&mut self) {
        self.kinematic.step_motion();
        self.kinematic.keep_on_screen();
        self.frames_left = self.frames_left.saturating_sub(1);
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
