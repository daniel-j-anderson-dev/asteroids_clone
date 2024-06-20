//! Implement an asteroid struct that is drawn as a spinning hexagon with constant velocity
//! random position constructor!
//! - figure out where vertexes are relative to the origin
//! - rotate those vertexes according to the rotation matrix
//! - add those rotated vertexes to the position of the hexagon
//! - draw a line between each vertex

use crate::{
    polar_vec2, rock_texture, Bullet, Draw, Kinematic, KinematicGetters, Player, RotationMatrix,
    FRAC_SQRT3_2,
};
use macroquad::{prelude::*, rand::gen_range};
use std::f32::consts::{FRAC_PI_2, TAU};

pub struct Asteroid {
    kinematic: Kinematic,
    size: f32,
    orientation: f32,
    rotation_speed: f32,
    has_collided: bool,
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

    pub const MIN_SIZE: f32 = Player::SIZE / 2.0;
    pub const MAX_SIZE: f32 = Player::SIZE * 4.0;

    const CHILD_SIZE_FACTOR: f32 = 1.0 / 2.0;
    const CHILD_ROTATION_SPEED_FACTOR: f32 = 2.0 / 3.0;
}
impl Asteroid {
    pub fn many_random(count: usize) -> Vec<Self> {
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
            has_collided: false,
        };
    }
    /// Creates another [Asteroid] with
    /// - the same position as `self`
    /// - a given `velocity`
    /// - size scaled by [Self::CHILD_SIZE_FACTOR]
    /// - rotation speed scaled by [Self::CHILD_ROTATION_SPEED_FACTOR]
    /// - has **not** collided
    pub fn create_child(&self, velocity: Vec2) -> Self {
        return Self {
            kinematic: Kinematic::new(self.position(), velocity, Vec2::ZERO),
            size: self.size * Self::CHILD_SIZE_FACTOR,
            orientation: self.orientation,
            rotation_speed: self.rotation_speed * Self::CHILD_ROTATION_SPEED_FACTOR,
            has_collided: false,
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
    pub fn has_collided(&self) -> bool {
        return self.has_collided;
    }
    pub fn vertices(&self) -> [Vec2; 6] {
        let rotation = self.orientation.rotation_matrix();
        let position = self.position();
        let scale = self.size;

        return Self::UNIT_VERTICES.map(|vertex| (rotation * (vertex * scale)) + position);
    }
    /// Returns two [Asteroid]s at the same position as `self` with opposite velocities perpendicular to `bullet_velocity`
    pub fn split(&self, bullet_velocity: Vec2) -> [Self; 2] {
        // randomly generate a speed for the children
        let speed = gen_range(Self::MIN_SPEED, Self::MAX_SPEED);

        // create a unit vector perpendicular to the bullet's velocity.
        //   - rotate 90°: swap the x and y components, and multiply the x component by -1
        //   - ensure norm == 1 (aka unit vector): `normalize` returns a vector with the same direction but with size normalized to 1
        // SAFETY: bullet_velocity is constant non-zero so `normalize` wont panic on division-by-zero
        let direction = vec2(-bullet_velocity.y, bullet_velocity.x).normalize();

        let velocity = speed * direction;
        
        return [
            self.create_child(velocity),
            self.create_child(-velocity),
        ];
    }
}
impl KinematicGetters for Asteroid {
    fn kinematic(&self) -> &Kinematic {
        return &self.kinematic;
    }
}
impl Asteroid {
    pub fn destroy(&mut self) {
        self.has_collided = true;
    }
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

        const TEXTURE_SCALE: f32 = 1.4;
        let texture_offset = self.size * (TEXTURE_SCALE / 2.0);
        let texture_position = self.position() - texture_offset;
        draw_texture_ex(
            rock_texture(),
            texture_position.x,
            texture_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::splat(self.size * TEXTURE_SCALE)),
                source: None,
                rotation: self.orientation - FRAC_PI_2,
                flip_x: false,
                flip_y: true,
                pivot: None,
            },
        );
    }
}
