//! Implement an asteroid struct that is drawn as a spinning hexagon with constant velocity
//! random position constructor!
//! - figure out where vertexes are relative to the origin
//! - rotate those vertexes according to the rotation matrix
//! - add those rotated vertexes to the position of the hexagon
//! - draw a line between each vertex

use crate::{
    polar_vec2, rock_texture, Bullet, Draw, Kinematic, KinematicGetters, Player, RotationMatrix, FRAC_SQRT3_2
};
use macroquad::{prelude::*, rand::gen_range};
use std::f32::consts::{FRAC_PI_2, TAU};

pub struct Asteroid {
    kinematic: Kinematic,
    size: f32,
    orientation: f32,
    rotation_speed: f32,
}
impl Asteroid {
    pub const UNIT_VERTICES: [Vec2; 6] = [
        vec2(1.0, 0.0),
        vec2(0.5, FRAC_SQRT3_2),
        vec2(-0.5, FRAC_SQRT3_2),
        vec2(-1.0, 0.0),
        vec2(-0.5, -FRAC_SQRT3_2),
        vec2(0.5, -FRAC_SQRT3_2),
    ];

    pub const MIN_SPEED: f32 = 1.0;
    pub const MAX_SPEED: f32 = Player::MAX_SPEED / 16.0;

    pub const MIN_ROTATION_SPEED: f32 = Player::ROTATION_DELTA / 4.0;
    pub const MAX_ROTATION_SPEED: f32 = Player::ROTATION_DELTA / 2.0;

    pub const MIN_SIZE: f32 = Player::SIZE;
    pub const MAX_SIZE: f32 = Player::SIZE * 2.0;
}
impl Asteroid {
    pub fn multiple_random(count: usize) -> Vec<Self> {
        return (0..count).map(|_| Asteroid::random()).collect();
    }
    pub fn random() -> Self {
        let size = gen_range(Self::MIN_SIZE, Self::MAX_SIZE);

        let position = vec2(
            gen_range(0.0, screen_width()),
            gen_range(0.0, screen_height()),
        );

        let speed = gen_range(Self::MIN_SPEED, Self::MAX_SPEED);
        let angle = gen_range(0.0, TAU);
        let velocity = polar_vec2(speed, angle);

        let orientation = gen_range(0.0, TAU);
        let rotation_speed = gen_range(Self::MIN_ROTATION_SPEED, Self::MAX_ROTATION_SPEED);

        return Self {
            kinematic: Kinematic::new(position, velocity, Vec2::ZERO),
            size,
            orientation,
            rotation_speed,
        };
    }
    pub fn size(&self) -> f32 {
        return self.size;
    }
    pub fn orientation(&self) -> f32 {
        return self.orientation;
    }
    pub fn is_too_small(&self) -> bool {
        return self.size < Self::MIN_SIZE;
    }
    pub fn vertices(&self) -> [Vec2; 6] {
        let rotation = self.orientation.rotation_matrix();
        let position = self.position();
        let scale = self.size;

        return Self::UNIT_VERTICES.map(|vertex| (rotation * (vertex * scale)) + position);
    }
    pub fn split(self, bullet_velocity: Vec2) -> (Self, Self) {
        let asteroid_velocity_1 = vec2(-bullet_velocity.y, bullet_velocity.x);
        let asteroid_velocity_2 = -asteroid_velocity_1;
        
        let asteroid_1 = Asteroid {
            kinematic: Kinematic::new(self.position(), asteroid_velocity_1, Vec2::ZERO,),
            size: self.size / 2.0,
            orientation: self.orientation,
            rotation_speed: self.rotation_speed * (2.0 / 3.0),
        };

        let asteroid_2 = Asteroid {
            kinematic: Kinematic::new(self.position(), asteroid_velocity_2, Vec2::ZERO,),
            size: self.size / 2.0,
            orientation: self.orientation,
            rotation_speed: self.rotation_speed * (2.0 / 3.0),
        };
        

       return (asteroid_1, asteroid_2);
    }
    /// The asteroid is "destroyed" when it is removed from the collection. The asteroid is dropped when .split is called
    pub fn destroy(self) {
        if self.size > self.size / 4.0 {

        }
    }
}
impl KinematicGetters for Asteroid {
    fn kinematic(&self) -> &Kinematic {
        return &self.kinematic;
    }
}
impl Asteroid {
    pub fn step(&mut self) {
        self.rotate();
        self.kinematic.cap_speed(Self::MAX_SPEED);
        self.kinematic.keep_on_screen();
        self.kinematic.step_motion();
    }
    pub fn rotate(&mut self) {
        self.orientation += self.rotation_speed;
    }
}
impl Draw for Asteroid {
    /// # Example
    /// <img src="https://i.imgur.com/sI2p3qU.png">
    fn draw(&self) {
        let [v1, v2, v3, v4, v5, v6] = self.vertices();
        draw_triangle(v1, v2, v3, WHITE);
        draw_triangle(v1, v3, v5, WHITE);
        draw_triangle(v1, v6, v5, WHITE);
        draw_triangle(v3, v4, v5, WHITE);

        let texture_scale = 1.4;
        let texture_offset = self.size * (texture_scale / 2.0);
        let texture_position = self.position() - texture_offset;
        draw_texture_ex(
            rock_texture(),
            texture_position.x,
            texture_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::splat(self.size * texture_scale)),
                source: None,
                rotation: self.orientation - FRAC_PI_2,
                flip_x: false,
                flip_y: true,
                pivot: None,
            },
        );
    }
}
