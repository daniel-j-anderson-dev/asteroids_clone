//! make a bullet struct that
//! - drawn as a small circle
//! - has a constant velocity
//! - Starts from the front of the player (a vertex)
//! - travels in the direction of the player (an angle)
//! - disappears after n frames

use crate::{polar_vec2, Draw, Kinematic, KinematicGetters, KinematicMutators, Player};
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
        Vec::new()
    }
    pub fn is_too_old(&self) -> bool {
        self.frames_left == 0
    }
    pub fn has_collided(&self) -> bool {
        self.has_collided
    }
    pub fn is_alive(&self) -> bool {
        !self.has_collided && self.frames_left > 0
    }
    pub fn set_collided(&mut self) {
        self.has_collided = true;
    }
    pub fn step(&mut self) {
        self.step_motion();
        self.keep_on_screen();
        self.frames_left = self.frames_left.saturating_sub(1);
    }
}
impl KinematicGetters for Bullet {
    fn kinematic(&self) -> &Kinematic {
        &self.kinematic
    }
}
impl KinematicMutators for Bullet {
    fn kinematic_mut(&mut self) -> &mut Kinematic {
        &mut self.kinematic
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
        let player_front = player.front_vertex();
        let velocity = polar_vec2(Self::SPEED + player.speed(), player.orientation());

        Bullet {
            kinematic: Kinematic::new(player_front, velocity, Vec2::ZERO),
            frames_left: Self::FRAMES_ALIVE,
            has_collided: false,
        }
    }
}
